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
//use std::path::Path;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub permission: u8
}

pub fn create_db() {
    
}

/*
        conn.execute("CREATE TABLE user (
                  id              INTEGER PRIMARY KEY,
                  username        TEXT NOT NULL,
                  email           TEXT NOT NULL,
                  password        TEXT NOT NULL,
                  permission      INTEGER
                  
    )", &[]).unwrap();
            let me = User {
                id: 0,
                username: "Shige".to_string(),
                password: "cpaw".to_string(),
                email: "shige@cpaw.com".to_string(),
                permission: 1
            };

        conn.execute("INSERT INTO user (username, password, email, permission)
                  VALUES (?1, ?2, ?3, ?4)",
                     &[&me.username, &me.password, &me.email, &me.permission]).unwrap();

    }

    
    // from post parameter         
     */

pub fn register_get(req: &mut Request) -> IronResult<Response> {
    
    let mut resp = Response::new();
    let mut data = HashMap::new();
    
    data.insert(String::from("register_path"),
                format!("{}", url_for(req, "register", HashMap::new())));
    data.insert(String::from("title"),
                format!("{}", url_for(req, "register", HashMap::new())));
        
    resp.set_mut(Template::new("register", data)).set_mut(status::Ok);
    
    return Ok(resp);
}

pub fn register(req: &mut Request) -> IronResult<Response> {

    let conn = Connection::open("./sqlite3.db").unwrap();
    
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

    // to_string() means &str to std::string::String;
    let me = User {
        id: 0,
        username: username.to_string(),
        password: password.to_string(),
        email: email.to_string(),
        permission: 1
    };

        conn.execute("INSERT INTO user (username, password, email, permission)
                  VALUES (?1, ?2, ?3, ?4)",
                 &[&me.username, &me.password, &me.email, &me.permission]).unwrap();
    }
    
    let ref top_url = url_for(req, "index", HashMap::new());
    return Ok(Response::with((status::Found, Redirect(top_url.clone()))))
        
}


