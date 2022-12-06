#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod argus;
mod db;
mod guards;
mod log;
mod models;
mod schema;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
// #[macro_use]
extern crate rocket_sync_db_pools;

use argus::EnvLog;
use db::FilterData;
use db::{get_filters, get_logs};
use dotenv::dotenv;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use guards::{create_from_now, Admin};
use models::Log;
use rocket::fairing::{self, AdHoc};
use rocket::form::prelude::*;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::http::{Cookie, CookieJar};
use rocket::response::stream::{Event, EventStream};
use rocket::response::Redirect;
use rocket::serde::json::{to_string, Json};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
use rocket::tokio::sync::broadcast::{channel, Sender};
use rocket::{Build, Rocket, Shutdown, State};
use rocket_sync_db_pools::database;

#[database("sqlite_logs")]
pub struct MyConn(diesel::SqliteConnection);

embed_migrations!();

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let db = MyConn::get_one(&rocket).await.expect("database connection");
    db.run(|conn| match embedded_migrations::run(conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

#[post("/log?<count>&<before>&<after>&<share_name>&<client_name>&<client_ip>&<actions>&<search>")]
pub async fn getlog(
    _admin: Admin,
    conn: MyConn,
    count: Option<i64>,
    before: Option<i64>,
    after: Option<i64>,
    share_name: Option<String>,
    client_name: Option<String>,
    client_ip: Option<String>,
    actions: Option<String>,
    search: Option<String>,
) -> Json<Vec<Log>> {
    let mut results = get_logs(
        conn,
        count.unwrap_or(50),
        before,
        after,
        client_name,
        client_ip,
        share_name,
        actions,
    )
    .await;
    if search.is_some() {
        let matcher = SkimMatcherV2::default();
        let search = search.unwrap();
        results = results
            .iter()
            .cloned()
            .filter(|log| {
                let mut score = [0, 0];
                if let Some(scor) = matcher.fuzzy_match(log.path.as_str(), &search) {
                    score[0] = scor
                };
                match &log.path2 {
                    Some(path) => {
                        if let Some(scor) = matcher.fuzzy_match(path.as_str(), &search) {
                            score[1] = scor
                        }
                    }
                    None => (),
                }
                score.iter().any(|score| score > &30)
            })
            .collect();
    }

    Json(results)
}

#[post("/filters")]
pub async fn getfilters(_admin: Admin, conn: MyConn) -> Json<FilterData> {
    Json(get_filters(conn).await)
}

#[get("/sse")]
async fn events(events: &State<Sender<Log>>, mut end: Shutdown, _admin: Admin) -> EventStream![] {
    let mut rx = events.subscribe();

    EventStream! {
        loop {
            select! {
                changed = rx.recv() => match changed {
                    Err(err) => eprintln!("{}", err),
                    Ok(msg) => yield Event::json(&msg),
                },
                _ = &mut end => break,
            };
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct LoginForm {
    #[field(validate = len(..20))]
    pub username: String,
    #[field(validate = len(..50))]
    pub password: String,
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("session"));
    Redirect::to(uri!("/"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoggedIn {
    pub logged_in: bool,
}

#[get("/status")]
pub fn status(admin: Option<Admin>) -> Json<LoggedIn> {
    Json(LoggedIn {
        logged_in: admin.is_some(),
    })
}

#[post("/login", data = "<form>")]
pub fn login(
    env: &State<EnvLog>,
    form: Json<LoginForm>,
    admin: Option<Admin>,
    cookies: &CookieJar<'_>,
) -> rocket::http::Status {
    if admin.is_some() {
        return Status::NotAcceptable;
    }

    if form.password == env.password && form.username == env.user {
        let admin = Admin {
            login: create_from_now(),
        };

        cookies.add_private(Cookie::new("session", to_string(&admin).unwrap()));
        Status::NoContent
    } else {
        Status::Unauthorized
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let env = argus::init_env();
    let (channel, _receiver) = channel(1);

    let file = env.file.clone();

    rocket::build()
        .manage(channel.clone())
        .manage(env)
        .mount("/", FileServer::from("../frontend/build"))
        .mount(
            "/api",
            routes![login, getlog, events, logout, status, getfilters],
        )
        .attach(MyConn::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", run_migrations))
        .attach(AdHoc::try_on_ignite("Log monitor", |rocket| async move {
            let pool = MyConn::get_one(&rocket).await.unwrap();
            tokio::spawn(async move { log::monitor(file, pool, channel).await });
            Ok(rocket)
        }))
}
