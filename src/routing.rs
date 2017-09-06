extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate crypto;
extern crate serde_json;
extern crate iron_sessionstorage;

use std::env;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;
use iron::prelude::*;
use iron::{headers, status};
use iron::modifiers::{Redirect,Header};
use self::iron_sessionstorage::traits::*;
use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;
use router::url_for;
use router::Router;
use handlebars::Handlebars;
use hbs::{Template};
use params::{Params, Value};
use rustc_serialize::json;
use user::User;
use rand::{thread_rng, Rng};
use login;
use login::UserSession;



// take_param!(map, "key", Value::String) でOption<String>な値を取り出す
macro_rules! take_param {
    ($map:expr, $key:expr, $type:path) => {
        match $map.find(&[$key]) {
            Some(&$type(ref value)) => Some(value),
            _ => None,
        }
    }
}

pub fn response_json(json: serde_json::Value) -> Response {
    Response::new()
        .set(status::Ok)
        .set(Header(headers::ContentType::json()))
        .set(json.to_string())
}

/*
pub fn blog(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called blog");

    let mut resp = Response::new();
    let mut data = HashMap::new();
    
    let conn = Connection::open("./sqlite3.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, title, author, body FROM blog").unwrap();
    let user_iter = stmt.query_map(&[], |row| {
         Blog {
             id: row.get(0),
             title: row.get(1),
             author: row.get(2),
             body: row.get(3),
         }
    }).unwrap();
    
    let mut v = Vec::new();
    for user in user_iter {
        
        let mut hash: HashMap<String, String> = HashMap::new();
        let userUnwrapped = user.unwrap();
        
        hash.insert(String::from("id"), userUnwrapped.id.to_string());
        hash.insert(String::from("title"), userUnwrapped.title);
        hash.insert(String::from("body"), userUnwrapped.body);
        hash.insert(String::from("author"), userUnwrapped.author);
        v.push(hash);
        
    }
    
    data.insert(String::from("blog"), v);
    resp.set_mut(Template::new("blog", data)).set_mut(status::Ok);
    
    return Ok(resp);
}
*/
/*
pub fn users_json(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called random");
    
    let conn = Connection::open("./sqlite3.db").unwrap();
    let mut stmt = conn.prepare("SELECT username FROM user").unwrap();
    let user_iter = stmt.query_map(&[], |row| {
        let a:String = row.get(0);
        a
    }).unwrap();

    let mut v2 = Vec::new();
    for user in user_iter {
        v2.push(user.unwrap());
    }
    let usernames = UserNames { usernames: v2 };
    let payload = json::encode(&usernames).unwrap();
    Ok(Response::with((status::Ok, payload)))
}
*/
pub fn random(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called random");
    
    let mut data = HashMap::new();
    let mut resp = Response::new();
    let mut rng = thread_rng();
    let mut users = User::all();
    rng.shuffle(&mut users);
    data.insert(String::from("users"), users);
    resp.set_mut(Template::new("random", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn register_get(req: &mut Request) -> IronResult<Response> {
    
    let filename = "register.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css", "register.css"],
        "js": ["register.js"],
    });

    let rslt_html = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    let mut resp = Response::new();
    resp
        .set_mut(rslt_html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));
    
    return Ok(resp);
}

