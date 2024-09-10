use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use std::env;
use dotenvy::dotenv;



// const DATABASE_URL: &str = "mysql://user:pass@localhost:6000";
// const DB_NAME: &str = "my_db";

async fn run() -> Result<(), DbErr> {
    dotenv().expect(".env file not found");
    let db = Database::connect(env::var("DATABASE_URL").unwrap()).await?;
    println!("{:?}",db);
    Ok(())
}
fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}