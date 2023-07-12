use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // bot.send_dice(msg.chat.id).await?;
        bot.answer_dice(msg.chat.id).emoji("🎲").await?;
        Ok(())
    })
        .await;
}
