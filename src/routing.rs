extern crate rusqlite;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate crypto;
extern crate serde_json;

use std::path::Path;
use std::collections::HashMap;
use std::path::Path;
use iron::prelude::*;
use iron::{headers, status};
use router::url_for;
use iron::modifiers::{Redirect, Header};
use handlebars::Handlebars;
use hbs::{Template};
use params::{Params, Value};
use self::rusqlite::Connection;
use rustc_serialize::json;
use sql::Blog;
use user::User;
use rand::{thread_rng, Rng};
use std::option::Option;
use std::env;
use handlebars::Handlebars;


pub fn users(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called member");


    let mut resp = Response::new();
    /*
    let mut data = HashMap::new();
    
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
    data.insert(String::from("usernames"), v2);
     */
    
    let mut data = HashMap::new();
    // let mut v = Vec::new();
    // for user in User::all() {
    //     let mut h = HashMap::new();
    //     h.insert(String::from("username"), user.username);
    //     h.insert(String::from("bio"), user.bio);
    //     h.insert(String::from("graphic"), user.graphic);
    //     v.push(h);
    // }

    data.insert(String::from("users"), User::all());
    resp.set_mut(Template::new("users", data)).set_mut(status::Ok);
    return Ok(resp);
}

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
    let mut resp = Response::new();
    let mut data = HashMap::new();
    data.insert("", "");
    resp.set_mut(Template::new("register", data)).set_mut(status::Ok);
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
        
        if token.unwrap() != env!("CPAW_TOKEN") {
            println!("[!] Invalid token");
            let mut h = HashMap::new();
            h.insert("result", "invalid token");
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
        let emailSplited: Vec<&str> = email.unwrap().split("@").collect();
        if emailSplited.len() != 2 ||
            !emailSplited[0].is_ascii() ||
            !emailSplited[1].is_ascii()
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
    
    let ref top_url = url_for(req, "index", HashMap::new());
    return Ok(Response::with((status::Found, Redirect(top_url.clone()))));
}

pub fn timer(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called timer");
    
    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("timer", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn invite_token(req: &mut Request) -> IronResult<Response> {

    //TODO 起動時にCPAW_TOKEN環境変数を定義する
    //コードを公開しない前提ならハードコーディングで良い?
    let mut h = HashMap::new();
    let token = env!("CPAW_TOKEN");
    println!("{}", token);
    h.insert("token", token);
    return Ok(Response::with((status::Ok,
                              json::encode(&h).unwrap())));
    
}

pub fn template_html(filename: &str) -> Handlebars {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file(filename, &Path::new(&["src/templates/", filename].connect("")))
        .ok()
        .unwrap();

    handlebars
        .register_template_file("base", &Path::new("src/templates/base.hbs"))
        .ok()
        .unwrap();
    handlebars
}

pub fn temp_test(req: &mut Request) -> IronResult<Response> {

    let filename = "test_about";
    let mut resp = Response::new();
    
    let mut handlebars = template_html(filename);
    let data1 =
        btreemap! {
            "parent".to_string() => "base".to_string()
        };

    let ret_html = handlebars.render(filename, &data1).unwrap_or_else(
        |e| format!("{}", e)
    );
    handlebars
}

// 助けてくれ〜〜〜〜〜〜
pub fn test(req: &mut Request) -> IronResult<Response> {

    let filename = "users.hbs";
    let mut handlebars = template_html(filename);

    // ここでうまくいい感じのMapを作る
    /*
    let mut inner_map = Map::new();
    inner_map.insert("users".to_string(), User::all());
    */
    let data =
        btreemap! {
            "users_flag".to_string() => "true".to_string(),
            "parent".to_string() => "base".to_string()
        };

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
    let mut handlebars = template_html(filename);
    let data =
        btreemap! {
             "parent".to_string() => "base".to_string()
        };

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
    let mut handlebars = template_html(filename);
    let data =
        btreemap! {
            "parent".to_string() => "base".to_string()
        };

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
    let mut handlebars = template_html(filename);
    let data =
        btreemap! {
            "parent".to_string() => "base".to_string()
        };

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
