// Copyright (C) 2021-2021 Fuwn
// SPDX-License-Identifier: GPL-3.0-only

use serenity::{
  framework::standard::{macros::command, CommandResult},
  model::prelude::*,
  prelude::*,
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
