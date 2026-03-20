use std::io::{self, Write};
use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Regra {
    premissas: Vec<String>,
    conclusao: String,
}

fn perguntar_usuario(fato: &str) -> bool {
    print!("\n--- Pergunta: Isso está acontecendo: '{}'? (s/n): ", fato);
    io::stdout().flush().unwrap();
    let mut resposta = String::new();
    io::stdin().read_line(&mut resposta).unwrap();
    resposta.trim().to_lowercase() == "s"
}

fn provar(objetivo: &str, fatos: &mut Vec<String>, regras: &Vec<Regra>) -> bool {
    if fatos.contains(&objetivo.to_string()) { return true; }

    for regra in regras {
        if regra.conclusao == objetivo {
            let mut todas_provadas = true;
            for premissa in &regra.premissas {
                if !provar(premissa, fatos, regras) {
                    todas_provadas = false;
                    break;
                }
            }
            if todas_provadas {
                fatos.push(objetivo.to_string());
                return true;
            }
        }
    }

    if perguntar_usuario(objetivo) {
        fatos.push(objetivo.to_string());
        return true;
    }
    false
}

fn main() {
    let dados_json = fs::read_to_string("regras.json")
        .expect("Erro ao abrir o arquivo regras.json");
    
    let regras: Vec<Regra> = serde_json::from_str(&dados_json)
        .expect("Erro ao converter o JSON");

    let mut fatos_conhecidos = Vec::new();
    let objetivo = "monitor_estragado";

    println!("\n=== MOTOR DE INFERÊNCIA v2 (JSON) ===");
    println!("Conhecimento carregado: {} regras do arquivo externo.", regras.len());

    if provar(objetivo, &mut fatos_conhecidos, &regras) {
        println!("\n✅ CONCLUSÃO: {} é VERDADE.", objetivo);
    } else {
        println!("\n❌ CONCLUSÃO: Não foi possível provar {}.", objetivo);
    }
}