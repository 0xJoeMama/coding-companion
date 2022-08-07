use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serenity::{
    model::{gateway::ActivityType, id::ChannelId},
    prelude::TypeMapKey,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub activity: ActivityType,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub prefix: String,
    pub status: Status,
    pub reaction_roles: HashMap<String, String>,
    pub reaction_role_channel: ChannelId,
    pub thread_channels: HashSet<String>,
    pub thread_creation_message: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, std::io::Error> {
        let config_path = path.to_owned();
        let file = std::fs::read_to_string(config_path)?;

        let config = serde_json::from_str(&file)?;
        Ok(config)
    }

    pub fn get_role<'a>(&'a self, reaction: &str) -> Option<&'a String> {
        self.reaction_roles.get(reaction)
    }
}

impl TypeMapKey for Config {
    type Value = Config;
}
