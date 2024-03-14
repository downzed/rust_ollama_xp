use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

use xp_ollama::consts::{DEFAULT_SYSTEM_MOCK, MODEL};
use xp_ollama::{generator, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (be concise)".to_string();

    let gen_req = GenerationRequest::new(model, prompt).system(DEFAULT_SYSTEM_MOCK.to_string());

    // -- single response generation
    // let res = ollama.generate(gen_req).await?;
    // println!("llama [A] {}", res.response);

    // -- stream response generation
    generator::stream_print(&ollama, gen_req).await?;

    Ok(())
}
