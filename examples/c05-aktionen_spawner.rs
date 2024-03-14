use std::process;

use futures::StreamExt as _;
use ollama_rs::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
        completion::request::GenerationRequest,
        options::GenerationOptions,
        parameters::FormatType,
    },
    Ollama,
};
use tokio::{fs::File, io::AsyncWriteExt}; // Import for async read

use xp_ollama::{
    consts::{FUNC_SCHEMA, MATH_ASST_SCHEMA, MODEL, SCHEMA, SUB_FUNC_SCHEMA, SUM_FUNC_SCHEMA},
    Result,
};

async fn create_a_modelfile(messages: &[ChatMessage]) -> Result<()> {
    let content = &messages.last().unwrap().content;
    let mut file = File::create("Modelfile-test").await?;
    file.write_all(content.as_bytes()).await?;
    if file.metadata().await?.len() > 0 {
        println!("Modelfile created successfully!");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let ollama = Ollama::default();

    print_ascii();
    println!();
    // async_chat(&ollama).await?;
    basic_prompt(&ollama).await?;
    Ok(())
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

async fn basic_prompt(ollama: &Ollama) -> Result<()> {
    let mut stdout = tokio::io::stdout();

    let model = MODEL.to_string();
    // let prompt = "Why is the sky blue?".to_string();
    stdout.write_all(b"\n> ").await?;
    stdout.flush().await?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let prompt = input.trim_end().to_string();
    let options = GenerationOptions::default().temperature(0.2);
    // .repeat_penalty(1.5)
    // .top_k(25)
    // .top_p(0.25);

    let mut messages = vec![ChatMessage::user(prompt.clone())];

    // let system_message = ChatMessage::system();
    // messages.push(system_message);

    let res = ollama
        .send_chat_messages(
            ChatMessageRequest::new(model, messages)
                .format(FormatType::Json)
                .options(options),
        )
        .await;

    if let Ok(res) = res {
        let response = res.message.unwrap().content;
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        println!("<< {:#?}", json);
    }
    process::exit(0);
}

async fn async_chat(ollama: &Ollama) -> Result<()> {
    // Convert the iterator to an asynchronous stream
    let mut stdout = tokio::io::stdout();

    let mut messages: Vec<ChatMessage> = vec![];

    // const MODELFILE_SCHEMA: &str = r#"
    //     FROM llama2

    //     # set the temperature to 1 [higher is more creative, lower is more coherent]
    //     PARAMETER temperature random_temperature

    //     # set the system message
    //     SYSTEM """
    //     You are {name}. Answer as {name}, the assistant, only.
    //     """
    // "#;

    // let aktion_system_mock: String = format!(
    //     "
    //     You are a helpful AI assistant!.\n
    //     The user will enter a name and the assistant will create a Modelfile structure\n
    //     Replace random_temperature with a number between 0.0 and 1.\n
    //     Output should be in Modelfile text document using the schema defined here: {}.\n
    //     Just print the Modelfile and nothing else.\n
    // ",
    //     MODELFILE_SCHEMA
    // );

    let aktion_math_asst: String = format!(
        "
        You are a helpful AI assistant!.\n
        Output should be in JSON using the schema defined here: {}.\n
        Just print the JSON and nothing else.\n
        Print JSON code block please.
    ",
        MATH_ASST_SCHEMA
    );

    let aktion_math_sub: String = format!(
        "
        You are a helpful AI assistant!.\n
        You know which function to use in order to subtract two numbers.\n
        Output should be in JSON using the schema defined here: {}.\n
        Just print the JSON and nothing else.\n
        Print JSON code block please.
    ",
        SUB_FUNC_SCHEMA
    );
    let aktion_math_sum: String = format!(
        "
        You are a helpful AI assistant!.\n
        You know which function to use in order to add two numbers.\n
        Output should be in JSON using the schema defined here: {}.\n
        Just print the JSON and nothing else.\n
        Print JSON code block please.
    ",
        SUM_FUNC_SCHEMA
    );

    let options = GenerationOptions::default().temperature(0.2);

    let system_message = ChatMessage::system(aktion_math_asst.clone());
    messages.push(system_message);
    // let system_message = ChatMessage::system(aktion_math_sum.clone());
    // messages.push(system_message);
    // let system_message = ChatMessage::system(aktion_math_sub.clone());
    // messages.push(system_message);

    // let user_message = ChatMessage::user("create a modelfile".to_string());
    // messages.push(user_message);

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // if input.eq_ignore_ascii_case("create a modelfile") {
        //     create_a_modelfile(&messages).await?;
        //     println!("Modelfile created successfully!");
        //     continue;
        // }

        let user_message = ChatMessage::user(input.to_string());
        messages.push(user_message);

        let mut stream: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(
                ChatMessageRequest::new(MODEL.to_string(), messages.clone())
                    .options(options.clone()),
            )
            .await?;

        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                stdout
                    .write_all(assistant_message.content.as_bytes())
                    .await?;
                stdout.flush().await?;
                response += assistant_message.content.as_str();
            }
        }

        messages.push(ChatMessage::assistant(response));
    }
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}

fn print_ascii() {
    println!(
        r#"  
_______ ______  _____ _____                ________                                                  
___    |___  /____  /____(_)______ _______ __  ___/________ ___      ________ ________ _____ ________
__  /| |__  //_/_  __/__  / _  __ \__  __ \_____ \ ___  __ \__ | /| / /_  __ `/__  __ \_  _ \__  ___/
_  ___ |_  ,<   / /_  _  /  / /_/ /_  / / /____/ / __  /_/ /__ |/ |/ / / /_/ / _  / / //  __/_  /    
/_/  |_|/_/|_|  \__/  /_/   \____/ /_/ /_/ /____/  _  .___/ ____/|__/  \__,_/  /_/ /_/ \___/ /_/     
                                                   /_/                                               
                      "#
    );
    println!();
    println!("Press Ctrl-C or type 'exit' to quit.");
}
