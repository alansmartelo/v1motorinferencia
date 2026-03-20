# 🧠 Motor de Inferência - Rust

Este projeto é um **Sistema Especialista de Encadeamento Reverso** (Backward Chaining) construído para fins de aprendizado. Ele demonstra como um programa pode "raciocinar" para trás, partindo de um objetivo final e buscando as causas necessárias para prová-lo.

## 🚀 O que eu aprendi neste projeto

### 1. A Natureza do Rust (Compilado vs Script)
Diferente de linguagens como Python, o **Rust** não é um script que "roda na hora". Ele é uma **linguagem compilada**.
* Eu escrevi o código em arquivos `.rs`.
* O **Compilador (Cargo)** transformou esse texto em um arquivo **Binário (ELF)**.
* Uma vez gerado o binário, o programa é independente e não precisa do Rust instalado para rodar.

### 2. O Fluxo de Trabalho (Nuvem para o Local)
Utilizei o **GitHub Codespaces** como minha fábrica de software:
1.  **Desenvolvimento:** Escrita do código no editor online.
2.  **Compilação:** Comando `cargo build --release` para gerar o executável otimizado.
3.  **Distribuição:** Download do binário para o meu notebook com **Zorin OS 17**.

### 3. Anatomia do Binário no Linux
No Linux, as extensões (como `.txt` ou `.exe`) não definem o que um arquivo faz. O que manda são as **permissões**.
* **ELF:** É o formato dos executáveis Linux. Se abrir no editor de texto, parece "código Matrix".
* **chmod +x:** É o comando que ativa o "interruptor de execução" do arquivo, transformando dados em um programa funcional.



---

## 🛠️ Como o Motor Funciona (Lógica)

O motor utiliza **Encadeamento Reverso**. Ele funciona como um detetive:

1.  **Define um Objetivo:** Ex: "O monitor está estragado?".
2.  **Busca Regras:** Procura no código quais condições levam a essa conclusão.
3.  **Investiga as Causas:** Se uma regra diz que "Monitor Preto" causa "Monitor Estragado", ele passa a tentar provar o "Monitor Preto".
4.  **Interação:** Se ele não encontrar uma resposta nas regras ou nos fatos conhecidos, ele **pergunta ao usuário** em tempo real.



---

## 💻 Comandos Essenciais que usei no Terminal

### No Codespace (Para Fabricar):
* `cargo init`: Inicia a estrutura de um projeto Rust.
* `. "$HOME/.cargo/env"`: Ativa as ferramentas do Rust no terminal.
* `cargo build --release`: Fabrica o binário final (fica em `target/release/`).

### No Zorin OS (Para Rodar):
* `cd ~/Downloads`: Entra na pasta onde o arquivo foi baixado.
* `chmod +x nome_do_arquivo`: Dá permissão de execução.
* `./nome_do_arquivo`: Executa o programa.

---

## 🎯 Próximos Passos (v2)
* Separar o conhecimento (Regras) em um arquivo **JSON** externo.
* Permitir que o motor resolva problemas médicos ou de lógica de jogos.

# 🤖 Motor de Inferência (v2.2 - O Salto Poliglota)

Este projeto evoluiu de um protótipo estático para um sistema de IA dinâmico e flexível escrito em **Rust**.

## 🔄 Evolução do Projeto

Anteriormente (v1), o motor tinha suas regras fixas no código-fonte. Seguindo o roadmap planejado, o projeto alcançou os seguintes marcos:

### ✅ Conhecimento Externo (v2.0)
Conforme planejado, separamos o "cérebro" do motor. As regras de lógica agora vivem em arquivos **JSON** externos, permitindo que o motor seja usado para medicina, botânica ou eletrônica sem tocar no código Rust.

### 🚀 O Salto Poliglota (v2.2)
Durante o desenvolvimento da v2, decidi ir além do JSON. O motor agora é **Poliglota**, capaz de detectar e interpretar tanto arquivos **JSON** quanto **YAML** automaticamente.
- **Detecção Automática:** O motor lê a extensão do arquivo (`.json` ou `.yaml`) e escolhe o tradutor (parser) correto em tempo de execução.
- **Flexibilidade Humana:** O suporte ao YAML permite que especialistas humanos (que não são programadores) escrevam regras de forma muito mais legível e natural.

---

## 🛠️ Como Usar a Versão Atual

1. **Prepare sua Base de Conhecimento:**
   Crie um arquivo `.json` ou `.yaml`. Exemplo de regra em YAML:
   ```yaml
   - conclusao: motor_fervendo
     premissas:
       - ponteiro_no_vermelho
       - vapor_saindo_capo
   ```

2. **Execute o Motor:**
   ```bash
   cargo run
   ```

3. **Interação:**
   O motor solicitará o caminho do arquivo e o objetivo que você deseja provar. Caso ele não encontre os fatos nas regras, ele perguntará diretamente a você!

---

## 📂 Tecnologias Utilizadas
- **Rust:** Core do motor de inferência.
- **Serde & Serde_JSON:** Para processamento de dados estruturados.
- **Serde_YAML:** Para suporte ao formato amigável de configuração.

## 🎯 Próximos Passos (v3)
- [ ] Criar um seletor automático que lista arquivos da pasta.
- [ ] Implementar suporte a explicações ("Por que você está me perguntando isso?").
- [ ] Interface gráfica (GUI) para usuários leigos.


