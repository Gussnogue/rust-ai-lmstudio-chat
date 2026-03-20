# 🦀 Rust AI LMStudio Chat – Cliente CLI para Modelos Locais

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://rustlang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Dependency: ureq](https://img.shields.io/badge/ureq-3.2-blue)](https://crates.io/crates/ureq)
[![Dependency: serde](https://img.shields.io/badge/serde-1.0-purple)](https://serde.rs)

Aplicação de linha de comando (CLI) desenvolvida em **Rust** para interagir com modelos de linguagem rodando localmente no **LM Studio**. O programa se comunica com a API compatível com OpenAI do LM Studio, permitindo conversas diretas pelo terminal com eficiência e leveza.

* * *

## ✨ Funcionalidades

* ✅ **CLI interativa:** Loop contínuo de perguntas e respostas diretamente no terminal.
* ✅ **Integração com LM Studio:** Conecta-se ao servidor local do LM Studio (porta 1234) usando a API compatível com OpenAI.
* ✅ **Execução leve e rápida:** Código Rust compilado, sem dependências de interpretadores ou máquinas virtuais.
* ✅ **Serialização eficiente:** Utiliza `serde` e `serde_json` para manipular os dados da API.
* ✅ **Tratamento de erros robusto:** Uso de `anyhow` para simplificar o gerenciamento de erros.
* ✅ **Portável:** Gera um único executável que pode ser distribuído sem necessidade de instalar Rust ou outras dependências.
* ✅ **Pronto para uso:** Basta ter o LM Studio rodando com o modelo desejado (ex: `hermes-3-llama-3.2-3b`).

* * *

## 🚀 Como Executar

### Pré-requisitos

* **LM Studio** instalado e rodando com:
  * Um modelo de chat carregado (ex: `hermes-3-llama-3.2-3b`).
  * Servidor local ativo na porta **1234** (padrão do LM Studio).

### Passo a Passo

1. **Clone o repositório**
   ```bash
   git clone https://github.com/Gussnogue/rust-ai-lmstudio-chat.git
   cd rust-ai-lmstudio-chat

2. **Compile o projeto**

   ```bash
   cargo build --release
   
4. **Execute o programa**
   ```bash
   cargo run --release

# 🧠 Como Funciona
O programa entra em um loop lendo a entrada do usuário.

A cada pergunta, monta uma requisição JSON no formato esperado pela API do LM Studio.

Utiliza a crate ureq para enviar uma requisição POST para http://localhost:1234/v1/chat/completions.

Recebe a resposta, desserializa o JSON e exibe o conteúdo da mensagem de resposta.

O loop continua até que o usuário digite sair.

# 🛠️ Tecnologias Utilizadas
Rust – Linguagem de programação.

ureq – Cliente HTTP leve e simples (sem async).

serde / serde_json – Serialização e desserialização de dados.

anyhow – Tratamento de erros simplificado.

# 📄 Licença
Este projeto está licenciado sob a MIT License. Sinta-se à vontade para usar, modificar e distribuir.
