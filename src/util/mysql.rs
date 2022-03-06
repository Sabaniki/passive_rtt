use diesel::mysql::MysqlConnection;
use diesel::{prelude::*, insert_into};
use dotenv::dotenv;
use std::env;
use crate::schema::raw_rtts::{self, dsl};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL is not found!'");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn update_db(new_rtt: &RawRtt, connection: &MysqlConnection) {
    insert_into(dsl::raw_rtts)
        .values(new_rtt)
        .execute(connection)
        .expect("Error saving new rtt");
}

#[derive(Insertable, QueryableByName, Queryable, AsChangeset)]
#[table_name = "raw_rtts"]
pub struct RawRtt {
    pub src: String,
    pub dst: String,
    pub sid: Option<String>,
    pub rtt: u32,
}