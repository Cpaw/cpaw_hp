extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate staticfile;
extern crate params;
extern crate mount;

use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use iron::prelude::*;
use iron::status;
use router::{Router, url_for};
use hbs::{Template, HandlebarsEngine, DirectorySource};
use staticfile::Static;
use mount::Mount;

mod login;
mod sql;
mod routing;

fn main() {
    
    fn top_handler(req: &mut Request) -> IronResult<Response> {

        println!("[+] Called top_handler");
        
        let mut resp = Response::new();
        let mut data = HashMap::new();
        
        data.insert(String::from("greeting_path"),format!("{}", url_for(req, "greeting", HashMap::new())));

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
    // 末尾のやつ同じだと駄目
    let mut router = Router::new();
    router.get("/", top_handler, "top");
    router.get("/index", top_handler, "index");
    router.get("/users", routing::users, "users");
    router.post("/greet", greet_handler, "greeting");
    router.post("/login", login::login, "login");
    router.get("/register", sql::register_get, "register");
    router.post("/register", sql::register, "register"); // OK

    // Mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/assets/", Static::new(Path::new("src/templates/assets/")));

     //Create Chain
    let mut chain = Chain::new(mount);
    
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
