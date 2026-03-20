Design Doc: Motor de Inferência v1
Título: Sistema Especialista com Encadeamento Reverso em Rust
Autor: Alan Silva
Status: Concluído / Protótipo v1
Visão Geral
O objetivo deste projeto é criar um motor de inferência minimalista que utilize a estratégia de Backward Chaining (Encadeamento Reverso) para resolver problemas de diagnóstico lógico. O sistema deve ser capaz de navegar por uma árvore de dependências para provar uma hipótese final.
Arquitetura do Sistema
O sistema é composto por três blocos principais:
Base de Conhecimento (Regras): Uma lista de estruturas Regra contendo premissas e conclusões.
Base de Fatos: Um vetor dinâmico que armazena as verdades confirmadas durante a execução.
Máquina de Inferência: Uma função recursiva (provar) que atua como o "cérebro" do sistema.
Decisões de Design
Linguagem: Rust (pela segurança de memória e velocidade de execução binária).
Interface: CLI (Interface de Linha de Comando) para simplicidade e portabilidade entre Linux/Codespaces.
Recursividade: Escolhida para facilitar a navegação "para trás" nas regras.

2. SPEC (Especificações Técnicas)
Requisitos Funcionais
Entrada de Dados: O sistema deve aceitar "s" (sim) ou "n" (não) do usuário via teclado.
Processamento: O motor deve verificar se um fato já é conhecido antes de perguntar ao usuário.
Saída: Deve exibir claramente se a conclusão final foi provada ou não.
Requisitos Não-Funcionais
Desempenho: O binário deve ser compilado com a flag --release para máxima otimização.
Portabilidade: O binário gerado (formato ELF) deve ser compatível com sistemas baseados em Debian/Ubuntu (como o Zorin OS).
Estrutura de Dados (Rust)
Rust
struct Regra {
    premissas: Vec<String>,
    conclusao: String,
}


3. Post-Mortem (Análise Pós-Projeto)
O que é um Post-Mortem? É a reunião/texto onde analisamos o que deu certo e o que deu errado após o lançamento.
O que funcionou bem ✅
Portabilidade: A estratégia de compilar no Codespace e rodar no Zorin OS funcionou perfeitamente, provando que o Rust gera binários robustos.
Lógica de Inferência: O motor conseguiu pular perguntas desnecessárias quando a lógica levava a uma falha precoce.
Aprendizado: A transição de "script" para "binário" foi compreendida com sucesso.
Desafios Encontrados ⚠️
Instalação do Rust: O ambiente inicial do Codespace não tinha o compilador, exigindo a instalação via rustup.
Nomenclatura: O Rust impediu o uso de nomes de projetos começando com -, o que forçou uma renomeação.
Permissões de Arquivo: O binário baixado precisou do comando chmod +x no Zorin, algo que pode confundir usuários iniciantes.
O que faremos na v2? 🚀
Externalização: Tirar as regras de dentro do código (main.rs) e colocá-las em um arquivo JSON.
Interface Gráfica: Talvez uma interface simples em GTK ou via Web para não depender do terminal.