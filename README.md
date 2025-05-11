# Chat API Server

A Rust-based API server for handling chat messages with Mistral AI integration.

## API Endpoints

### 1. Get All Chats
**GET** `/chats`

**Response:**
```json
[
  {
    "question": "What is Rust?",
    "response": "Rust is a systems programming language..."
  },
  {
    "question": "What is the capital of France?",
    "response": "The capital of France is Paris."
  }
]


2. Post a Question
POST /question

Request:

json
{
  "question": "Your question here"
}
Response:

json
{
  "question": "Your question here",
  "response": "The AI's response here"
}