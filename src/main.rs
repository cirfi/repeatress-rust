#![allow(proc_macro_derive_resolution_fallback)]

extern crate dotenv;
extern crate tokio_core;
extern crate futures;
extern crate strfmt;
#[macro_use] extern crate serde_json;
extern crate chrono;
#[macro_use] extern crate diesel;
extern crate redis;
extern crate telegram_bot;
// telebot does not meet my requirements
// extern crate telebot;

use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
use tokio_core::reactor::Core;
use diesel::prelude::*;
// use telegram_bot::prelude::*;
use telegram_bot::{Api, GetMe, User};
use telegram_bot::types::ChatId;

mod models;
mod schema;
mod functions;
mod templates;

use models::*;

fn establish_pg_connection() -> PgConnection {
    // dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn extablish_redis_connection() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    client.get_connection().unwrap()
}

pub struct Bot<'a> {
    api: &'a Api,
    pg: &'a PgConnection,
    redis: &'a redis::Connection,
    info: User,
    config_map: HashMap<ChatId, Config>
}

fn main() {
    dotenv().ok();

    use schema::config::dsl::config;

    let pg = establish_pg_connection();

    // 加载配置
    let configs = config.load::<Config>(&pg).expect("Error loading configs");
    let mut config_map: HashMap<ChatId, Config> = HashMap::new();
    for conf in configs {
        let cid = conf.chat_id.clone();
        let chat_id = ChatId::new(cid);
        config_map.insert(chat_id, conf);
    }

    let redis = extablish_redis_connection();

    let token = env::var("TOKEN")
        .expect("TOKEN must be set");

    let mut core = Core::new().unwrap();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let get_me = api.send(GetMe);

    let info = core.run(get_me).unwrap();

    let mut bot = Bot {
        api: &api.clone(),
        pg: &pg,
        redis: &redis,
        info: info,
        config_map: config_map
    };

    bot.run(&mut core);

    // let stream = bot.get_stream().and_then(|(bot, msg)| {
    //     let mut hasher = DefaultHasher::new();
    //     text.hash(&mut hasher);
    //     let hash_result = hasher.finish();

    //     let key = cid.to_string() + "_" + &format!("{:x}", hash_result);

    //     let incr_result: i16 = redis.incr(&key, 1).unwrap();
    //     let _ : () = try!(redis.expire(&key, conf.timeout as usize));

    //     if incr_result == conf.threshold {
    //         println!("{:?}: {:?} / {:?}", &text, incr_result, conf.threshold);
    //         bot.message(cid, text).send();
    //         // let resolve_name = bot.get_me().send()
    //         //     .map(move |user| {
    //         //     if let Some(name) = user.1.username {
    //         //     }
    //         // });
    //     }

    //     Ok(())
    // });
}
