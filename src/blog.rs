extern crate rusqlite;
extern crate time;

use std::env;
use std::vec::Vec;
use self::time::Timespec;

use self::rusqlite::Connection;
use self::rusqlite::types::ToSql;
use user::get_connection;


#[derive(Debug)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub author: String,
    pub time_posted: Timespec,
    pub time_updated: Timespec,
}

impl Blog {

    pub fn create(&self) -> bool {
        let conn = get_connection();
        conn.execute("INSERT INTO blogs (title, author, body, time_posted, time_updated)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
                  &[&self.title, &self.author, &self.body, &self.time_posted, &self.time_updated]).unwrap();
        true
    }

    pub fn update(&self) -> bool {
        let conn = get_connection();
        conn.execute("UPDATE blog SET (title, author, body, time_posted, time_updated)
                        VALUES (?1, ?2, ?3, ?4, ?5)",
                        &[&self.title, &self.author, &self.body, &self.time_posted, &self.time_updated]).unwrap();
        true
    }

}
