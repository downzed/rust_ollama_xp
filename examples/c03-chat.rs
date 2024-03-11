use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    Ollama,
};

use xp_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    generator::stream_chat_print,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let prompts = &[
        "What was the color of Napoleon's black hamster?",
        "What is the best programming language? (be short!), (it's rust btw)",
        "What was my last question?",
    ];

    let system_msg = ChatMessage::new(MessageRole::System, DEFAULT_SYSTEM_MOCK.to_string());

    let mut thread_messages: Vec<ChatMessage> = vec![system_msg];

    for p in prompts {
        println!("\n [Prompt]: {p}");
        let prompt_msg = ChatMessage::new(MessageRole::User, p.to_string());

        thread_messages.push(prompt_msg);

        let chat_req = ChatMessageRequest::new(MODEL.to_string(), thread_messages.clone());

        let msg_content = stream_chat_print(&ollama, chat_req).await?;

        if let Some(content) = msg_content {
            let asst_msg = ChatMessage::new(MessageRole::Assistant, content);
            thread_messages.push(asst_msg);
        }
    }

    Ok(())
}
