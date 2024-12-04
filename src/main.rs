use chatgpt::client::ChatGPT;
use chatgpt::config::{ChatGPTEngine, ModelConfigurationBuilder};
use chatgpt::types::CompletionResponse;
use rocket::form::Form;
use rocket::fs::{FileServer, Options};
use rocket::response::content::RawHtml;
use rocket::State;
use tokio::sync::RwLock;
use tokio::time::Instant;

use gppt::Generator;

const PPT_DIR: &str = "./ppt";
const PROMPT: &str = include_str!("prompt.txt");

fn prompt_for(subject: &str) -> String {
    PROMPT.replace("{SUBJECT}", subject)
}

#[rocket::launch]
async fn launch() -> _ {
    // Getting the API key here
    let key = std::env::var("OPENAI_KEY").unwrap();

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .engine(ChatGPTEngine::Gpt35Turbo)
            .build()
            .unwrap(),
    )
    .unwrap();

    let fs = FileServer::new(PPT_DIR, Options::default());

    rocket::build()
        .manage(client)
        .manage(RwLock::new(Generator::new(PPT_DIR)))
        .mount("/ppt", fs)
        .mount("/", rocket::routes![submit, submit_post, ppt_list])
}

#[derive(rocket::FromForm)]
struct SubmitForm {
    password: String,
    theme: String,
}

#[rocket::get("/")]
fn submit() -> RawHtml<&'static str> {
    RawHtml(
        r###"<form method="POST"><input type="password" name="password" placeholder="Password"><input name="theme" placeholder="Agriculture..."><input type="submit"></form>"###,
    )
}

#[rocket::post("/", data = "<form>")]
async fn submit_post(
    form: Form<SubmitForm>,
    chat_gpt: &State<ChatGPT>,
    generator: &State<RwLock<Generator>>,
) -> Result<RawHtml<String>, String> {
    if form.password != "Bourne" {
        return Err("unauthorized".into());
    }

    let theme = &form.theme;

    // Sending a message and getting the completion
    let start = Instant::now();
    let response: CompletionResponse = chat_gpt.send_message(prompt_for(theme)).await.unwrap();

    let elapsed = start.elapsed();
    let price = response.usage.total_tokens;

    let mut generator = generator.write().await;
    match generator.update(theme, &response.message().content) {
        Err(err) => Err(err.to_string()),
        Ok(name) => Ok(RawHtml(format!(
            r#"time={elapsed:?}, tokens={price}<br><a href="/ppt/{name}">/ppt/{name}</a>"#
        ))),
    }
}

#[rocket::get("/ppt")]
async fn ppt_list() -> Result<RawHtml<String>, std::io::Error> {
    use std::fmt::Write;

    let mut buf = String::new();
    let mut read_dir = tokio::fs::read_dir(PPT_DIR).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let _ = writeln!(buf, r###"<a href="/ppt/{name}">{name}</a>"###);
    }

    Ok(RawHtml(buf))
}
