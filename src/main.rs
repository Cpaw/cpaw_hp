extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate staticfile;
extern crate params;
extern crate mount;
extern crate rustc_serialize;

use std::path::Path;
use std::collections::HashMap;
use std::error::Error;
use iron::prelude::*;
use iron::status;
use router::{Router, url_for};
use hbs::{Template, HandlebarsEngine, DirectorySource};
use staticfile::Static;
use mount::Mount;
use rustc_serialize::json;
mod login;
mod sql;
mod routing;

fn main() {
    
    //Create Router
    // 末尾のやつ同じだと駄目
    let mut router = Router::new();
    
    router.get("/", routing::index, "top");
    router.get("/index", routing::index, "index");
    router.get("/users", routing::users, "users");
    router.get("/about", routing::about, "about");
    router.get("/blog", routing::blog, "blog");
    router.get("/random", routing::random, "random");
    router.get("/users_json", routing::users_json, "users_json");
    router.get("/activity", routing::activity, "activity");
    router.get("/random", routing::random, "random");
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
