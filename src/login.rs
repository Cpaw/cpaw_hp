extern crate iron;
extern crate iron_sessionstorage;
extern crate params;
extern crate crypto;
extern crate serde_json;

use iron::prelude::*;
use iron::{status,headers};
use iron::modifiers::{Redirect,Header};
use std::collections::HashMap;
use hbs::Template;
use self::iron_sessionstorage::traits::*;
use params::{Params, Value};
use router::url_for;
use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;
use handlebars::Handlebars;
use user::User;
use routing::template_html;
use rustc_serialize::json;

use routing::response_html;
use routing::response_json;

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
    println!("[+] Called current_user_json");

    let session = if is_logged_in(req) {
        current_user(req).unwrap().username
    } else {
        "guest".to_string()
    };

    Ok(response_json(json!({"session": session})))
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
    println!("[+] Called login_get");

    if is_logged_in(req) {
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
    }

    let filename = "login.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css", "register.css"],
    });
    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn login_post(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called login_post");

    if is_logged_in(req) {
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));
    }

    let map = req.get_ref::<Params>().unwrap().clone();

    let username = match take_param!(map, "username", Value::String) {
        Some(user) => user,
        None => { return Ok(response_json(json!({"result": false}))) }
    };
    println!("[+] Username {}", username);

    let password = match take_param!(map, "password", Value::String) {
        Some(user) => user,
        None => { return Ok(response_json(json!({"result": false}))) }
    };
    println!("[+] Password {}", password);

    let mut sha = Sha512::new();
    sha.input_str(&password);
    let password_hash = sha.result_str();

    // 見つからないとOption None
    let user: User = match User::find_by("username", username) {
        Some(user) => { user },
        None => { return Ok(response_json(json!({"result": false}))) }
    };

    if user.password != password_hash {
        println!("Invalid password");
        return Ok(response_json(json!({"result": false})))
    }

    println!("[ ] Save session");
    // セッションにユーザー名を保存
    try!( req.session().set(UserSession { id: user.id.to_string() }) );

    Ok(response_json(json!({"result": true})))
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called logout");
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(url_for!(req, "top")))))
}
