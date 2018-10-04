use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use tokio_core::reactor::Handle;
use tokio_core::reactor::Core;
use futures::{Stream};
use strfmt::strfmt;
use redis::Commands;
use diesel;
use diesel::prelude::*;
use telegram_bot::prelude::*;
use telegram_bot::{Message, MessageKind};
use telegram_bot::types::*;

use super::Bot;
use models::*;
use templates::get_template;

impl<'a> Bot<'a> {
    pub fn run(self: &'a mut Self, core: &mut Core) {
        let handle = core.handle();

        let future = self.api.stream().for_each(|update| {
            if let UpdateKind::Message(message) = update.kind {
                self.handle_message(message, &handle)
            }
            Ok(())
        });

        core.run(future).unwrap();
    }

    fn handle_message(self: &'_ mut Self, message: Message, handle: &Handle) {
        let msg = message.clone();
        let kind = &msg.kind;
        match kind {
            MessageKind::Text { ref data, ..} => {
                if !self.filter_command(data) {
                    return;
                }

                let mut full_command_split = data.split_whitespace();
                let command = if data.starts_with("/") {
                    full_command_split.next().unwrap().split("@").next().unwrap()
                } else {
                    ""
                };

                match command {
                    "/status" => self.handle_under_developing(message, handle),
                    "/timeout" => self.handle_set_timeout(message, full_command_split.next(), handle),
                    "/threshold" => self.handle_set_threshold(message, full_command_split.next(), handle),
                    "/timezone" => self.handle_under_developing(message, handle),
                    "/today" => self.handle_under_developing(message, handle),
                    "/recently" => self.handle_under_developing(message, handle),
                    "/day" => self.handle_under_developing(message, handle),
                    "/interval" => self.handle_under_developing(message, handle),
                    "/anchor" => self.handle_under_developing(message, handle),
                    "/forward" => self.handle_under_developing(message, handle),
                    _ => self.handle_forward(message, handle),
                }
            }
            _ => self.handle_forward(message, handle)
        };
    }

    fn handle_under_developing(self: &Self, message: Message, _handle: &Handle) {
        self.api.spawn(message.chat.text("项目重构中，该功能尚未完成。"));
    }

    fn handle_set_timeout(self: &'_ mut Self, message: Message, timeout: Option<&str>, handle: &Handle) {
        if timeout.is_none() || timeout.unwrap().trim().is_empty() {
            self.api.spawn(message.text_reply("缺少参数"));
            return;
        }

        let chat_id = message.chat.id();

        let timeout = timeout.unwrap().parse::<i16>().unwrap();

        let conf = match self.get_config(&message.chat, handle) {
            Some(config) => config,
            None => return,
        };

        if conf.timeout != timeout {
            use schema::config::dsl;

            let new_conf = diesel::update(dsl::config.find(conf.id))
                .set(dsl::timeout.eq(timeout))
                .get_result::<Config>(self.pg)
                .expect(&format!("Unable to find config {}", conf.id));

            self.config_map.insert(chat_id, new_conf);

            let lang = conf.lang.as_str();
            let template = get_template(lang).timeout.to_string();

            let mut vars = HashMap::new();
            vars.insert("timeout".to_string(), timeout.to_string());

            self.api.spawn(message.text_reply(strfmt(&template, &vars).unwrap()));
        }
    }

    fn handle_set_threshold(self: &'_ mut Self, message: Message, threshold: Option<&str>, handle: &Handle) {
        if threshold.is_none() || threshold.unwrap().trim().is_empty() {
            self.api.spawn(message.text_reply("缺少参数"));
            return;
        }

        let chat_id = message.chat.id();

        let threshold = threshold.unwrap().parse::<i16>().unwrap();
        
        let conf = match self.get_config(&message.chat, handle) {
            Some(config) => config,
            None => return,
        };

        if conf.threshold != threshold {
            use schema::config::dsl;

            let new_conf = diesel::update(dsl::config.find(conf.id))
                .set(dsl::threshold.eq(threshold))
                .get_result::<Config>(self.pg)
                .expect(&format!("Unable to find config {}", conf.id));

            self.config_map.insert(chat_id, new_conf);

            self.api.spawn(message.text_reply(format!("The threshold is set to {}", threshold)));
        }
    }

