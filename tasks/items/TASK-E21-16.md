# TASK-E21-16 — Não oferecer saída DSD

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-16` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-16` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §5 (FMT-AC-07) |
| Fase | M3 |
| Prioridade | P2 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 1–3 dias úteis |
| Risco | médio |

## Entrega

Não existe saída DSD: `Format` não tem variante DSF nem DFF, e o pedido é recusado como erro de uso antes de chegar à lib.

## Boundary

- **Não** é a recusa de formato presente na matriz e ausente na build: `FormatAbsent` do MP3 é de `TASK-E21-02`. Aqui o formato não existe no enum em build nenhuma, e por isso a recusa acontece no parsing, antes de `EncodeParams` ser montado.

- **Não** converte DSD para PCM na entrada: a conversão é de `TASK-E01-09`, em `vynamp-decode`, e a entrada DSD chega ao encoder já convertida pelo chamador. Aqui a afirmação é só sobre a saída, e escrever um conversor DSD neste crate criaria um decodificador dentro do encoder.

- **Não** é a matriz de combinações válidas: as faixas aceitas por formato são de `TASK-E21-10`. Aqui a entrega é a ausência da variante e o erro de uso no parsing do formato, que acontece antes de qualquer validação de plano.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-16 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-16-AC-1` | o enum `Format` do crate e uma lista de nomes de formato aceitos no parsing escrita à mão a partir de `specs/formatos.md` §3, sem nenhum nome de DSD | a lista é percorrida e cada nome é convertido pelo parsing do crate | não existe variante `Format` para DSD e o parsing do formato devolve erro de uso |
| `TASK-E21-16-AC-2` | os nomes `dsf` e `dff` e as variantes de DSD usuais, submetidos ao mesmo parsing que aceita os oito nomes válidos | o parsing é chamado para cada nome de DSD e o resultado é comparado com o dos oito nomes válidos | nenhum nome de DSD é aceito, e o erro devolvido é erro de uso no parsing, não uma variante de `Format`. |
| `TASK-E21-16-AC-3` | a lista de variantes do enum, lida do código compilado por um teste que a enumera exaustivamente com `match` sem braço curinga | o teste é compilado com o `match` exaustivo sobre `Format` e a contagem de variantes é conferida | o `match` exaustivo compila com exatamente as oito variantes de §3, e acrescentar uma variante de DSD quebraria a compilação do teste em vez de passar despercebida. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/matriz_formatos.rs`, rodado por `cargo test --test matriz_formatos --locked`, com a lista de nomes válidos transcrita de `specs/formatos.md` §3 e um `match` exaustivo sem braço curinga sobre `Format`, que é a prova estrutural de que a variante não existe. | os oito nomes válidos são aceitos, `dsf` e `dff` são recusados no parsing com erro de uso, e o `match` exaustivo compila com oito variantes. O que ainda não existe hoje: `Format` não existe em `libs/encode/src`, não há parsing de formato no crate, `tests/` não existe no repo e `specs/formatos.md` §10 registra que nenhum tipo da spec existe em `src/`. |
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
