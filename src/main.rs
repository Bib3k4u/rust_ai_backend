mod models;
mod db;
mod handlers;

use models::{ChatMessage, QuestionRequest};
use db::connect_db;
use handlers::{handle_get_chats, handle_post_question, with_collection};
use mongodb::Collection;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_client = connect_db().await?;
    let collection: Collection<ChatMessage> = mongo_client.database("ChatDB").collection("chat_messages");

    let get_chats = warp::get()
        .and(warp::path("chats"))
        .and(with_collection(collection.clone()))
        .and_then(handle_get_chats);

    let post_question = warp::post()
        .and(warp::path("question"))
        .and(warp::body::json::<QuestionRequest>())
        .and(with_collection(collection.clone()))
        .and_then(handle_post_question);

    let routes = get_chats.or(post_question);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}