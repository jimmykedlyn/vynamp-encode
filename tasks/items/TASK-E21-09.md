# TASK-E21-09 — Rotular toda saída lossy como lossy no resumo

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-09` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-09` |
| Critérios da spec | `specs/encode.md` §3, §4 e §5 (ENCODE-AC-09) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | médio |

## Entrega

O resumo marca `lossy = true` para Opus, MP3, AAC e Vorbis; nenhuma API do crate devolve rótulo hi-res ou lossless para saída lossy.

## Boundary

- **Não** define `Format::is_lossy` nem a lista de formatos com perda: a fonte única do rótulo é de `TASK-E21-15`, com a matriz em `specs/formatos.md` §3. Aqui o campo `lossy` do `EncodeSummary` é preenchido a partir dela, e uma segunda tabela divergiria na primeira mudança de matriz.

- **Não** escreve o relatório de conversão nem o texto de interface do lote: a regra de que a conversão lossy não é anunciada como restauração hi-res é de `TASK-E08-02`, no converter, que consome este campo. Aqui a obrigação termina no valor que o resumo devolve.

- **Não** prova a integridade dos formatos sem perda: a igualdade bit a bit de WAV, AIFF, FLAC e ALAC é de `TASK-E21-05`. Aqui, para esses quatro formatos, a única afirmação é que o resumo devolve `lossy == false`, e ela não substitui a prova de amostras.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-09 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-09-AC-1` | um FLAC 24 bits 96 kHz gerado no teste e codificado em Opus 256 kbps, com o `EncodeSummary` lido ao fim da sessão | o resumo é inspecionado campo a campo e o texto de cada campo é confrontado com a lista de termos proibidos para saída com perda | `lossy == true` e nenhum campo do resumo contém “hi-res” ou “lossless” |
| `TASK-E21-09-AC-2` | os quatro formatos lossy de `specs/formatos.md` §3 codificados a partir do mesmo PCM de entrada | o campo `lossy` do resumo de cada um é lido e comparado com a tabela de `is_lossy` transcrita no teste | os quatro devolvem `lossy == true` e os quatro sem perda devolvem `lossy == false`, sem exceção por formato. |
| `TASK-E21-09-AC-3` | um build em que o encoder MP3 está ausente, com o restante do conjunto de features igual | o registry é consultado e, para os formatos presentes, o resumo é lido como no primeiro critério | a ausência de um encoder lossy não faz nenhum outro formato mudar de rótulo, e nenhum resumo de saída lossy presente passa a se declarar sem perda. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/rotulo_lossy.rs`, rodado por `cargo test --test rotulo_lossy --locked`, que transcreve a coluna de lossy de `specs/formatos.md` §3 como tabela independente e varre os campos do resumo por um conjunto de termos proibidos declarado no próprio teste, sem consultar `Format::is_lossy` para decidir o esperado. | os quatro formatos lossy devolvem `lossy == true` e os quatro sem perda devolvem `lossy == false`, conforme a tabela transcrita; nenhum campo do resumo contém `hi-res`, `hires` nem `lossless` para saída com perda; e a ausência do encoder MP3 não muda o rótulo de nenhum outro formato. O que ainda não existe hoje: `EncodeSummary` e `Format::is_lossy` não existem em `libs/encode/src`, `tests/` não existe no repo e nenhum encoder produz resumo. |
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
