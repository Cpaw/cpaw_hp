extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;


use std::collections::HashMap;
use std::error::Error;
use iron::prelude::*;
use iron::status;
use router::{Router, url_for};
use hbs::{Template, HandlebarsEngine, DirectorySource};
use params::{Params, Value};

pub fn login(req: &mut Request) -> IronResult<Response> {

    println!("[+] Called login");
    let map = req.get_ref::<Params>().unwrap();
    
    let username = match map.find(&["username"]) {
        Some(&Value::String(ref name))  => {
            name
        },
        _ => {
            "hoge"
        }
    };
    
    println!("[+] Username {}", username);
    let password = match map.find(&["password"]) {
        Some(&Value::String(ref name))  => {
            name
        },
        _ => {
            "hoge"
        }
    };
    
    println!("[+] Password {}", password);
    return Ok(Response::with((iron::status::Ok)))
}
