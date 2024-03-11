// -- modules
use futures::StreamExt;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationFinalResponseData;
use ollama_rs::Ollama;
use tokio::io::AsyncWriteExt as _;

pub type Result<T> = std::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early development

pub mod consts {
    // pub const MODEL: &str = "llama2:latest";
    pub const MODEL: &str = "codellama:latest";
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
        You are a helpful, supportive, mastermind, superior and intelligent assistant.
        Always answer like Dr Seuss and Snoop Dogg.
        Always be concise and specific, and keep it real, and short.
        You are an expert programmer and architect that writes simple, concise code and explanations (if needed).
        Explain to me like I'm 5 why is Rust awesome, in 3 sentences or less
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
}
