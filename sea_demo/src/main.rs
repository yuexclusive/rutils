mod entity;
use std::env;

use sea_orm::Database;

mod operation;
mod select;

use operation::*;
use select::*;

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();


    let url = env::var("DATABASE_URL").expect("please set DATABASE_URL");
    let db = Database::connect(url).await.unwrap();

    println!("{:?}\n", db);

    println!("===== =====\n");

    all_about_select(&db).await.unwrap();

    println!("===== =====\n");


    // all_about_operation(&db).await.unwrap();
}
