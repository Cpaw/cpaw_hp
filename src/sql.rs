extern crate rusqlite;
extern crate iron;
extern crate router;
extern crate handlebars_iron as hbs;
extern crate params;
extern crate rustc_serialize;
extern crate time;

use std::collections::HashMap;
use iron::prelude::*;
use iron::status;
use router::url_for;
use iron::modifiers::{Redirect};
use hbs::{Template};
use params::{Params, Value};
use self::rusqlite::Connection;
use self::rustc_serialize::json;
use self::time::Timespec;
//use std::path::Path;


#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub permission: u8
}

#[derive(RustcEncodable)]
pub struct UserNames {
    pub usernames: Vec<String>
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
