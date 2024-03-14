use futures::StreamExt;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationFinalResponseData;
use ollama_rs::Ollama;
use tokio::io::AsyncWriteExt as _;

pub type Result<T> = std::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early development

pub mod consts {
    // pub const MODEL: &str = "llama2:latest";
    pub const EMBEDDING_MODEL: &str = "nomic-embed-text";
    pub const MODEL: &str = "codellama:latest";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
        Your name is Steve!.
        You are a helpful, supportive, mastermind, superior and intelligent assistant.
        Always be concise and specific, and keep it real, and short.
        You are an expert programmer and architect that writes simple, concise code and explanations (if needed).

    "#;

    pub const MATH_ASST_SCHEMA: &str = r#"
       {
            "type": "function",
            "parameters": {
                "function_name": {
                    "type": "string",
                    "description": "The name of the function.",
                },
            },
       } 
    "#;

    pub const SUB_FUNC_SCHEMA: &str = r#"
        {
            "type": "function",
            "description": "Subtracts two numbers.",
            "parameters": {
                "a": {
                    "type": "number",        
                    "description": "The first number param."
                },
                "b": {
                    "type": "number",        
                    "description": "The second number param."
                }

            },
            "returns": {
                "type": "number",
                "description": "The difference of the two numbers."
            }
        }
    "#;

    pub const SUM_FUNC_SCHEMA: &str = r#"
        {
            "type": "function",
            "description": "Sums two numbers.",
            "parameters": {
                "a": {
                    "type": "number",        
                    "description": "The first number param."
                },
                "b": {
                    "type": "number",        
                    "description": "The second number param."
                }

            },
            "returns": {
                "type": "number",
                "description": "The sum of the two numbers."
            }
        }
    "#;

    pub const SCHEMA: &str = r#"
        {
            "city": {
                "type": "string",
                "description": "The name of the city."
            },
            "lat": {
                "type": "float",
                "description": "Decimal latitude of the city."
            },
            "long": {
                "type": "float",
                "description": "Decimal longitude of the city."
            }
        }
    "#;
}

pub mod generator {

    use super::*;

    pub async fn stream_print(
        ollama: &Ollama,
        gen_req: GenerationRequest,
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;

        while let Some(res) = stream.next().await {
            let res_list = res.map_err(|_| "stream_next error")?;

            for res in res_list {
                let bytes = res.response.as_bytes();

                // Poor man's wrapping
                char_count += bytes.len();
                if char_count > 55 {
                    char_count = 0;
                }

                // Write output
                stdout.write_all(bytes).await?;
                stdout.flush().await?;

                if let Some(final_data) = res.final_data {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    return Ok(Some(final_data));
                }
            }
        }

        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(None)
    }

    pub async fn stream_chat_print(
        ollama: &Ollama,
        chat_req: ChatMessageRequest,
    ) -> Result<Option<String>> {
        let mut stream = ollama.send_chat_messages_stream(chat_req).await?;

        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;
        let mut current_asst_msg_elems: Vec<String> = Vec::new();

        println!("\nSteve >>");
        while let Some(res) = stream.next().await {
            let res = res.map_err(|_| "stream.next error")?;
            if let Some(msg) = res.message {
                let content = msg.content;
                char_count += content.len();
                if char_count > 120 {
                    stdout.write_all(b"").await?;
                    char_count = 0;
                }
                // Write output
                stdout.write_all(content.as_bytes()).await?;
                stdout.flush().await?;

                current_asst_msg_elems.push(content);
            }

            if let Some(_final_res) = res.final_data {
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                let asst_content = current_asst_msg_elems.join("");
                return Ok(Some(asst_content));
            }
        }
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;

        Ok(None)
    }
}
