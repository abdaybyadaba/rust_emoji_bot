use teloxide::
{
    payloads::SetMessageReactionSetters,
    types::{ReactionType::Emoji, User},
    prelude::*
};

use std::error::Error;
use std::collections::HashMap;
use std::sync::Arc;

mod emoji;
use emoji::EMOJIES;

pub fn get_emoji(user: Option<User>, map: Arc<HashMap<UserId, EMOJIES>>) -> Option<String> {
    match user {
        Some(us) => {
            if let Some(set) = map.get(&us.id) { return Some(set.give_emoji()); }
        } None => ()
    } None
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    map: Arc<HashMap<UserId, EMOJIES>>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(given_emoji) = get_emoji(msg.from, map) {
        bot.set_message_reaction(msg.chat.id, msg.id)
            .reaction( Vec::from( [Emoji{ emoji: given_emoji }] )).await?;
    } Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot = Bot::from_env();
    let mut map: HashMap<UserId, EMOJIES> = HashMap::new(); 
    //map.insert(UserId(val), EMOJIES::Better);
    

    let map = Arc::new(map);
    
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));
        
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![Arc::clone(&map)])
        .enable_ctrlc_handler().build().dispatch().await;
    Ok(())
}


