extern crate crypto;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;

use std::env;
use std::vec::Vec;

use self::crypto::sha2::Sha512;
use self::crypto::digest::Digest;

use self::rusqlite::Connection;
use self::rusqlite::types::ToSql;


pub fn get_connection() -> Connection {
    let db_path = env::var("DATABASE_URL").expect("Please set 'DATABASE_URL' environment variable");
    Connection::open(db_path).unwrap()
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub permission: i32,
    pub bio: String,
    pub twitter: String,
    pub facebook: String,
    pub tags: Vec<String>,
}

impl User {
    fn tags_from_json_str(json_str: String) -> Vec<String> {
        serde_json::from_str(&json_str[..]).expect("tags expect Vec<String>")
    }

    pub fn set_password(&mut self, password: &String) {
        let mut sha = Sha512::new();
        sha.input_str(password);
        self.password = sha.result_str();
    }

    pub fn tags_to_json_str(&self) -> String {
        json!(self.tags).to_string()
    }

    pub fn insert(&self) -> bool {
        let conn = get_connection();
        let result = conn.execute(
            "INSERT INTO users (email, username, password, permission, bio, twitter, facebook, tags)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&self.email, &self.username, &self.password, &self.permission,
              &self.bio, &self.twitter, &self.facebook, &self.tags_to_json_str()]);

        match result {
            Ok(_) => { true },
            Err(_) => { false }
        }
    }

    // 変更を反映する
    pub fn update(&self) -> bool {
        let conn = get_connection();
        let result = conn.execute(
            "UPDATE users SET email=?2, username=?3, password=?4, permission=?5,
                              bio=?6, twitter=?7, facebook=?8, tags=?9
             WHERE id=?1",
            &[&self.id, &self.email, &self.username, &self.password,
                &self.permission, &self.bio,
                &self.twitter, &self.facebook, &self.tags_to_json_str()]);

        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn all() -> Vec<User> {
        let conn = get_connection();
        let mut stmt = conn.prepare("SELECT id, email, username, bio, twitter, facebook, tags FROM users").unwrap();
        let user_iter = stmt.query_map(&[], |row| {
            User {
                id: row.get(0),
                email: row.get(1),
                username: row.get(2),
                password: "".to_string(),
                permission: 0,
                bio: row.get(3),
                twitter: row.get(4),
                facebook: row.get(5),
                tags: User::tags_from_json_str(row.get(6)),
            }
        }).unwrap();

        let mut users: Vec<User> = Vec::new();
        for user in user_iter {
            users.push(user.unwrap());
        }

        users
    }

    pub fn find_by(key: &str, value: &ToSql) -> Option<User> {
        let conn = get_connection();
        // TODO Danger
        let mut stmt = conn.prepare(&format!("SELECT id, email, username, password, permission,
                                             bio, twitter, facebook, tags FROM users WHERE {} = ?", key)[..]).unwrap();
        let result_users = stmt.query_map(&[value], |row| {
            User {
                id: row.get(0),
                email: row.get(1),
                username: row.get(2),
                password: row.get(3),
                permission: row.get(4),
                bio: row.get(5),
                twitter: row.get(6),
                facebook: row.get(7),
                tags: User::tags_from_json_str(row.get(8)),
            }
        });

        // なぜかmutが必要
        let mut users = match result_users {
            Ok(users) => { users }
            Err(_) => { return None; }
        };

        let user = match users.nth(0) {
            Some(result_first_user) => {
                match result_first_user {
                    Ok(first_user) => { first_user },
                    Err(_) => { return None; }
                }
            }
            None => { return None; }
        };

        Some(user)
    }

    pub fn find(id: i32) -> Option<User> {
        User::find_by(&"id", &id)
    }

    pub fn delete(id: i32) -> bool {
        let conn = get_connection();
        conn.execute("DELETE FROM users WHERE id = ?1", &[&id]).is_ok()
    }

    pub fn new(email: String, username: String, password: String, bio: String,
               twitter: String, facebook: String, tags: Vec<String>) -> Result<User, String> {
        if User::find_by(&"email", &email).is_some() {
            return Err("This email already registered".to_string());
        }

        if User::find_by(&"username", &username).is_some() {
            return Err("This username already registered".to_string());
        }

        let mut u = User {
            id: -1, // ダミー
            email: email,
            username: username,
            password: "".to_string(),
            permission: 0,
            bio: bio,
            twitter: twitter,
            facebook: facebook,
            tags: tags,
        };
        u.set_password(&password);

        if !u.insert() {
            return Err("Fialed to insert new user".to_string());
        }

        return match User::find_by("email", &u.email) {
            Some(user) => { Ok(user) },
            None => { Err("Failed to register new user".to_string()) }
        };
    }
}
