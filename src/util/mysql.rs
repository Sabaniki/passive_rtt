use diesel::mysql::MysqlConnection;
use diesel::{prelude::*, Queryable};
use dotenv::dotenv;
use std::env;
use crate::schema::rtts;
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL is not found!'");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// id  SERIAL PRIMARY KEY,
// src VARCHAR(39) NOT NULL,
// dst VARCHAR(39) NOT NULL,
// rtt INT UNSIGNED NOT NULL

#[derive(Insertable)]
#[table_name = "rtts"]
pub struct Rtt {
    pub id: String,
    pub src: String,
    pub dst: String,
    pub rtt: u32,
}