pub fn register_post(req: &mut Request) -> IronResult<Response> {
    
    
    println!("[+] Called register");
    {
        let map = req.get_ref::<Params>().unwrap();

        let token = match map.find(&["invite_token"]) {
            Some(&Value::String(ref name)) => Some(name),
            _ => None,
        };
        
        if token.is_none() {
            println!("[!] Invite token is None");
            let mut h = HashMap::new();
            h.insert("result", "invalid parameter");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        if token.unwrap() != &env::var("CPAW_TOKEN").expect("Please set 'CPAW_TOKEN' environment variable") {
            println!("[!] Invalid token");
            let mut h = HashMap::new();
            h.insert("result", "Invalid invite token");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        let username = match map.find(&["username"]){
            Some(&Value::String(ref name))  => Some(name),
            _ => None,
        };
        
        if username.is_none() {
            println!("[!] Username is None");
            let mut h = HashMap::new();
            h.insert("result", "invalid parameter");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        if username.unwrap() == "" {
            println!("[!] Username is empty");
            let mut h = HashMap::new();
            h.insert("result", "parameter is empty");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        println!("[+] Username {}", username.unwrap());
        
        let password = match map.find(&["password"]) {
            Some(&Value::String(ref name))  => Some(name),
            _ => None,
        };
        
        if password.is_none() {
            println!("[!] Password is None");
            let mut h = HashMap::new();
            h.insert("result", "invalid parameter");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        if password.unwrap() == "" {
            println!("[!] Password is empty");
            let mut h = HashMap::new();
            h.insert("result", "parameter is empty");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        println!("[+] Password {}", password.unwrap());
        
        let email = match map.find(&["email"]) {
            Some(&Value::String(ref name))  => Some(name),
            _ => None,
        };

        if email.is_none() {
            println!("[!] Email is None");
            let mut h = HashMap::new();
            h.insert("result", "invalid parameter");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }

        if email.unwrap() == "" {
            println!("[!] Email is empty");
            let mut h = HashMap::new();
            h.insert("result", "parameter is empty");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }

        // TODO メールアドレスの検証
        // 1. @で分割した際に要素が２つかどうか
        // 2. 分割した各要素がasciiのprintabeかどうか
        // 3. 分割した各要素に半角スペース等の区切り文字がないか
        // 4. 名前解決できるかどうか
        use std::ascii::AsciiExt;
        let email_splited: Vec<&str> = email.unwrap().split("@").collect();
        if email_splited.len() != 2 ||
            !email_splited[0].is_ascii() ||
            !email_splited[1].is_ascii()
        {
            println!("[!] Email validation error");
            let mut h = HashMap::new();
            h.insert("result", "email validation error");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        println!("[+] Email {}", email.unwrap());

        let bio = match map.find(&["bio"]) {
            Some(&Value::String(ref name))  => Some(name),
            _ => None,
        };

        if bio.is_none() {
            println!("[!] bio is None");
            let mut h = HashMap::new();
            h.insert("result", "invalid parameter");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }

        if bio.unwrap() == "" {
            println!("[!] Bio is empty");
            let mut h = HashMap::new();
            h.insert("result", "parameter is empty");
            return Ok(Response::with((status::Ok,
                                      json::encode(&h).unwrap())));
        }
        
        println!("[+] Bio {}", bio.unwrap());

        // to_string() means &str to std::string::String;
        let result = User::new(email.unwrap().to_string(),
                  username.unwrap().to_string(),
                  password.unwrap().to_string(),
                  bio.unwrap().to_string(),
                  username.unwrap().to_string());

        match result {
            Ok(_) => { println!("[+] User registered"); }
            Err(err_str) => {
                println!("{}", err_str);

                let mut h = HashMap::new();
                h.insert("result", err_str);
                return Ok(Response::with(
                    (status::Ok, json::encode(&h).unwrap())));
            }
        }
    }
    let mut h = HashMap::new();
    h.insert("result", true);
    return Ok(Response::with(
        (status::Ok, json::encode(&h).unwrap())));
}

pub fn timer(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called timer");
    
    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("timer", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn template_html(filename: &str) -> Handlebars {
    let mut handlebars = Handlebars::new();
    
    handlebars
        .register_template_file(filename, &Path::new(&["src/templates/", filename].join("")))
        .ok()
        .unwrap();

    handlebars
        .register_template_file("base", &Path::new("src/templates/base.hbs"))
        .ok()
        .unwrap();
    handlebars
}

pub fn users(req: &mut Request) -> IronResult<Response> {

    let filename = "users.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["material.css", "member.css"],
        "js": ["member.js", "jquery.csv.js", "minigrid.min.js"],
        "users_ob": User::all(),
    });
    let rslt_html = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    let mut resp = Response::new();
    resp
        .set_mut(rslt_html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));
    
    return Ok(resp);
}

pub fn about(req: &mut Request) -> IronResult<Response> {

    let filename = "about.hbs";
    let handlebars = template_html(filename);
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
    
    return Ok(resp);
}

pub fn index(req: &mut Request) -> IronResult<Response> {

    let filename = "index.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "index": true,
        "css": ["index.css"],
    });
    let rslt_html = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    let mut resp = Response::new();
    resp
        .set_mut(rslt_html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));
    
    return Ok(resp);
}

pub fn activity(req: &mut Request) -> IronResult<Response> {

    let filename = "activity.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["activity.css"],
    });
    let rslt_html = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );
    let mut resp = Response::new();
    resp
        .set_mut(rslt_html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));
    
    return Ok(resp);
}

