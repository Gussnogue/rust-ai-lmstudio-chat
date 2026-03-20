use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

fn main() -> anyhow::Result<()> {
    println!("🤖 Cliente Rust para LM Studio (Hermes)");
    println!("Digite sua pergunta (ou 'sair' para encerrar):\n");

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut pergunta = String::new();
        io::stdin().read_line(&mut pergunta)?;
        let pergunta = pergunta.trim();

        if pergunta.eq_ignore_ascii_case("sair") {
            break;
        }
        if pergunta.is_empty() {
            continue;
        }

        let request = ChatRequest {
            model: "hermes-3-llama-3.2-3b".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "Você é um assistente útil e direto.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: pergunta.to_string(),
                },
            ],
            temperature: 0.7,
            max_tokens: 500,
        };

        // Envia a requisição POST
        let response = ureq::post("http://localhost:1234/v1/chat/completions")
            .header("Content-Type", "application/json")
            .send_json(serde_json::to_value(&request)?)?;

        if response.status() != 200 {
            eprintln!("❌ Erro na API: {}", response.status());
            continue;
        }

        // Lê o corpo da resposta como bytes
        let body_bytes = response.into_body().read_to_vec()?;
        let chat_response: ChatResponse = serde_json::from_slice(&body_bytes)?;

        if let Some(choice) = chat_response.choices.first() {
            println!("\n🤖 Resposta:\n{}\n", choice.message.content);
        } else {
            println!("❌ Resposta vazia do modelo.\n");
        }
    }
    println!("👋 Até mais!");
    Ok(())
}

