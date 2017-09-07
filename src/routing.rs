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


// --- Helpers ---

// enum内の値を取り出す
macro_rules! cast_enum {
    ($src:expr, $type:path) => {
        match $src {
            $type(v) => Some(v),
            _ => None
        }
    }
}

// 参照を返す
// take_param!(map, "key", Value::String) でOption<String>な値を取り出す
macro_rules! take_param {
    ($map:expr, $key:expr, $type:path) => {
        match $map.find(&[$key]) {
            // ラップされたenumの値の参照を返す
            Some(&$type(ref value)) => Some(value),
            _ => None,
        }
    }
}

// 実態を返す
// Option<Vec<TYPE>>
macro_rules! take_param_array {
    ($map:expr, $key:expr, $type:path) => {
        match take_param!($map, $key, params::Value::Array) {
            Some(param_vec) => {
                let vec = param_vec
                    .iter()
                    .map(|param_val| {
                        // Clone
                        cast_enum!(param_val.to_owned(), $type)
                            .expect(concat!("Expect ", stringify!($type)))
                    })
                    .collect::<Vec<_>>();
                Some(vec)
            },
            _ => None
        };
    }
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

pub fn response_html(html: String) -> Response {
    Response::new()
        .set(status::Ok)
        .set(Header(headers::ContentType::html()))
        .set(html)
}

pub fn response_json(json: serde_json::Value) -> Response {
    Response::new()
        .set(status::Ok)
        .set(Header(headers::ContentType::json()))
        .set(json.to_string())
}

// URLを基に更新対象のUserを返す
pub fn user_update_valid(req: &mut Request) -> Result<User,Response> {

    println!("[+] Called user update valid");
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
    //println!("[+] passed exsists"); PASS
    // 自身 or 権限持ち
    if current_user.username != target_user.username && current_user.permission != 1 {
        println!("[ ] Different user or not permission");
        return Err(Response::with((status::Found, Redirect(url_for!(req, "top")))));
    }

    Ok(target_user)
}

// TODO メールアドレスの検証
pub fn email_valid(email: &String) -> bool {
    // 1. @で分割した際に要素が２つかどうか
    // 2. 分割した各要素がasciiのprintabeかどうか
    // 3. 分割した各要素に半角スペース等の区切り文字がないか
    // 4. 名前解決できるかどうか
    use std::ascii::AsciiExt;

    let email_splited: Vec<&str> = email.split("@").collect();

    email_splited.len() != 2 ||
        !email_splited[0].is_ascii() ||
        !email_splited[1].is_ascii()
}


// --- Routing handlers ---

pub fn timer(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called timer");

    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("timer", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn register_get(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called register_get");
    let filename = "register.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css", "user.css"],
        "js": ["register.js"],
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn register_post(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called register_post");

    let map = req.get_ref::<Params>().unwrap();

    let token = take_param!(map, "invite_token", Value::String);

    if token.is_none() {
        println!("[!] Invite token is None");
        return Ok(response_json(json!({"result": "invalid parameter"})))
    }

    if token.unwrap() != &env::var("CPAW_TOKEN").expect("Please set 'CPAW_TOKEN' environment variable") {
        println!("[!] Invalid token");
        return Ok(response_json(json!({"result": "Invalid invite token"})))
    }

    let username = take_param!(map, "username", Value::String);

    if username.is_none() {
        println!("[!] Username is None");
        return Ok(response_json(json!({"result": "invalid parameter"})))
    }

    if username.unwrap() == "" {
        println!("[!] Username is empty");
        return Ok(response_json(json!({"result": "username is empty"})))
    }

    println!("[+] Username {}", username.unwrap());

    let password = take_param!(map, "password", Value::String);

    if password.is_none() {
        println!("[!] Password is None");
        return Ok(response_json(json!({"result": "invalid parameter"})))
    }

    if password.unwrap() == "" {
        println!("[!] Password is empty");
        return Ok(response_json(json!({"result": "password is empty"})))
    }

    println!("[+] Password {}", password.unwrap());

    let email = take_param!(map, "email", Value::String);

    if email.is_none() {
        println!("[!] Email is None");
        return Ok(response_json(json!({"result": "invalid parameter"})))
    }

    if email.unwrap() == "" {
        println!("[!] Email is empty");
        return Ok(response_json(json!({"result": "email is empty"})))
    }

    if email_valid(email.unwrap()) {
        println!("[!] Email validation error");
        return Ok(response_json(json!({"result": "email validation error"})))
    }

    println!("[+] Email {}", email.unwrap());

    let bio = take_param!(map, "bio", Value::String);

    if bio.is_none() {
        println!("[!] Bio is None");
        return Ok(response_json(json!({"result": "invalid parameter"})))
    }

    if bio.unwrap() == "" {
        println!("[!] Bio is empty");
        return Ok(response_json(json!({"result": "bio is empty"})))
    }

    println!("[+] Bio {}", bio.unwrap());

    let twitter = match take_param!(map, "twitter", Value::String) {
        Some(twitter) => twitter.to_owned(),
        None => {
            println!("[!] Twitter is None");
            return Ok(response_json(json!({"result": "invalid parameter"})))
        }
    };
    println!("[+] Twitter {}", twitter);

    let facebook = match take_param!(map, "facebook", Value::String) {
        Some(facebook) => facebook.to_owned(),
        None => {
            println!("[!] Facebook is None");
            return Ok(response_json(json!({"result": "invalid parameter"})))
        }
    };
    println!("[+] Facebook {}", facebook);

    let tags:Vec<String> = match take_param_array!(map, "tags", Value::String) {
        Some(tags) => tags,
        None => {
            println!("[!] tags is None");
            return Ok(response_json(json!({"result": "invalid parameter"})))
        }
    };
    println!("[+] Tags {:?}", tags);

    let result = User::new(
                    email.unwrap().to_string(),
                    username.unwrap().to_string(),
                    password.unwrap().to_string(),
                    bio.unwrap().to_string(),
                    username.unwrap().to_string(),
                    twitter,
                    facebook,
                    tags);

    match result {
        Ok(_) => { println!("[+] User registered"); }
        Err(err_str) => {
            println!("{}", err_str);
            return Ok(response_json(json!({"result": err_str})))
        }
    }

    Ok(response_json(json!({"result": true})))
}

pub fn users(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called users");
    let filename = "users.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["material.css", "member.css"],
        "js": ["member.js", "jquery.csv.js", "minigrid.min.js"],
        "users_ob": User::all(),
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn about(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called about");
    let filename = "about.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css"],
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn index(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called index");
    let filename = "index.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "index": true,
        "css": ["index.css"],
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn activity(req: &mut Request) -> IronResult<Response> {
    println!("[+] Called activity");
    let filename = "activity.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["activity.css"],
    });

    let html_str = handlebars.render(filename, &data).unwrap_or_else(
        |e| format!("{}", e),
    );

    Ok(response_html(html_str))
}

pub fn make_csrf_token(id: String) -> String {
    
    println!("[+] Called make_csrf_token");
    
    let mut buf = id + &env::var("CPAW_TOKEN").unwrap();
    let mut sha = Sha512::new();
    sha.input_str(&buf);
    sha.result_str()
}

pub fn user_update_get(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called user_update_get");
    
    if !login::is_logged_in(req) {
        return Ok(Response::with((status::Found, Redirect(url_for!(req, "index")))));        
    }
    
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
        "css": ["about.css", "user.css"],
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
    let twitter  = take_param!(map, "twitter", Value::String);
    let facebook = take_param!(map, "facebook", Value::String);
    let tags:Option<Vec<String>> = take_param_array!(map, "tags", Value::String);
    let csrf_token = take_param!(map, "csrf_token", Value::String);
    
    println!("[ ] id:       {}", user.id);
    println!("[ ] email:    \"{}\"", email.unwrap_or(&"None".to_string()));
    println!("[ ] username: \"{}\"", username.unwrap_or(&"None".to_string()));
    println!("[ ] password: \"{}\"", password.unwrap_or(&"None".to_string()));
    println!("[ ] bio:      \"{}\"", bio.unwrap_or(&"None".to_string()));
    println!("[ ] twitter:  \"{}\"", twitter.unwrap_or(&"None".to_string()));
    println!("[ ] facebook: \"{}\"", facebook.unwrap_or(&"None".to_string()));
    println!("[ ] tags:     {:?}",   tags.as_ref().unwrap_or(&vec![]));
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

    if twitter.is_some() {
        user.twitter = twitter.unwrap().clone();
    }

    if facebook.is_some() {
        user.facebook = facebook.unwrap().clone();
    }

    if tags.is_some() {
        user.tags = tags.unwrap(); // Move
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

pub fn random_get(req: &mut Request) -> IronResult<Response> {

    let filename = "random.hbs";
    let handlebars = template_html(filename);
    let data = json!({
        "parent": "base",
        "css": ["about.css"],
        "js": ["random.js"],
        "users": User::all()
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
