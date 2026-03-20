📑 Documentação Técnica - Motor de Inferência v3.0 (Modo Consultor)

Design Doc (O Salto na Inteligência)
Objetivo: Transformar o motor reativo (v1/v2) em um sistema proativo de diagnóstico capaz de guiar o usuário através de perguntas lógicas.

Mudança de Paradigma:
De Backward para Forward Chaining: O motor não espera mais por um objetivo final isolado. Ele aceita "sintomas" iniciais e tenta deduzir o máximo de conclusões possíveis.

Interatividade Recursiva: Se o motor identifica que uma regra está "quase completa" (faltando apenas um fato), ele interrompe a execução para interrogar o usuário.

Technical Spec (Como o "Cérebro" Funciona)

O algoritmo da v3.0 segue este fluxo lógico:
Input de Sintomas: O usuário insere fatos iniciais separados por vírgula.
Varredura de Regras: O motor percorre a base de conhecimento (JSON/YAML).
Cálculo de Proximidade: * Se uma regra tem todas as premissas satisfeitas $\rightarrow$ Conclusão é adicionada aos fatos automaticamente.
      Se uma regra tem $N-1$ premissas satisfeitas $\rightarrow$ O motor pergunta ao usuário sobre a premissa faltante.

Recursividade: Sempre que um novo fato é aprendido (por dedução ou por resposta do usuário), o motor reinicia a varredura para ver se esse novo fato "destrava" outras regras em cascata.

Post-Mortem (Lições da Madrugada)
Desafios Encontrados:

    Sensibilidade de String: Descobrimos que o motor é um "purista". Se o usuário digitar sol. com ponto, e a regra for sol, a conexão quebra.
    
    Ambiente de Dev: O Codespace pode perder o toolchain do Rust, exigindo reinstalação do rustup.
    
    Conectividade: A importância de nomes de arquivos e caminhos relativos na execução do binário compilado.

Aprendizados:
    
    Poder da Abstração: O mesmo código Rust agora cuida de plantas (Jiboia) e viagens (Destino Praia) apenas trocando o arquivo de dados.
    
    Lógica Proativa: É muito mais natural para um humano fornecer sintomas e ser guiado do que ter que saber a conclusão final de antemão.

🚀 Roadmap para a v3.1 (Backlog)
    [ ] Normalização de Input: Criar uma função limpar_texto para ignorar espaços extras, pontos e letras maiúsculas/minúsculas.

    [ ] Interface de Lista: Em vez de digitar, o usuário escolhe o arquivo por um número.

    [ ] Explicação (Why?): Permitir que o usuário pergunte "por que" antes de responder Sim ou Não.    