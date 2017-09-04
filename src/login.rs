extern crate iron;
extern crate iron_sessionstorage;
extern crate params;
extern crate crypto;
extern crate serde_json;

use iron::prelude::*;
use iron::{status,headers};
use iron::modifiers::{Redirect,Header};
use std::collections::HashMap;
use hbs::{Template};
use self::iron_sessionstorage::traits::*;
use params::{Params, Value};
use router::url_for;
use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;
use handlebars::Handlebars;
use user::User;
use routing::template_html;
use rustc_serialize::json;

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

pub fn current_user_json(req: &mut Request) -> IronResult<Response> {
    
    let mut h = HashMap::new();
    if is_logged_in(req) {
        println!("[+] Called current_user_json");
        let crrnt_user = current_user(req).unwrap();
        h.insert("session", crrnt_user.username);
    } else {
        h.insert("session", "guest".to_string());
    }
    return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));    
}

pub fn current_user(req: &mut Request) -> Result<User, String> {
    let opt_user_session = match req.session().get::<UserSession>() {
        Ok(opt_us) => { opt_us },
        Err(_) => { return Err("Session can not get".to_string()); }
    };

    let user_id_str =  match opt_user_session {
        Some(us) => { us.id },
        None => { return Err("UserSession not found in cookie".to_string()); }
    };

    let user_id = match user_id_str.parse::<i32>() {
        Ok(id) => { id },
        Err(_) => { return Err("Failed to convert user_id to i32 from String".to_string()); }
    };

    let rslt_user = match User::find(user_id) {
        Some(user) => { Ok(user) },
        None => { return Err("User not found".to_string()); }
    };
    rslt_user
}

pub fn is_logged_in(req: &mut Request) -> bool {
    current_user(req).is_ok()
}

pub fn login_get(req: &mut Request) -> IronResult<Response> {    

    if is_logged_in(req) {
        // if try!(req.session().get::<UserSession>()).is_some() {
        // Already logged in
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
    }
    
    let filename = "login.hbs";
    let mut handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css"],
    });
    let rslt_html = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    let mut resp = Response::new();
    resp
        .set_mut(rslt_html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));
    
    Ok(resp)
        
}

pub fn login_post(req: &mut Request) -> IronResult<Response> {
    
    if is_logged_in(req) {
        // if try!(req.session().get::<UserSession>()).is_some() {
        // Already logged in
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
    }
    
    // セッションにUserSessionがあるなら
    // println!("{}", req.session().get::<UserSession>().unwrap().unwrap().id);
    
    let map = req.get_ref::<Params>().unwrap().clone();

    let username:&String = match map.find(&["username"]) {
        Some(&Value::String(ref value))  => { value },
        _ => {
            let mut h = HashMap::new();            
            h.insert("result", false);
            return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));
        }
    };
    
    // let email = {
    //     println!("Enter email block");
    //     let formdata = iexpect!(req.get_ref::<UrlEncodedBody>().ok());
    //     println!("{:?}", formdata);
    //     iexpect!(formdata.get("email"))[0].to_owned()
    // };
    println!("[+] Username {}", username);
    
    let password:&String = match map.find(&["password"]) {
        Some(&Value::String(ref value))  => { value },
        _ => {
            let mut h = HashMap::new();
            h.insert("result", false);
            return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));
        }
    };
    println!("[+] Password {}", password);

    let mut sha = Sha512::new();
    sha.input_str(&password);
    let password_hash = sha.result_str();

    // 見つからないとOption None
    let user: User = match User::find_by("username", username) {
        Some(user) => { user },
        None => {
            let mut h = HashMap::new();
            h.insert("result", false);
            return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));
        }
    };

    if user.password != password_hash {
        println!("Invalid password");
        // passwordが一致しなかったら適当にリダイレクト
        let mut h = HashMap::new();
        h.insert("result", false);
        return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));        
    }

    println!("[ ] Save session");
    // セッションにユーザー名を保存
    try!( req.session().set(UserSession { id: user.id.to_string() }) );

    // '/'にリダイレクト
    let mut h = HashMap::new();
    h.insert("result", true);
    return Ok(Response::with((status::Ok, json::encode(&h).unwrap())));        
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "top")))))
}
