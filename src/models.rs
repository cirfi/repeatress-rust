use chrono;
use serde_json;
use super::schema::*;

#[derive(Debug, Queryable, Clone)]
pub struct Config {
    pub id: i32,
    pub chat_id: i64,
    pub chat_name: String,
    pub chat_username: String,
    pub threshold: i16,
    pub timeout: i16,
    pub timezone: i16,
    pub lang: String
}

#[derive(Insertable)]
#[table_name="config"]
pub struct NewConfig<'a> {
    pub chat_id: &'a i64,
    pub chat_name: &'a str,
    pub chat_username: &'a str,
    pub threshold: &'a i16,
    pub timeout: &'a i16,
    pub timezone: &'a i16,
    pub lang: &'a str
}

impl <'a> Default for NewConfig<'a> {
    fn default() -> NewConfig<'a> {
        NewConfig {
            chat_id: &0,
            chat_name: "",
            chat_username: "",
            threshold: &3,
            timeout: &30,
            timezone: &0,
            lang: "zh",
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Message {
    pub id: i32,
    pub chat_id: i64,
    pub fwd_msg_id: i64,
    pub msg_id: i64,
    pub content: String,
    pub create_time: chrono::NaiveDateTime
}

impl Default for Message {
    fn default() -> Message {
        Message {
            id: 0,
            chat_id: 0,
            fwd_msg_id: 0,
            msg_id: 0,
            content: "".to_string(),
            create_time: chrono::Utc::now().naive_utc()
        }
    }
}

#[derive(Debug, Queryable)]
pub struct Record {
    pub id: i32,
    pub chat_id: i64,
    pub msg_id: i64,
    pub msg_ids: serde_json::Value,
    pub create_time: chrono::NaiveDateTime
}

impl Default for Record {
    fn default() -> Record {
        Record {
            id: 0,
            chat_id: 0,
            msg_id: 0,
            msg_ids: json!(0),
            create_time: chrono::Utc::now().naive_utc()
        }
    }
}