use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use std::env;



const DATABASE_URL: &str = "mysql://user:pass@localhost:6000";
const DB_NAME: &str = "my_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    println!("{:?}",db);
    Ok(())
}
fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}