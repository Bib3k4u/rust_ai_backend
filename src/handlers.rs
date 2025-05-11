use warp::Filter;
use mongodb::Collection;
use std::convert::Infallible;
use futures_util::stream::TryStreamExt;
use crate::models::{ChatMessage, QuestionRequest, ChatCompletionResponse};

pub async fn handle_get_chats(collection: Collection<ChatMessage>) -> Result<impl warp::Reply, Infallible> {
    let chats = collection.find(None, None).await.unwrap();
    let chats: Vec<ChatMessage> = chats.try_collect().await.unwrap();
    Ok(warp::reply::json(&chats))
}

pub async fn handle_post_question(
    question_request: QuestionRequest,
    collection: Collection<ChatMessage>
) -> Result<impl warp::Reply, Infallible> {
    let http_client = reqwest::Client::new();
    let api_key = "UOO9dfJ3IYbVMivpgE7heN8uXOAkR3VN";

    let request_body = serde_json::json!({
        "model": "mistral-tiny",
        "messages": [{"role": "user", "content": question_request.question}]
    });

    match http_client.post("https://api.mistral.ai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(response_text) => {
                        match serde_json::from_str::<ChatCompletionResponse>(&response_text) {
                            Ok(chat_completion) => {
                                if let Some(choice) = chat_completion.choices.first() {
                                    // Create the message we'll return first
                                    let response_message = ChatMessage {
                                        question: question_request.question,
                                        response: choice.message.content.trim().to_string(),
                                    };

                                    // Clone it for insertion
                                    collection.insert_one(response_message.clone(), None).await.unwrap();
                                    Ok(warp::reply::json(&response_message))
                                } else {
                                    Ok(warp::reply::json(&"No response from AI"))
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to parse response: {}", e);
                                Ok(warp::reply::json(&"Failed to parse response from AI"))
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read response: {}", e);
                        Ok(warp::reply::json(&"Failed to read response from AI"))
                    }
                }
            } else {
                eprintln!("API request failed with status code: {}", response.status());
                match response.text().await {
                    Ok(error_details) => {
                        eprintln!("Error details: {}", error_details);
                        Ok(warp::reply::json(&"Failed to get response from AI"))
                    },
                    Err(e) => {
                        eprintln!("Failed to read error details: {}", e);
                        Ok(warp::reply::json(&"Failed to read error details from AI"))
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to send request: {}", e);
            Ok(warp::reply::json(&"Failed to send request to AI"))
        }
    }
}

pub fn with_collection(
    collection: Collection<ChatMessage>
) -> impl Filter<Extract = (Collection<ChatMessage>,), Error = Infallible> + Clone {
    warp::any().map(move || collection.clone())
}