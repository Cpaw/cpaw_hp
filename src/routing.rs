extern crate rusqlite;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use iron::modifiers::{Redirect};
use hbs::{Template};
use params::{Params, Value};
use self::rusqlite::Connection;
use rustc_serialize::json;
use sql::Blog;
use user::User;


pub fn index(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called index");
    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    return Ok(resp);
}

pub fn activity(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called activity");
    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("activity", data)).set_mut(status::Ok);
    return Ok(resp);
}

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
    
    let mut v = Vec::new();
    for user in User::all() {
        let mut h = HashMap::new();
        h.insert(String::from("username"), user.username);
        h.insert(String::from("bio"), user.bio);
        h.insert(String::from("graphic"), user.graphic);
        v.push(h);
    }
    resp.set_mut(Template::new("users", v)).set_mut(status::Ok);
    
    return Ok(resp);
}

pub fn about(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called about");

    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("about", data)).set_mut(status::Ok);
    
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

pub fn random(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called random");
    let mut resp = Response::new();
    let data: HashMap<String, String> = HashMap::new();
    resp.set_mut(Template::new("random", data)).set_mut(status::Ok);
    return Ok(resp);
}
*/
pub fn register(req: &mut Request) -> IronResult<Response> {

    //TODO 登録出来る人を制限するコードを書く

    println!("[+] Called register");
    {
        let map = req.get_ref::<Params>().unwrap();
        
        let username = match map.find(&["username"]) {
        Some(&Value::String(ref name))  => {
            name
        },
        _ => {
            "fail"
        }
        };
        println!("[+] Username {}", username);
        
        let password = match map.find(&["password"]) {
            Some(&Value::String(ref name))  => {
                name
            },
            _ => {
                "fail"
            }
        };
    
        println!("[+] Password {}", password);
        
        let email = match map.find(&["email"]) {
            Some(&Value::String(ref name))  => {
                name
            },
            _ => {
                "fail"
            }
        };
    
        println!("[+] Email {}", email);

        let bio = match map.find(&["bio"]) {
            Some(&Value::String(ref name))  => {
                name
            },
            _ => {
                "fail"
            }
        };
        println!("[+] Bio {}", bio);
        
        // to_string() means &str to std::string::String;
        User::new(email.to_string(),
                  username.to_string(),
                  password.to_string(),
                  bio.to_string(),
                  username.to_string());
    }
    
    let ref top_url = url_for(req, "index", HashMap::new());
    return Ok(Response::with((status::Found, Redirect(top_url.clone()))))
}

