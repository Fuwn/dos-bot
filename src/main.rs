// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

#![feature(
  type_ascription,
  hash_set_entry,
  type_name_of_val,
  decl_macro,
  proc_macro_hygiene
)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code
)]
#![deny(clippy::all, clippy::nursery, clippy::pedantic)]
#![recursion_limit = "128"]
#![doc(
  html_logo_url = "https://cdn.discordapp.com/icons/854071194453671976/bc8c80e4bcfa66ecd55da8ceaa80\
  f5a8.webp?size=128",
  html_favicon_url = "https://cdn.discordapp.com/icons/854071194453671976/bc8c80e4bcfa66ecd55da8cea\
  a80f5a8.webp?size=128"
)]

pub mod commands;
pub mod config;
pub mod database;

#[macro_use]
extern crate log;

use std::{collections::HashSet, sync::Arc};

#[allow(clippy::wildcard_imports)]
use commands::*;
use serenity::{
  async_trait,
  client::bridge::gateway::{GatewayIntents, ShardManager},
  framework::{standard::macros::group, StandardFramework},
  http::Http,
  model::{
    channel::Reaction,
    gateway::{Activity, Ready},
  },
  prelude::*,
};

use crate::config::Config;

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
  async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
    if let Some(guild) = reaction.guild_id {
      if let Ok(role) = database::Database::new().get_reaction_role(reaction.message_id.0) {
        guild
          .member(&ctx, reaction.user_id.expect("unable to locate user id"))
          .await
          .expect("unable to locate member")
          .add_role(&ctx, role)
          .await
          .expect("unable to add role to member");
      }
    }
  }

  async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
    if let Some(guild) = reaction.guild_id {
      if let Ok(role) = database::Database::new().get_reaction_role(reaction.message_id.0) {
        guild
          .member(&ctx, reaction.user_id.expect("unable to locate user id"))
          .await
          .expect("unable to locate member")
          .remove_role(&ctx, role)
          .await
          .expect("unable to add role to member");
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    info!("connected to discord gateway as {}", ready.user.name);

    ctx
      .set_activity(Activity::watching(">help - discord.io/assembly"))
      .await;
  }
}

#[group]
#[commands(ping, help, say)]
struct General;

#[group]
#[commands(create, remove, count)]
struct RoleReactions;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  std::env::set_var("RUST_LOG", "dos_bot=info");
  pretty_env_logger::init();

  let http = Http::new_with_token(Config::get().token.as_str());

  let (owners, _bot_id) = match http.get_current_application_info().await {
    Ok(info) => {
      let mut owners = HashSet::new();
      owners.insert(info.owner.id);

      (owners, info.id)
    }
    Err(why) => panic!("could not access application info: {:?}", why),
  };

  let framework = StandardFramework::new()
    .configure(|c| c.owners(owners).prefix(">"))
    .group(&GENERAL_GROUP)
    .group(&ROLEREACTIONS_GROUP);

  let mut client = Client::builder(Config::get().token.as_str())
    .framework(framework)
    .event_handler(Handler)
    .intents(GatewayIntents::all())
    .await
    .expect("error creating dos-bot");

  {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
  }

  let shard_manager = client.shard_manager.clone();

  tokio::spawn(async move {
    tokio::signal::ctrl_c()
      .await
      .expect("could not register ctrl+c handler");
    shard_manager.lock().await.shutdown_all().await;
  });

  if let Err(why) = client.start().await {
    error!("client error: {:?}", why);
  }
}
