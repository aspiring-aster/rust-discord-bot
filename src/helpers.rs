use serenity::all::GetMessages;
use serenity::all::Message;
use serenity::prelude::*;

pub async fn clear_messages(ctx: Context, msg: Message) {
    let messages = msg
        .channel_id
        .messages(&ctx.http, GetMessages::new())
        .await
        .unwrap();

    for message in messages {
        if message.author.id == ctx.cache.current_user().id {
            if let Err(why) = message.delete(&ctx).await {
                println!("Message delete error: {why:?}");
            }
        }
    }
}
