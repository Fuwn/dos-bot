// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use array_tool::vec::Intersect;
use serenity::{
  framework::standard::{macros::command, Args, CommandResult},
  model::prelude::*,
  prelude::*,
  utils::{content_safe, ContentSafeOptions},
};

fn num_to_emoji(num: i32) -> String {
  // Naive
  num.to_string() + "️⃣"
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  msg.channel_id.say(&ctx.http, "Pong!").await?;

  Ok(())
}

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
  msg
    .channel_id
    .say(
      &ctx.http,
      r#"```
  commands
  ========

help
  you are here

ping
  pong!


create <message_id> <role_id>
  create a role reaction

remove <message_id>
  remove a role reaction

count
  count the role reactions


  information
  ===========

- https://github.com/fuwn/dos-bot
- https://discord.io/assembly
```"#,
    )
    .await?;

  Ok(())
}

#[command]
pub async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let settings = msg.guild_id.map_or_else(
    || {
      ContentSafeOptions::default()
        .clean_channel(false)
        .clean_role(false)
    },
    |guild_id| {
      ContentSafeOptions::default()
        .clean_channel(false)
        .display_as_member_from(guild_id)
    },
  );

  let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
  msg.delete(&ctx.http).await?;

  msg.channel_id.say(&ctx.http, &content).await?;

  Ok(())
}

#[command]
pub async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if msg
    .guild(&ctx.cache)
    .await
    .unwrap()
    .member(&ctx.http, msg.author.id)
    .await?
    .roles
    .intersect(
      crate::config::Config::get()
        .admin_roles
        .iter()
        .map(|r| RoleId(*r))
        .collect(),
    )
    .is_empty()
  {
    msg
      .channel_id
      .say(&ctx.http, "invalid permissions!")
      .await?;

    return Ok(());
  }

  crate::database::Database::new()
    .create_reaction_role(args.single::<u64>()?, args.single::<u64>()?);

  msg
    .channel_id
    .say(&ctx.http, "created created role reaction")
    .await?;

  Ok(())
}

#[command]
pub async fn remove(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
  if msg
    .guild(&ctx.cache)
    .await
    .unwrap()
    .member(&ctx.http, msg.author.id)
    .await?
    .roles
    .intersect(
      crate::config::Config::get()
        .admin_roles
        .iter()
        .map(|r| RoleId(*r))
        .collect(),
    )
    .is_empty()
  {
    msg
      .channel_id
      .say(&ctx.http, "invalid permissions!")
      .await?;

    return Ok(());
  }

  crate::database::Database::new().remove_reaction_role(args.single::<u64>()?);

  msg
    .channel_id
    .say(&ctx.http, "removed role reaction")
    .await?;

  Ok(())
}

#[command]
pub async fn count(ctx: &Context, msg: &Message) -> CommandResult {
  if msg
    .guild(&ctx.cache)
    .await
    .unwrap()
    .member(&ctx.http, msg.author.id)
    .await?
    .roles
    .intersect(
      crate::config::Config::get()
        .admin_roles
        .iter()
        .map(|r| RoleId(*r))
        .collect(),
    )
    .is_empty()
  {
    msg
      .channel_id
      .say(&ctx.http, "invalid permissions!")
      .await?;

    return Ok(());
  }

  msg
    .channel_id
    .say(&ctx.http, crate::database::Database::new().count())
    .await?;

  Ok(())
}

#[command]
pub async fn poll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let message = format!("__Poll created by {}__\n\n{}", msg.author.name, args.rest());

  // let ops = args.rest().split(" ");
  // let mut tick = 1;
  // for op in ops {
  //   message += &(num_to_emoji(tick) + " " + op + "\n");
  //   tick += 1;
  // }

  let sent = msg.channel_id.say(&ctx.http, message).await?;
  for i in 1..10 {
    sent
      .react(&ctx.http, ReactionType::Unicode(num_to_emoji(i)))
      .await?;
  }

  Ok(())
}
