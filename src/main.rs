#[macro_use]
extern crate lazy_static;
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

mod login;
mod sql;

fn main() {
    
    fn top_handler(req: &mut Request) -> IronResult<Response> {
        
        let mut resp = Response::new();
        let mut data = HashMap::new();
        
        data.insert(String::from("greeting_path"),
                    format!("{}", url_for(req, "greeting", HashMap::new())));
        
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
        return Ok(resp);
    }
    
    fn greet_handler(req: &mut Request) -> IronResult<Response> {
        
        use params::{Params, Value};
        
        let map = req.get_ref::<Params>().unwrap();
        
        return match map.find(&["name"]) {
            Some(&Value::String(ref name)) => {
                Ok(Response::with(
                    (status::Ok,
                     format!("Hello {}", name).as_str())
                ))
            },
            _ => Ok(Response::with((status::Ok, "Hello world")))
        }
    }
    
    // Connect database(SQLite3)
    sql::create_db();
    
    //Create Router
    let mut router = Router::new();
    router.get("/", top_handler, "index");
    router.post("/greet", greet_handler, "greeting");
    router.post("/login", login::login, "login");
    router.get("/register", sql::register_get, "register");
    router.post("/register", sql::register, "register");
    
    //Create Chain
    let mut chain = Chain::new(router);
    // Add HandlerbarsEngine to middleware Chain
    let mut hbse = HandlebarsEngine::new();
    
    hbse.add(Box::new(
        DirectorySource::new("./src/templates/", ".hbs")
    ));
    
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);


    println!("[+] Listen on localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
