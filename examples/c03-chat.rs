use std::process;

use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, MessageRole},
    Ollama,
};
use tokio::io::{self as tokio_io, AsyncBufReadExt, AsyncWriteExt, BufReader}; // Import for async read

use xp_ollama::{
    consts::{DEFAULT_SYSTEM_MOCK, MODEL},
    generator::stream_chat_print,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    let system_msg = ChatMessage::new(MessageRole::System, DEFAULT_SYSTEM_MOCK.to_string());

    // Convert the iterator to an asynchronous stream
    let stdin = tokio_io::stdin();
    let mut stdout = tokio_io::stdout();

    let mut thread_messages: Vec<ChatMessage> = vec![system_msg];

    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    print_ascii();

    println!();
    println!("Ask away >> ");

    while let Some(line) = lines.next_line().await? {
        stdout.flush().await?;

        let prompt = line.trim().to_string();

        if prompt == "exit" {
            println!("Exiting...");
            process::exit(0);
        }

        let prompt_msg = ChatMessage::new(MessageRole::User, prompt);

        thread_messages.push(prompt_msg);

        let chat_req = ChatMessageRequest::new(MODEL.to_string(), thread_messages.clone());

        let msg_content = stream_chat_print(&ollama, chat_req).await?;

        if let Some(content) = msg_content {
            let asst_msg = ChatMessage::new(MessageRole::Assistant, content.clone());
            thread_messages.push(asst_msg);
        }
        println!();
        println!("Ask away >> ");
    }
    Ok(())
}

fn print_ascii() {
    println!(
        r#"  
  ___ _               
 / __| |_ _____ _____ 
 \__ \  _/ -_) V / -_)
 |___/\__\___|\_/\___|
                      "#
    );
    println!("\n High! Type the magic word to end the conversation.\n");
}
