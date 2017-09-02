extern crate crypto;
extern crate rusqlite;
extern crate serde;

use std::vec::Vec;

use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;

use self::rusqlite::Connection;
use self::rusqlite::types::ToSql;

use self::serde::ser::{Serialize, Serializer, SerializeStruct};

static DB_PATH: &'static str = "db.sql";

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub permission: i32,
    pub bio: String,
    pub graphic: String,
}

// TODO Result
impl User {
    pub fn save(&self) -> bool {
        let conn = Connection::open(DB_PATH).unwrap();
        conn.execute("INSERT INTO user (email, username, password, permission, bio, graphic)
                  VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                  &[&self.email, &self.username, &self.password, &self.permission, &self.bio, &self.graphic]).unwrap();
        true // TODO
    }

    pub fn all() -> Vec<User> {
        let conn = Connection::open(DB_PATH).unwrap();
        let mut stmt = conn.prepare("SELECT id, email, username, bio, graphic FROM user").unwrap();
        let user_iter = stmt.query_map(&[], |row| {
            User {
                id: row.get(0),
                email: row.get(1),
                username: row.get(2),
                password: "".to_string(),
                permission: 0,
                bio: row.get(3),
                graphic: row.get(4),
            }
        }).unwrap();

        let mut users: Vec<User> = Vec::new();
        for user in user_iter {
            users.push(user.unwrap());
        }

        users
    }

    pub fn find_by(key: &str, value: &ToSql) -> Option<User> {
        let conn = Connection::open(DB_PATH).unwrap();
        // TODO Danger
        let mut stmt = conn.prepare(&format!("SELECT id, email, username, password, permission,
                                             bio, graphic FROM user WHERE {} = ?", key)[..]).unwrap();
        let result_users = stmt.query_map(&[value], |row| {
            User {
                id: row.get(0),
                email: row.get(1),
                username: row.get(2),
                password: row.get(3),
                permission: row.get(4),
                bio: row.get(5),
                graphic: row.get(6),
            }
        });

        // なぜかmutが必要
        let mut users = match result_users {
            Ok(users) => { users }
            Err(e) => { return None; }
        };

        let user = match users.nth(0) {
            Some(result_first_user) => {
                match result_first_user {
                    Ok(first_user) => { first_user },
                    Err(e) => { return None; }
                }
            }
            None => { return None; }
        };
        
        Some(user)
    }

    pub fn find(id: i32) -> Option<User> {
        User::find_by(&"id", &id)
    }

    // TODO return value
    pub fn delete(id: i32) {
        let conn = Connection::open(DB_PATH).unwrap();
        conn.execute("DELETE FROM user WHERE id = ?1", &[&id]).unwrap();
    }

    pub fn new(email: String, username: String, password: String,
               bio: String, graphic: String) -> Option<User> {
        let mut sha = Sha512::new();
        sha.input_str(&password);

        let u = User {
            id: -1,  // 適当
            email: email,
            username: username,
            password: sha.result_str(), // to hash
            permission: 0,
            bio: bio,
            graphic: graphic
        };
        u.save();
        User::find_by("email", &u.email)
    }
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
        {
            let mut s = serializer.serialize_struct("User", 3)?;
            s.serialize_field("username", &self.username)?;
            s.serialize_field("bio", &self.bio)?;
            s.serialize_field("graphic", &self.graphic)?;
            s.end()
        }
}
