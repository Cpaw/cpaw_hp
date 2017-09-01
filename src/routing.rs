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
use sql::User;

pub fn users(req: &mut Request) -> IronResult<Response> {
    
    println!("[+] Called member");
    
    let mut resp = Response::new();
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
    
    resp.set_mut(Template::new("users", data)).set_mut(status::Ok);
    
    return Ok(resp);
}

