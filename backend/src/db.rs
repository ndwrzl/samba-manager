use crate::diesel::BoolExpressionMethods;
use crate::diesel::ExpressionMethods;
use crate::models::{Log, NewLog};
use crate::schema::logs::dsl::*;
use crate::MyConn;
use diesel::insert_into;
use diesel::{QueryDsl, RunQueryDsl};
use lazy_static::lazy_static;
use regex::Regex;
use rocket::serde::Serialize;

const PARSE_STRING: &str = r"(?x)
    ^(?P<date>\d+)\s\[SMB\sAUDIT]\s
    (?P<server_user>\w+)\|
    (?P<client_ip>.+?)\|
    (?P<client_name>.+?)\|
    (?P<share_name>.+?)\|
    (?P<action>.+?)\|
    ((?P<ok>ok)\|)?
    ((?P<permissions>0x\w+)\|)?
    ((?P<extra>[^/]+?)\|)?
    (?P<path>(/[^\|]+))
    (\|(?P<path2>(/[^\|]+)))?
    $
";

macro_rules! unwrap_or_err {
    ( $e:expr ) => {
        match $e {
            Some(e) => e,
            None => return Err("No group".to_owned()),
        }
    };
}

pub fn parse_line(line: &String) -> Result<NewLog, String> {
    lazy_static! {
        pub static ref PARSE_REGEX: Regex = regex::Regex::new(PARSE_STRING).unwrap();
    }
    let parsed = match PARSE_REGEX.captures(&line) {
        Some(parsed) => parsed,
        None => return Err("Regex doesn't match".to_owned()),
    };

    let get_str = |name: &'static str| match parsed.name(name) {
        None => None,
        Some(reg) => Some(reg.as_str().to_owned()),
    };

    Ok(NewLog {
        date: unwrap_or_err!(get_str("date")).parse::<i64>().unwrap(),
        server_user: unwrap_or_err!(get_str("server_user")),
        client_ip: unwrap_or_err!(get_str("client_ip")),
        client_name: unwrap_or_err!(get_str("client_name")),
        share_name: unwrap_or_err!(get_str("share_name")),
        action: unwrap_or_err!(get_str("action")),
        ok: match get_str("ok") {
            Some(reg) => Some(reg == "ok"),
            None => None,
        },
        permissions: match get_str("permissions") {
            Some(reg) => match i32::from_str_radix(&reg[2..], 16) {
                Ok(e) => Some(e),
                Err(err) => return Err(err.to_string()),
            },
            None => None,
        },
        data: get_str("extra"),
        path: unwrap_or_err!(get_str("path")),
        path2: get_str("path2"),
    })
}

#[derive(Debug)]
pub enum ReadLineError {
    LogExists(String),
    ParseError(String, String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FilterData {
    pub oldest: i64,
    pub share_name: Vec<String>,
    pub client_name: Vec<String>,
    pub client_ip: Vec<String>,
    pub actions: Vec<String>,
}

pub async fn get_filters(conn: MyConn) -> FilterData {
    conn.run(move |c| {
        let query = logs
            .select((client_name, action, client_ip, share_name))
            .distinct()
            .load::<(String, String, String, String)>(c)
            .unwrap();
        let oldest = logs.order(date).select(date).first(c).unwrap();

        FilterData {
            oldest,
            client_name: query.iter().map(|x| x.0.clone()).collect(),
            actions: query.iter().map(|x| x.1.clone()).collect(),
            client_ip: query.iter().map(|x| x.2.clone()).collect(),
            share_name: query.iter().map(|x| x.3.clone()).collect(),
        }
    })
    .await
}

pub async fn get_logs(
    conn: MyConn,
    count: i64,
    before: Option<i64>,
    after: Option<i64>,
    client: Option<String>,
    share: Option<String>,
    actions: Option<String>,
) -> Vec<Log> {
    conn.run(move |c| {
        let mut query = logs.order(id.desc()).limit(count).into_boxed();

        if before.is_some() {
            query = query.filter(date.le(before.unwrap()));
        }

        if after.is_some() {
            query = query.filter(date.ge(after.unwrap()));
        }

        if client.is_some() {
            query = query.filter(client_name.eq(client.unwrap()));
        }

        if share.is_some() {
            query = query.filter(share_name.eq(share.unwrap()));
        }

        if actions.is_some() {
            query = query.filter(action.eq(actions.unwrap()));
        }

        query.load::<Log>(c).expect("Error loading logs")
    })
    .await
}

pub async fn add_line(conn: &MyConn, line: String) -> Result<Log, ReadLineError> {
    let parsed = match parse_line(&line) {
        Err(context) => return Err(ReadLineError::ParseError(line, context)),
        Ok(line) => line,
    };

    let (first, parsed) = conn
        .run(move |c| {
            // Check if data already exists
            (
                // commented out filters have issues
                logs.filter(
                    date.eq(&parsed.date)
                        .and(server_user.eq(&parsed.server_user))
                        .and(client_ip.eq(&parsed.client_ip))
                        .and(path.eq(&parsed.path))
                        // .and(path2.eq(&parsed.path2))
                        .and(client_name.eq(&parsed.client_name))
                        .and(share_name.eq(&parsed.share_name))
                        .and(action.eq(&parsed.action)), // .and(ok.eq(&parsed.ok))
                                                         // .and(permissions.eq(&parsed.permissions))
                                                         // .and(data.eq(&parsed.data)),
                )
                .get_results::<Log>(c),
                parsed,
            )
        })
        .await;

    let found = match first {
        Ok(log) => log
            .into_iter()
            .map(|log| {
                &log.path2 == &parsed.path2
                    || &log.ok == &parsed.ok
                    || &log.permissions == &parsed.permissions
                    || &log.data == &parsed.data
            })
            .any(|f| f == true),
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    };

    match found {
        true => Err(ReadLineError::LogExists(line)),
        false => {
            // insert data and return
            println!("insert");
            conn.run(move |c| {
                let rows_inserted = insert_into(logs).values(&parsed).execute(c);
                assert_eq!(Ok(1), rows_inserted);
                let inserted = logs.order(id.desc()).first(c).unwrap();

                Ok(inserted)
            })
            .await
        }
    }
}
