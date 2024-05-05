use mysql::{prelude::Queryable, Pool, PooledConn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    password: String,
    phone: String,
    email: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub dob: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninRequest {
    pub username: String,
    password: String,
}

pub struct DB {
    pool: Pool,
}

impl DB {
    pub async fn init() -> Result<Self, ()> {
        let url = "mysql://root:root123@localhost:3306/baatein";
        let pool: Pool = Pool::new(url).expect("error connecting to database");
        // let mut conn = pool.get_conn().unwrap();

        Ok(Self { pool })
    }

    async fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub async fn create_table(&self) {
        self.get_conn()
            .await
            .query_drop(
                "CREATE TABLE users (
                            username varchar(20) primary key not null,
                            password varchar(256) not null,
                            phone varchar(10) not null unique,
                            email varchar(60) not null unique
                        )",
            )
            .unwrap();
    }

    pub async fn add_user(&self, req: SignupRequest) {
        self.get_conn()
            .await
            .exec_drop(
                r"INSERT INTO users VALUES (?, ?, ?, ?)",
                (req.username, req.password, req.phone, req.email),
            )
            .expect("error adding user");
    }

    pub async fn user_exists(&self, username: String) -> bool {
        let user: Option<String> = self
            .get_conn()
            .await
            .query_first(format!(
                "SELECT username FROM users WHERE username=\"{username}\"",
            ))
            .expect("error fetching user");

        match user {
            Some(_u) => true,
            None => false,
        }
    }

    pub async fn authorize_user(&self, user_cred: SigninRequest) -> bool {
        let username = user_cred.username;
        let pass: Option<String> = self
            .get_conn()
            .await
            .query_first(format!(
                "SELECT password FROM users WHERE username=\"{username}\"",
            ))
            .unwrap();

        match pass {
            Some(pass) => {
                pass == user_cred.password
            },
            None => {
                // handle password not found error
                false
            }
        }
    }
}
