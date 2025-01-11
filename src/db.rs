extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_logger::LoggingConnection;


use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> LoggingConnection<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    LoggingConnection::new(conn)
}





