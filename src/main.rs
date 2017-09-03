#[macro_use] extern crate iron;
extern crate iron_sessionstorage;
#[macro_use] extern crate router;
extern crate handlebars_iron as hbs;
extern crate staticfile;
extern crate params;
extern crate mount;
extern crate rustc_serialize;
extern crate rand;

use std::path::Path;
use std::error::Error;
use iron::prelude::*;
use router::Router;
use hbs::{HandlebarsEngine, DirectorySource};
use staticfile::Static;
use mount::Mount;

use self::iron_sessionstorage::traits::*;
use self::iron_sessionstorage::SessionStorage;
use self::iron_sessionstorage::backends::SignedCookieBackend;

mod login;
mod sql;
mod routing;
mod user;

fn main() {

    //Create Router
    // 末尾のやつ同じだと駄目
    let mut router = Router::new();

    router.get("/", routing::index, "top");
    router.get("/index", routing::index, "index");
    router.get("/users", routing::users, "users");
    router.get("/about", routing::about, "about");
    //router.get("/blog", routing::blog, "blog");
    router.get("/random", routing::random, "random");
    //router.get("/users_json", routing::users_json, "users_json");
    router.get("/invite_token.json", routing::invite_token, "invite_token");
    router.get("/activity", routing::activity, "activity");
    router.get("/login", login::login, "login");
    router.post("/login", login::login, "login");
    router.get("/register", routing::register, "register");
    router.post("/register", routing::register, "register");
    router.get("/timer", routing::timer, "timer");

    // Mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/assets/", Static::new(Path::new("src/templates/assets/")));

     //Create Chain
    let mut chain = Chain::new(mount);

    // Setup SessionStorage
    let my_secret = b"verysecret".to_vec(); // TODO Secret
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


    //sqlite_test::test_main();
    println!("[+] Listen on localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}
