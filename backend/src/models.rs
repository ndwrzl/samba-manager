use crate::schema::logs;
use diesel::{Insertable, Queryable};
use rocket::serde::Serialize;

#[derive(Queryable, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Log {
    pub id: i32,
    pub date: i64,
    pub server_user: String,
    pub client_ip: String,
    pub client_name: String,
    pub share_name: String,
    pub action: String,
    pub ok: Option<bool>,
    pub permissions: Option<i32>,
    pub data: Option<String>,
    pub path: String,
    pub path2: Option<String>,
}

#[derive(Insertable, Serialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "logs"]
pub struct NewLog {
    pub date: i64,
    pub server_user: String,
    pub client_ip: String,
    pub client_name: String,
    pub share_name: String,
    pub action: String,
    pub ok: Option<bool>,
    pub permissions: Option<i32>,
    pub data: Option<String>,
    pub path: String,
    pub path2: Option<String>,
}
