use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub question: String,
    pub response: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionRequest {
    pub question: String,
}