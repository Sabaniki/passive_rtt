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
    let update_target = dsl::raw_rtts
        .filter(dsl::src.eq(&new_rtt.src))
        .filter(dsl::dst.eq(&new_rtt.dst))
        .filter(dsl::sid.eq(&new_rtt.sid));
    let count = update_target.execute(connection).expect("Error counting raw_rtts");
    if 0 < count {
        diesel::update(update_target)
        .set(new_rtt)
        .execute(connection)
        .expect("Error updating new_rtt");
    }
    else {
        insert_into(dsl::raw_rtts)
            .values(new_rtt)
            .execute(connection)
            .expect("Error saving new rtt");
    }
}

#[derive(Insertable, QueryableByName, Queryable, AsChangeset)]
#[table_name = "raw_rtts"]
pub struct RawRtt {
    pub src: String,
    pub dst: String,
    pub sid: Option<String>,
    pub rtt: u32,
}