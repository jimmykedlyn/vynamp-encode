# TASK-E21-01 — Entregar registry de encoders e disponibilidade por build

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-01` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-01` |
| Critérios da spec | `specs/encode.md` §4 e §5 (ENCODE-AC-01) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | nenhuma |
| Bloqueia | `TASK-E21-02`, `TASK-E21-03`, `TASK-E21-04`, `TASK-E21-05`, `TASK-E21-06`, `TASK-E21-07`, `TASK-E21-08`, `TASK-E21-09`, `TASK-E21-10`, `TASK-E21-15`, `TASK-E21-16`, `TASK-E18-02`, `TASK-E08-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

`Registry` com `formats()` e `availability()` expondo cada um dos oito formatos de `specs/formatos.md` §3 como presente, com nome e versão do encoder, ou ausente com motivo, na build em que foi compilado.

## Boundary

- **Não** liga `Availability::Absent` a erro tipado: o caminho de recusa do MP3 e a garantia de zero byte escrito no `sink` são de `TASK-E21-02`, no mesmo módulo. Aqui a única obrigação é enumerar as oito linhas com uma das duas variantes, e antecipar o erro fecharia o critério dela por tabela.

- **Não** entrega orquestração de lote, escrita atômica nem validação pós-escrita: `ENCODE-REQ-01` em `specs/encode.md` §4 põe o registry nesta lib, e o que fica em `TASK-E08-02` é o trabalho em lote sobre ele. Escrever a escolha de encoder no app criaria dois donos da mesma tabela de oito formatos.

- **Não** escolhe qual implementação de encoder entra na build nem resolve licença ou patente: o inventário de codecs, fontes e políticas de distribuição é de `TASK-E11-03`, em `vynamp`. Aqui o registry apenas relata o que a compilação incluiu, sem prometer encoder que a build não tem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-01 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-01-AC-1` | a tabela de oito linhas de `specs/formatos.md` §3 transcrita no teste com formato, extensão, profundidade e marca de lossy, e o `Cargo.toml` do repo com `[dependencies]` vazio | `Registry::with_builtin().formats()` é consumido até o fim do iterador e cada entrada é confrontada com a linha correspondente da tabela | os 8 formatos de `formatos.md` aparecem, cada um com `Present{encoder, version}` não vazio ou `Absent{reason}` não vazio |
| `TASK-E21-01-AC-2` | o mesmo registry e a tabela transcrita, e um laço que percorre os oito formatos separado do laço que enumera `formats()` | `availability` é chamado formato a formato e cada variante devolvida é comparada com a que `formats()` enumerou para o mesmo formato | `availability` devolve, para cada um dos oito formatos, a mesma variante que a entrada correspondente de `formats()`. |
| `TASK-E21-01-AC-3` | o repo `vynamp-encode` recém-compilado no perfil padrão, com `Cargo.lock` versionado e sem feature extra ligada | `cargo tree` e `cargo metadata` são lidos e conferidos contra a lista de dependências declaradas no `Cargo.toml` | a build padrão não carrega crate de encoder de terceiro, e `vynamp-dsp` aparece só atrás da feature `resample`, desligada por padrão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/registry.rs`, rodado por `cargo test --test registry --locked`, que transcreve à mão a matriz de `specs/formatos.md` §3 como tabela de oito linhas independente do código, mais a leitura de `cargo tree --manifest-path libs/encode/Cargo.toml` e de `cargo metadata --no-deps` no mesmo job. | as oito entradas aparecem e cada uma devolve `Present` com `encoder` e `version` não vazios ou `Absent` com `reason` não vazio; a tabela transcrita faz o teste falhar se um formato sumir da enumeração, se a ordem mudar ou se uma variante vier vazia; `cargo metadata` não lista crate de encoder de terceiro na build padrão. O que ainda não existe hoje: `libs/encode/src/lib.rs` tem 6 linhas e só `pub const VERSION`; `[dependencies]` do `Cargo.toml` está vazio; não há diretório `tests/` no repo; e `grep -rn Registry libs/encode/src` devolve 0 linhas. |
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
