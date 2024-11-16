use crate::game::*;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage, GetMessages, Message};
use serenity::builder::{
    CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::futures::StreamExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

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

pub async fn send_quiz_results_good(ctx: Context, msg: Message) {
    let footer = CreateEmbedFooter::new("Bot of '09 Developed by @anon for Class of '09").icon_url("https://images-ext-1.discordapp.net/external/AJhyvuDbJn-xEOg-8aZWAUtsg7Z9OGkWuSOminCc0hk/%3Fsize%3D256/https/cdn.discordapp.com/icons/1129581515584589914/162d2a7c4059aa53aba76bba0702788e.webp");
    let embed = CreateEmbed::new()
        .title("Quiz Results")
        .footer(footer)
        .description("# You Have Successfully Completed The Quiz!")
        .image("https://i.imgur.com/cdSerzD.png")
        .thumbnail("https://i.imgur.com/cfXguCv.png");
    let new_m = CreateMessage::new().add_embed(embed);
    msg.channel_id.send_message(&ctx.http, new_m).await.ok();
}

fn question_button(id: usize, emoji: ReactionType) -> CreateButton {
    CreateButton::new(id.to_string()).emoji(emoji)
}

pub async fn start_quiz(ctx: Context, dm_channel: PrivateChannel) {
    let questions = vec![
        Question::new(
            String::from("What's 2+2?"),
            vec![String::from("4"), String::from("2")],
            0,
        ),
        Question::new(
            String::from("Hi there"),
            vec![String::from("Hi"), String::from("Bye")],
            1,
        ),
    ];

    let mut game = Game::new(questions);
    let current_question_text = &game.questions[game.current_question_index].question;

    let m = dm_channel
        .id
        .send_message(
            &ctx,
            CreateMessage::new()
                .content(format!(
                    "{current_question_text}\nA) {}\nB) {}",
                    game.questions[game.current_question_index].answers[0],
                    game.questions[game.current_question_index].answers[1],
                ))
                .button(question_button(0, "🅰️".parse().unwrap()))
                .button(question_button(1, "🅱️".parse().unwrap())),
        )
        .await
        .unwrap();

    let mut interaction_stream = m
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let answer: usize = interaction.data.custom_id.parse().unwrap();
        let is_over = game.next_question(answer);
        if is_over == true {
            break;
        } else {
            interaction
                .create_response(
                    &ctx,
                    CreateInteractionResponse::UpdateMessage(
                        CreateInteractionResponseMessage::default().content(format!(
                            "fuck me\nA) {}\nB) {}",
                            game.questions[game.current_question_index].answers[0],
                            game.questions[game.current_question_index].answers[1],
                        )),
                    ),
                )
                .await
                .unwrap();
        }
    }
    if game.score == 2 {
        m.delete(&ctx).await.unwrap();
        // send_quiz_results_good(ctx, msg).await;
    } else {
        m.delete(&ctx).await.unwrap();
        dm_channel
            .id
            .send_message(&ctx.http, CreateMessage::new().content("You suck lol"))
            .await
            .ok();
    }
}