// URLを基に更新対象のUserを返す
pub fn user_update_valid(req: &mut Request) -> Result<User,Response> {
    let target_username:String = req.extensions
        .get::<Router>().unwrap()
        .find("username").unwrap_or("/")
        .to_string();

    println!("[ ] /user/{}", target_username);

    // Login確認
    let current_user:User = match login::current_user(req) {
        Ok(u) => u,
        Err(_) => {
            println!("[ ] Please login");
            return Err(Response::with((status::Found, Redirect(url_for!(req, "login")))));
        }
    };

    // 存在確認
    let target_user = match User::find_by("username", &target_username) {
        Some(user) => user,
        None => {
            println!("[ ] User \"{}\" not found", target_username);
            return Err(Response::with((status::Found, Redirect(url_for!(req, "register")))));
        }
    };

    // 自身 or 権限持ち
    if current_user.username != target_user.username && current_user.permission != 1 {
        println!("[ ] Different user or not permission");
        return Err(Response::with((status::Found, Redirect(url_for!(req, "top")))));
    }

    Ok(target_user)
}

pub fn make_csrf_token(id: String) -> String {
    let mut buf = id + &env::var("CPAW_TOKEN").unwrap();
    let mut sha = Sha512::new();
    sha.input_str(&buf);
    sha.result_str()
}

pub fn user_update_get(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called user_update_get");
    let mut id = req.session().get::<UserSession>().unwrap().unwrap().id.to_string();
    let target_user = match user_update_valid(req) {
        Ok(user) => user,
        Err(res) => { return Ok(res); }
    };

    let csrf_token = make_csrf_token(
        req.session().get::<UserSession>().unwrap().unwrap().id.to_string()
    );
    let filename = "user_update.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css", "user_update.css"],
        "js": ["user_update.js"],
        "user": target_user,
        "csrf_token": csrf_token
    });

    let html:String = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    let mut resp = Response::new();
    resp
        .set_mut(html)
        .set_mut(status::Ok)
        .set_mut(Header(headers::ContentType::html()));

    return Ok(resp);
}

pub fn user_update_patch(req: &mut Request) -> IronResult<Response> {

    let user = req.session().get::<UserSession>().unwrap().unwrap();
    println!("[+] Called user_update_patch");

    let mut user:User = match user_update_valid(req) {
        Ok(user) => user,
        Err(_) => {
            let json = json!({"result": "Invalid user"});
            return Ok(response_json(json));
        }
    };
    
    let map = req.get_ref::<Params>().unwrap();
    let email    = take_param!(map, "email", Value::String);
    let username = take_param!(map, "username", Value::String);
    let password = take_param!(map, "password", Value::String);
    let bio      = take_param!(map, "bio", Value::String);
    let csrf_token = take_param!(map, "csrf_token", Value::String);
    
    println!("[ ] email:    \"{}\"", email.unwrap_or(&"None".to_string()));
    println!("[ ] username: \"{}\"", username.unwrap_or(&"None".to_string()));
    println!("[ ] password: \"{}\"", password.unwrap_or(&"None".to_string()));
    println!("[ ] bio:      \"{}\"", bio.unwrap_or(&"None".to_string()));
    
    println!("[ ] csrf token: \"{}\"", csrf_token.unwrap_or(&"None".to_string()));


    
    if csrf_token.unwrap().clone() != make_csrf_token(user.id.to_string()) {
        println!("[!] Invalid csrf token");
        return Ok(response_json(json!({"result": "Invalid csrf token"})));
    }
    
    fn already_used(target_user: &User, key: &str, new_value: &String) -> bool{
        match User::find_by(key, new_value) {
            Some(u) => u.id != target_user.id,
            None => false
        }
    }

    if email.is_some() && !username.unwrap().is_empty() {
        if already_used(&user, "email", email.unwrap()) {
            return Ok(response_json(json!({"result": "This email address already used"})))
        }
        user.email = email.unwrap().clone();
    }

    if username.is_some() && !username.unwrap().is_empty() {
        if already_used(&user, "username", username.unwrap()) {
            return Ok(response_json(json!({"result": "This username already used"})))
        }
        user.username = username.unwrap().clone();
    }

    if password.is_some() && !password.unwrap().is_empty() {
        user.set_password(password.unwrap());
    }

    if bio.is_some() {
        user.bio = bio.unwrap().clone();
    }

    if user.update() {
        println!("[ ] Update user");
        Ok(response_json(json!({"result": true, "username": user.username})))
    }
    else {
        println!("[ ] Failed to update user");
        Ok(response_json(json!({"result": "Update failed"})))
    }
}
