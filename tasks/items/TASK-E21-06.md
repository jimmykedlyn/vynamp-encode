# TASK-E21-06 — Interromper a sessão por `CancelToken` sem escrever mais nada

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-06` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-06` |
| Critérios da spec | `specs/encode.md` §3, §4 e §6 (ENCODE-AC-06) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Cancelamento por `CancelToken` interrompe a sessão no próximo `write` ou durante `finish`, devolve `EncodeError::Cancelled` e não escreve mais nada no `sink`.

## Boundary

- **Não** remove temporário nem decide o efeito do cancelamento no lote: a limpeza do temporário, o código de saída 4 e a preservação da origem são de `TASK-E08-02`, no converter. Aqui o contrato termina em `Cancelled` e na garantia de que o `sink` não cresce depois disso.

- **Não** é a política de erro de I/O: `EncodeError::Io` com o `ErrorKind` original e o caso `StorageFull` são de `TASK-E21-07`. As duas interrompem a escrita, mas só o cancelamento é decisão do chamador; tratar falha do `sink` como cancelamento engoliria a causa do erro.

- **Não** define o `Registry` nem a abertura da sessão: o token é parâmetro de `Registry::open`, conforme `specs/encode.md` §3, e a enumeração de formatos é de `TASK-E21-01`. Aqui a entrega é o ponto de verificação do token dentro de `write` e de `finish`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-06 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-06-AC-1` | uma sessão FLAC aberta sobre um `sink` em memória que conta bytes, alimentada com blocos até cobrir 10 s de áudio, e o `CancelToken` preparado para ser acionado entre dois blocos | `cancel()` é chamado entre dois `write` e o `write` seguinte é executado com o contador do `sink` lido antes e depois | o `write` seguinte devolve `Cancelled` e `bytes_written` do `sink` não cresce depois disso |
| `TASK-E21-06-AC-2` | a mesma sessão com o token acionado durante `finish`, depois de todos os blocos terem sido aceitos | `finish` é chamado com o token já cancelado e o retorno é comparado com o do critério anterior | `finish` devolve `EncodeError::Cancelled`, o `sink` não ganha trailer nem padding depois do cancelamento, e a sessão não pode ser reaproveitada. |
| `TASK-E21-06-AC-3` | o mesmo token clonado e compartilhado entre a sessão e uma segunda cópia que o aciona a partir de outra tarefa, com um `Write` de teste que registra a ordem das chamadas | `cancel()` é chamado pela cópia clonada enquanto a sessão escreve, e a ordem registrada pelo `Write` é lida depois | o cancelamento feito pela cópia clonada é visto na sessão, a ordem registrada mostra que nenhuma escrita acontece depois do ponto de cancelamento, e `cancel()` repetido não muda o resultado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/cancelamento.rs`, rodado por `cargo test --test cancelamento --locked`, com um `Write` de teste que registra cada chamada e o tamanho de cada buffer, mais `std::thread` para provar que a cópia clonada do token atravessa a fronteira — a ordem registrada é a evidência, não o estado interno da sessão. | o `write` seguinte ao cancelamento devolve `Cancelled` e o contador de bytes não cresce; `finish` cancelado devolve o mesmo erro e não escreve trailer; o token clonado aciona a sessão aberta em outra thread; nenhuma escrita aparece na ordem registrada depois do ponto de cancelamento. O que ainda não existe hoje: `CancelToken` e `EncodeError::Cancelled` não existem em `libs/encode/src`, e `tests/` não existe no repo. |
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
