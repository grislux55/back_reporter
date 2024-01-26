mod api;
mod orm;

use api::wechat_login::wechat_login_service;
use rocket::{get, routes};

const APPID: &str = "your_appid";
const SECRET: &str = "your_secret";

#[get("/")]
fn hello_world() -> String {
    "Hello, world!".to_string()
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let mut app = rocket::build();

    let db = orm::establish_connection().await?;

    app = app.manage(db);

    app = app.mount("/", routes![hello_world, wechat_login_service]);

    app.launch().await?;

    Ok(())
}
