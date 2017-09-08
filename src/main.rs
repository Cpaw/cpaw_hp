#[macro_use] extern crate iron;
extern crate iron_sessionstorage;
#[macro_use] extern crate router;
extern crate handlebars_iron as hbs;
extern crate staticfile;
extern crate params;
extern crate mount;
extern crate rustc_serialize;
extern crate rand;
extern crate handlebars;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate maplit;
extern crate dotenv;

use std::path::Path;
use std::error::Error;
use std::env;

use iron::prelude::*;
use router::Router;
use hbs::{HandlebarsEngine, DirectorySource};
use handlebars::Handlebars;
use staticfile::Static;
use mount::Mount;

use self::iron_sessionstorage::traits::*;
use self::iron_sessionstorage::SessionStorage;
use self::iron_sessionstorage::backends::SignedCookieBackend;

#[macro_use] mod routing;
mod login;
mod user;

fn main() {
    // .envの環境変数読み込み
    dotenv::dotenv().ok();

    //Create Router
    // 末尾のやつ同じだと駄目
    let mut router = Router::new();
    
    router.get("/", routing::index, "top");
    router.get("/index", routing::index, "index");
    router.get("/users", routing::users, "users");
    router.get("/about", routing::about, "about");
    //router.get("/blog", routing::blog, "blog");
    router.get("/random", routing::random_get, "random");
    router.get("/activity", routing::activity, "activity");
    router.get("/login", login::login_get, "login");
    router.post("/login", login::login_post, "login");
    router.get("/logout", login::logout, "logout");
    router.get("/register", routing::register_get, "register");
    router.post("/register", routing::register_post, "register");
    router.get("/timer", routing::timer, "timer");
    router.get("/username.json", login::current_user_json, "username");
    router.get("/user/:username", routing::user_update_get, "user_update_get");
    router.patch("/user/:username", routing::user_update_patch, "user_update_patch");
    
    // Mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/assets/", Static::new(Path::new("src/templates/assets/")));

     //Create Chain
    let mut chain = Chain::new(mount);
    
    // Setup SessionStorage
    let my_secret = env::var("CPAW_SECRET").expect("Please set 'CPAW_SECRET' environment variable").as_bytes().to_vec();
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));

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
