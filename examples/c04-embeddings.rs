use std::{fs, path::Path};

use simple_fs::{ensure_dir, load_be_f64, read_to_string, save_be_f32, save_be_f64, save_json};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use xp_ollama::{consts::EMBEDDING_MODEL, Result};

use ollama_rs::Ollama;

const MOCK_DIR: &str = "_mock-data";
const C04_DIR: &str = ".c04-data";

#[tokio::main]
async fn main() -> Result<()> {
    read_embeddings().await?;
    // create_embeddings(&Ollama::default()).await?;
    Ok(())
}

async fn parse_and_print_embeddings(file_path: &Path) -> Result<()> {
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();

    // Read the entire file into a buffer
    file.read_to_end(&mut buffer).await?;

    // The first 8 bytes are the index
    let (index_bytes, embeddings_bytes) = buffer.split_at(std::mem::size_of::<f64>());
    let index = f64::from_be_bytes(
        index_bytes
            .try_into()
            .expect("Index slice with incorrect length"),
    );

    // Each subsequent 8-byte chunk is an embedding
    let embeddings = embeddings_bytes
        .chunks(std::mem::size_of::<f64>())
        .map(|bytes| {
            f64::from_be_bytes(
                bytes
                    .try_into()
                    .expect("Embedding slice with incorrect length"),
            )
        });

    // Print index and embeddings
    for embedding in embeddings {
        println!("{{index: {}, embedding: {}}}", index, embedding);
    }

    Ok(())
}

async fn read_embeddings() -> Result<()> {
    ensure_dir(C04_DIR)?;
    let path = Path::new(C04_DIR).join("c04-embeddings-01.be-f64.bin");
    parse_and_print_embeddings(&path).await?;
    // let d = File::read_f64(Path::new(C04_DIR).join("c04-embeddings-04.be-f64.bin")).await?;
    // println!(">> [debug: embeddings size] {}", d.ln());
    // println!(">> [debug: embeddings] {:?}", d);

    Ok(())
}

async fn create_embedding(ollama: &Ollama, seg: &str) -> Result<Vec<f64>> {
    let res = ollama
        .generate_embeddings(EMBEDDING_MODEL.to_string(), String::from(seg), None)
        .await?;

    Ok(res.embeddings)
}

async fn create_embeddings(ollama: &Ollama) -> Result<()> {
    ensure_dir(C04_DIR)?;

    let txt = read_to_string(Path::new(MOCK_DIR).join("embeddings.txt"))?;
    let chunks: Vec<&str> = txt.split("\n## ").collect();

    for (i, chunk) in chunks.iter().enumerate() {
        let emb = create_embedding(ollama, chunk.trim()).await?;
        let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        save_be_f64(&file_path, &emb)?;
    }

    Ok(())
}
