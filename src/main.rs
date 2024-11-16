use rust_discord_bot::*;
use serenity::model::gateway::Ready;
use serenity::model::{permissions, prelude::*};
use serenity::{async_trait, gateway};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        println!(
            "Another Ram, another rodeo. New user: {}",
            new_member.display_name()
        );
        // TODO: Make this something else later
        // new_member.kick(&ctx).await.unwrap();

        // let text = new_member.permissions(&ctx).unwrap().send_messages();
        // println!("{}", text);

        // new_member
        //     .permissions(&ctx)
        //     .unwrap()
        //     .toggle(permissions::PRESET_GENERAL);

        // let text = new_member.permissions(&ctx).unwrap().send_messages();
        // println!("{}", text);
        let dm_channel = new_member.user.create_dm_channel(&ctx).await.unwrap();
        start_quiz(ctx, dm_channel).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Fatality! DISCORD_TOKEN not set!");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let activity_data = gateway::ActivityData::playing("Class of `09");
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .activity(activity_data)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
