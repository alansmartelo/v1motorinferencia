📑 Documentação Técnica - Motor de Inferência v2.2 (Poliglota)
1. Design Doc (O Projeto)
Objetivo: Evoluir o motor de inferência estático (v1) para um sistema dinâmico, desacoplado e capaz de processar múltiplas linguagens de dados.

Decisões de Arquitetura:
Desacoplamento de Dados: A inteligência (regras) foi movida para arquivos externos (.json e .yaml). O motor Rust tornou-se um "interpretador puro".

Abstração Poliglota: Implementação de uma lógica de detecção de extensão. O motor decide em tempo de execução se usa o parser serde_json ou serde_yaml.

Recursividade: Manutenção do algoritmo de Backward Chaining (Encadeamento Reverso) para exploração de árvores de decisão complexas.

2. Technical Spec (Especificação)
O sistema opera seguindo o fluxo abaixo:

Input: Nome do arquivo de conhecimento e o objetivo (string) a ser provado.

Parser: * Se .json: Serialização via serde_json.

Se .yaml ou .yml: Serialização via serde_yaml.

Processamento: 1. Verifica se o fato já é conhecido.
2. Busca regras onde o objetivo é a conclusão.
3. Tenta provar todas as premissas dessa regra recursivamente.
4. Se falhar, pergunta ao usuário (Input Interativo).

Output: Booleano (Verdadeiro/Falso) com log de passos no terminal.

3. Post-Mortem (Análise de Erros e Aprendizado)
O que deu errado?
Perda de arquivos iniciais: Devido à troca de branches sem commit, os arquivos da v2 foram perdidos temporariamente.

Conflitos de Nome: O Rust apresentou sensibilidade a nomes de projetos com caracteres especiais.

Erro de Ambiente: O binário falhou ao rodar localmente no Zorin OS porque o arquivo regras.json não estava no diretório esperado (Panic: NotFound).

O que aprendemos?
Git é Seguro: Commits frequentes salvam vidas (e noites de sono).

Localização de Recursos: Binários compilados em Rust buscam arquivos externos no caminho relativo ao executável.

Flexibilidade do JSON/YAML: É muito mais fácil manter uma IA editando um arquivo de texto do que recompilando código-fonte.