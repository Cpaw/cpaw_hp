extern crate iron;
extern crate iron_sessionstorage;
extern crate params;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use std::collections::HashMap;
use hbs::{Template};
use self::iron_sessionstorage::traits::*;
use self::iron_sessionstorage::SessionStorage;
use self::iron_sessionstorage::backends::SignedCookieBackend;
use params::{Params, Value};
use router::url_for;


extern crate crypto;
extern crate rusqlite;
extern crate serde;
use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;
use user::User;


// セッションに保存される情報
struct UserSession {
    id: String
}

impl iron_sessionstorage::Value for UserSession {
    fn get_key() -> &'static str { "logged_in_user" }
    fn into_raw(self) -> String { self.id }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(UserSession { id: value })
        }
    }
}

pub fn logged_in(req: &mut Request) -> bool {
    // Result<UserSession>を取り出してNoneならfalse戻して関数終了
    let opt_user_session = match req.session().get::<UserSession>() {
        Ok(opt_us) => { opt_us },
        Err(e) => { return false; }
    };

    let user_id_str =  match opt_user_session {
        Some(us) => { us.id },
        None => { return false; }
    };

    let user_id = match user_id_str.parse::<i32>() {
        Ok(id) => { id },
        Err(e) => { return false }
    };

    User::find(user_id).is_some()
}

pub fn login(req: &mut Request) -> IronResult<Response> {

    
    // セッションにUserSessionがあるなら
    // println!("{}", req.session().get::<UserSession>().unwrap().unwrap().id);
    if logged_in(req) {
    // if try!(req.session().get::<UserSession>()).is_some() {
        // Already logged in
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "top")))));
    }

    if req.method.to_string() == "GET" {
        let mut resp = Response::new();
        let mut data = HashMap::new();
        data.insert("", "");
        resp.set_mut(Template::new("login", data)).set_mut(status::Ok);
        return Ok(resp);
    }
    
    // チェックとかしてない
    let map = req.get_ref::<Params>().unwrap().clone();

    let email = match map.find(&["email"]) {
        Some(&Value::String(ref value))  => { value },
        _ => { "fail" }
    };
    // let email = {
    //     println!("Enter email block");
    //     let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
    //     println!("{:?}", formdata);
    //     iexpect!(formdata.get("email"))[0].to_owned()
    // };
    println!("[+] Email {}", email);

    let password = match map.find(&["password"]) {
        Some(&Value::String(ref value))  => { value },
        _ => { "fail" }
    };
    // let password = {
    //     let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
    //     iexpect!(formdata.get("password"))[0].to_owned()
    // };
    println!("[ ] password {}", password);

    let mut sha = Sha512::new();
    sha.input_str(&password);
    let password_hash = sha.result_str();

    // 見つからないとOption None
    let user: User = match User::find_by("email", &email) {
        Some(user) => { user },
        None => { return Ok(Response::with((status::Ok, "User not found"))); }
    };

    if user.password != password_hash {
        println!("Invalid password");
        // passwordが一致しなかったら適当にリダイレクト
        return Ok(Response::with((status::Ok, "Login Failed: invalid password")));
    }

    println!("[+] Save session");
    // セッションにユーザー名を保存
    try!( req.session().set(UserSession { id: user.id.to_string() }) );

    // '/'にリダイレクト
    Ok(Response::with((status::Found, "Login Success")))
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "greet")))))
}

pub fn greet(req: &mut Request) -> IronResult<Response> {
    let login = iexpect!(
        req.session().get::<UserSession>().ok().and_then(|x| x),
        (
            status::Unauthorized,
            "text/html".parse::<iron::mime::Mime>().unwrap(),
            "<a href=/login>Log in</a>"
        )
    );

    Ok(Response::with((
        status::Ok,
        "text/html".parse::<iron::mime::Mime>().unwrap(),
        format!("Hello, {}! <br/>\n\
        <form method=post action=/logout>\n\
        <input type=submit value='Log out' />\n\
        </form>", login.id)
    )))
}

// fn main() {
//     let router = router!(
//         greet: get "/" => greet,
//         login: get "/login" => login,
//         login_post: post "/login" => login_post,
//         logout: post "/logout" => logout,
//     );
//
//     let my_secret = b"verysecret".to_vec();
//     let mut ch = Chain::new(router);
//     ch.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
//     let _res = Iron::new(ch).http("localhost:8080");
//     println!("Listening on 8080.");
// }
