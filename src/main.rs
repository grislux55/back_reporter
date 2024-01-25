mod api;
mod orm;

use api::wechat_login::wechat_login_service;
use rocket::routes;

const APPID: &str = "your_appid";
const SECRET: &str = "your_secret";

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let mut app = rocket::build();

    let db = orm::establish_connection().await?;

    app = app.manage(db);

    app = app.mount("/", routes![wechat_login_service]);

    app.launch().await?;

    Ok(())
}
