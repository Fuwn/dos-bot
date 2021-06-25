// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use serenity::{
  framework::standard::{macros::command, Args, CommandResult},
  model::prelude::*,
  prelude::*,
  utils::{content_safe, ContentSafeOptions},
};

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

help - you are here
ping - pong!

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
