//! # 🧠 Motor de Inferência v3.0
//! Este sistema simula um **Consultor Especialista**.
//!
//! ## 🛠️ Como funciona?
//! 1. O usuário entra com fatos/sintomas.
//! 2. O motor busca no arquivo `.json` ou `.yaml`.
//! 3. O motor faz perguntas para completar lacunas no diagnóstico.

use std::io::{self, Write};
use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Regra {
    premissas: Vec<String>,
    conclusao: String,
}

fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro ao ler");
    entrada.trim().to_lowercase().to_string()
}

/// Tenta encontrar novas conclusões com base nos fatos que já temos
fn inferir(fatos: &mut Vec<String>, regras: &Vec<Regra>) -> Vec<String> {
    let mut novas_conclusoes = Vec::new();
    
    for regra in regras {
        // Se já conhecemos a conclusão, pula
        if fatos.contains(&regra.conclusao) { continue; }

        // Verifica quantas premissas já temos
        let mut premissas_conhecidas = 0;
        let mut premissa_faltante = String::new();

        for p in &regra.premissas {
            if fatos.contains(p) {
                premissas_conhecidas += 1;
            } else {
                premissa_faltante = p.clone();
            }
        }

        // Se só falta UMA premissa, perguntamos ao usuário para tentar fechar a regra!
        if premissas_conhecidas == regra.premissas.len() - 1 && !premissa_faltante.is_empty() {
            println!("\n🤔 Investigando: Para concluir '{}', preciso saber...", regra.conclusao);
            print!("--- Isso acontece: '{}'? (s/n): ", premissa_faltante);
            io::stdout().flush().unwrap();
            let mut resp = String::new();
            io::stdin().read_line(&mut resp).unwrap();
            
            if resp.trim().to_lowercase() == "s" {
                fatos.push(premissa_faltante);
                // Recursão: agora que temos um novo fato, tentamos inferir tudo de novo
                return inferir(fatos, regras);
            }
        } 
        // Se já temos TODAS, a conclusão é automática
        else if premissas_conhecidas == regra.premissas.len() {
            fatos.push(regra.conclusao.clone());
            novas_conclusoes.push(regra.conclusao.clone());
            return inferir(fatos, regras);
        }
    }
    novas_conclusoes
}

fn main() {
    println!("\n=== 🧠 MOTOR DE INFERÊNCIA v3.0 (MODO CONSULTOR) ===");

    let nome_arquivo = ler_entrada("📁 Nome do arquivo de regras: ");
    let conteudo = fs::read_to_string(&nome_arquivo).expect("Arquivo não encontrado!");

    let regras: Vec<Regra> = if nome_arquivo.ends_with(".yaml") {
        serde_yaml::from_str(&conteudo).unwrap()
    } else {
        serde_json::from_str(&conteudo).unwrap()
    };

    println!("✅ Conhecimento carregado. Vamos começar o diagnóstico.");
    
    let sintomas = ler_entrada("\n📝 O que está acontecendo? (separe por vírgula): ");
    let mut fatos_conhecidos: Vec<String> = sintomas.split(',').map(|s| s.trim().to_string()).collect();

    println!("\n🧐 Analisando sintomas...");
    inferir(&mut fatos_conhecidos, &regras);

    println!("\n=== 🏁 DIAGNÓSTICO FINAL ===");
    println!("Fatos confirmados: {:?}", fatos_conhecidos);
}