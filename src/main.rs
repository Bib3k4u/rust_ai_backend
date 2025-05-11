mod models;
mod db;
mod handlers;

use warp::Filter;
use mongodb::Collection;
use crate::models::ChatMessage;
use crate::handlers::{handle_get_chats, handle_post_question, with_collection};
use crate::db::connect_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mongo_client = connect_db().await?;
    let collection: Collection<ChatMessage> = mongo_client.database("ChatDB").collection("chat_messages");

    // Simple CORS configuration that works with warp 0.3
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_header("content-type");

    // GET /chats route
    let get_chats = warp::get()
        .and(warp::path("chats"))
        .and(with_collection(collection.clone()))
        .and_then(handle_get_chats);

    // POST /question route
    let post_question = warp::post()
        .and(warp::path("question"))
        .and(warp::body::json())
        .and(with_collection(collection.clone()))
        .and_then(handle_post_question);

    // Combine routes with CORS
    let routes = get_chats
        .or(post_question)
        .with(cors);

    println!("Server running on http://localhost:3030");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;

    Ok(())
}