    fn handle_forward(self: &'_ mut Self, message: Message, handle: &Handle) {
        let config: Config = match self.get_config_mut(&message.chat, handle) {
            Some(conf) => conf,
            None => return,
        };

        let chat_id = config.chat_id.clone();

        let key: String = match message.kind {
            MessageKind::Text {ref data, ..} => {
                let mut hasher = DefaultHasher::new();
                data.hash(&mut hasher);
                let hash_result = hasher.finish();

                chat_id.to_string() + "_" + &format!("{:x}", hash_result)
            },
            _ => return,
        };

        let timeout = config.timeout as usize;
        let threshold = config.threshold;

        let incr_result: i16 = self.redis.incr(&key, 1).unwrap();
        let _ : () = self.redis.expire(&key, timeout).unwrap();

        if incr_result == threshold {
            self.api.spawn(message.forward(&message.chat));
        }
    }

    fn get_config(self: &Self, chat: &MessageChat, _handle: &Handle) -> Option<Config> {
        let cid: i64;
        let cname: &str;
        let username: &str;

        match chat {
            MessageChat::Private(user) => {
                cid = user.id.into();
                cname = user.first_name.as_str();
                username = match &user.username {
                    Some(name) => name.as_str(),
                    None => "",
                }
            },
            MessageChat::Group(group) => {
                cid = group.id.into();
                cname = group.title.as_str();
                username =  "";
            },
            MessageChat::Supergroup(group) => {
                cid = group.id.into();
                cname = group.title.as_str();
                username = match &group.username {
                    Some(name) => name.as_str(),
                    None => "",
                }
            },
            _ => {
                cid = 0;
                cname = "";
                username = "";
            }
        }

        let key = ChatId::new(cid);

        if self.config_map.contains_key(&key) {
            let result = match self.config_map.get(&key) {
                Some(conf) => Some(conf.clone()),
                None => None,
            };

            return result;
        } else if cid != 0 {
            use schema::config::dsl::*;

            let temp = NewConfig {
                chat_id: &cid,
                chat_name: cname,
                chat_username: username,
                ..Default::default()
            };

            let result: Config = diesel::insert_into(config)
                .values(&temp)
                .on_conflict_do_nothing()
                .get_result(self.pg)
                .expect("Error saving new config");

            return Some(result);
        }

        None
    }

    fn get_config_mut(self: &'_ mut Self, chat: &MessageChat, _handle: &Handle) -> Option<Config> {
        let cid: i64;
        let cname: &str;
        let username: &str;

        match chat {
            MessageChat::Private(user) => {
                cid = user.id.into();
                cname = user.first_name.as_str();
                username = match &user.username {
                    Some(name) => name.as_str(),
                    None => "",
                }
            },
            MessageChat::Group(group) => {
                cid = group.id.into();
                cname = group.title.as_str();
                username =  "";
            },
            MessageChat::Supergroup(group) => {
                cid = group.id.into();
                cname = group.title.as_str();
                username = match &group.username {
                    Some(name) => name.as_str(),
                    None => "",
                }
            },
            _ => {
                cid = 0;
                cname = "";
                username = "";
            }
        }

        let key = ChatId::new(cid);

        let result = if self.config_map.contains_key(&key) {
            match self.config_map.get(&key) {
                Some(conf) => Some(conf.clone()),
                None => None,
            }
        } else if cid != 0 {
            use schema::config::dsl::*;

            let temp = NewConfig {
                chat_id: &cid,
                chat_name: cname,
                chat_username: username,
                ..Default::default()
            };

            let result: Config = diesel::insert_into(config)
                .values(&temp)
                .on_conflict_do_nothing()
                .get_result(self.pg)
                .expect("Error saving new config");

            self.config_map.insert(key, result.clone());

            return Some(result);
        } else {
            None
        };

        result
    }

    // filter commands that @ another bot
    fn filter_command(self: &Self, text: &str) -> bool {
        if !text.starts_with("/") {
            return true;
        }

        let info = &self.info;

        let mut full_command_split = text.split_whitespace();

        let command = full_command_split.next().unwrap();

        let mut result = true;

        if command.contains("@") && !command.ends_with("@") {
            match info.username.clone() {
                Some(username) => {
                    let command_split = command.split("@");
                    let at = command_split.last().unwrap();

                    if username != at {
                        result = false;
                    }
                },
                _ => ()
            }
        }

        result
    }
}