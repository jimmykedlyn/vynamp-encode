# TASK-E21-07 — Devolver erro de I/O e disco cheio sem pânico

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-07` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-07` |
| Critérios da spec | `specs/encode.md` §4 e §6 (ENCODE-AC-07) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Erro de I/O do `sink`, inclusive `StorageFull`, volta como `EncodeError::Io` com o `ErrorKind` original, sem `panic!`.

## Boundary

- **Não** trata cancelamento: `EncodeError::Cancelled` e o ponto de verificação do token são de `TASK-E21-06`, e confundir os dois faria uma falha de disco virar cancelamento pedido pelo usuário. Aqui o erro preserva o `ErrorKind` original, que é justamente o que a variante de cancelamento não carrega.

- **Não** remove temporário, não mapeia código de saída 3 nem relata progresso do job: a escrita atômica e o descarte do temporário são de `TASK-E08-02`, no converter, e `specs/encode.md` §3 diz que a sessão não abre, renomeia nem apaga arquivo. Aqui a sessão apenas devolve o erro sem pânico.

- **Não** é a contagem de frames depois de decodificar: quantas amostras sobrevivem a uma escrita interrompida não interessa aqui, porque a spec declara o `sink` indefinido depois de `Io` e o chamador descarta o temporário. A prova desta TASK é o tipo do erro e a ausência de pânico, e `TASK-E21-03` segue dona da contagem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-07 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-07-AC-1` | um `Write` de teste que aceita a primeira metade dos bytes e devolve `io::ErrorKind::StorageFull` na metade seguinte, e uma sessão FLAC aberta sobre ele | a sessão escreve blocos até o `sink` falhar, e o erro devolvido é comparado com a variante e o `kind` declarados na spec | volta `Err(EncodeError::Io{kind: StorageFull, ..})`, sem pânico |
| `TASK-E21-07-AC-2` | o mesmo `Write` de teste configurado para devolver `PermissionDenied` e depois `WriteZero` em execuções separadas | a sessão escreve até cada falha e o `ErrorKind` devolvido é lido em cada caso | o `kind` do erro é o mesmo que o `sink` devolveu em cada execução, sem tradução para uma variante genérica. |
| `TASK-E21-07-AC-3` | a mesma falha de `StorageFull` dentro de `finish`, depois de todos os blocos terem sido aceitos | `finish` é chamado sobre o `sink` que falha no trailer e o processo que roda o teste é observado quanto a pânico | `finish` devolve `Err(EncodeError::Io{kind: StorageFull, ..})` e o teste termina sem pânico nem `abort`. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/io_falha.rs`, rodado por `cargo test --test io_falha --locked`, com um `std::io::Write` de teste que devolve o `ErrorKind` pedido depois de N bytes, e o `kind` comparado com o declarado na spec em vez de reconstruído a partir do texto da mensagem. | cada `ErrorKind` injetado volta na variante `Io` com o mesmo `kind`, inclusive em `finish`; nenhum caso termina em pânico; e a mensagem do erro não vira critério de comparação, porque o `ErrorKind` é o dado estável. O que ainda não existe hoje: `EncodeError::Io` não existe em `libs/encode/src`, `tests/` não existe no repo e não há caminho de escrita que possa falhar. |
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
