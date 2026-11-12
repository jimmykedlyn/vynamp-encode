# TASK-E21-15 — Ser a fonte única do rótulo lossy e da extensão

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-15` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-15` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §5 (FMT-AC-06) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | médio |

## Entrega

`Format::is_lossy` é verdadeiro para Opus, MP3, AAC e Vorbis, e é a única fonte do rótulo lossy usado pelo converter.

## Boundary

- **Não** preenche o campo `lossy` do `EncodeSummary`: o resumo de uma codificação concreta é de `TASK-E21-09`, e esta fornece a função que ele consulta. A confusão é plausível porque as duas falam do mesmo rótulo, mas aqui a pergunta é o valor por formato e lá é o resumo da sessão.

- **Não** escreve o relatório nem a interface do lote: o uso do rótulo pelo converter é de `TASK-E08-02`, que consome `is_lossy` e não mantém tabela própria. Aqui a entrega é a função e a tabela que ela lê, e uma segunda lista de lossy no app é exatamente o que a spec proíbe.

- **Não** é a matriz de parâmetros por formato: as faixas de taxa e profundidade validadas por `validate` são de `TASK-E21-10`. Aqui a tabela é a de propriedades fixas de `Format`, extensão e perda, que não varia por plano.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-15 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-15-AC-1` | os oito valores de `Format` e a tabela de `specs/formatos.md` §3 transcrita no teste com coluna de contêiner e coluna de perda | `is_lossy` e `extension` são lidos para cada um dos oito valores e confrontados com a tabela transcrita | Opus, Mp3, Aac e Vorbis são lossy, os outros quatro não; as extensões batem com §3 |
| `TASK-E21-15-AC-2` | o repo `vynamp-encode` e o `apps/converter`, com uma varredura por tabelas de perda declaradas nos dois | a varredura procura no código do converter qualquer lista de formatos com perda que não venha de `Format::is_lossy`, e o resultado é anexado ao relatório | nenhuma tabela de lossy independente existe no converter, e a única fonte do rótulo é a função do crate. |
| `TASK-E21-15-AC-3` | um valor de `Format` marcado como lossy e a extensão correspondente na tabela | a extensão devolvida por `extension()` é comparada com a coluna de contêiner da mesma linha | as oito extensões batem com a matriz, e nenhum formato com perda devolve extensão de contêiner sem perda. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/matriz_formatos.rs`, rodado por `cargo test --test matriz_formatos --locked`, mais uma varredura `grep` no repo `vynamp-converter` por listas de formato com perda, cujo comando e saída vão para `evidence/PR/task-e21-15/`. | Opus, MP3, AAC e Vorbis são lossy e os outros quatro não; as oito extensões batem com §3; e a varredura no converter não encontra tabela de perda independente. O que ainda não existe hoje: `Format` não existe em `libs/encode/src`, `tests/` não existe no repo, e o repo `vynamp-converter` tem 1 arquivo `.rs` de 3 linhas imprimindo a versão, então a varredura hoje devolve zero ocorrências por ausência de código, não por conformidade. |
| Hardware | nenhum: a prova é sobre PCM, tabelas de parâmetros e arquivos em memória ou em disco, e nenhum dispositivo de áudio é aberto; esta máquina é Windows 11 x64 com apenas o áudio integrado Intel e o Conexant ISST Audio, sem placa pro, sem aparelho Android, sem aparelho iOS e sem macOS, e nenhum dos critérios os exige | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final

## Notas de execução

Preenchido durante a execução, não antes.
