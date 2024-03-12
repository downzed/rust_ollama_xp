// -- modules
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::Ollama;

use simple_fs::{ensure_file_dir, save_json};
use xp_ollama::consts::MODEL;
use xp_ollama::{generator, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // By default it will connect to localhost:11434
    let ollama = Ollama::default();
    let prompts = &[
        "Write a simple function in python that kills tmux session (be concise)",
        "Write the same function from before in Rust", // "What was the color of Napoleon's black hamster? (be concise)",
                                                       // "What was my first question?",
    ];

    let mut last_ctx: Option<GenerationContext> = None;
    for p in prompts {
        println!("\n [Q]: {}", p);
        let mut gen_req = GenerationRequest::new(MODEL.to_string(), p.to_string());

        if let Some(last_ctx) = last_ctx.take() {
            gen_req = gen_req.context(last_ctx);
        }

        let final_data = generator::stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);

            // save for debug
            let ctx_file_path = ".c02-data/ctx.json";
            ensure_file_dir(ctx_file_path)?;
            save_json(ctx_file_path, &last_ctx)?;
        }
    }

    Ok(())
}
