# TASK-E21-14 — Impedir que WAV float 32 vire inteiro sem pedido

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-14` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-14` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §6 (FMT-AC-05) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

WAV float 32 nunca vira inteiro sem pedido explícito de `SampleFormat` inteiro com confirmação.

## Boundary

- **Não** é a matriz de profundidades por formato: quais valores de `SampleFormat` cada contêiner admite é de `TASK-E21-10`, de que esta depende. Aqui a regra é sobre a passagem de uma família de amostra para outra, de ponto flutuante para inteiro, que a matriz admite mas o plano precisa pedir.

- **Não** é a redução dentro da mesma família: passar de 24 para 16 bits inteiros é de `TASK-E21-11`. A confusão é plausível porque as duas perdem resolução e as duas usam confirmação, mas float para inteiro muda a representação da amostra e tem `field` próprio, que é `sample_format`.

- **Não** prova que a saída inteira preserva as amostras quantizadas: a igualdade bit a bit dos formatos lossless é de `TASK-E21-05`. Aqui a entrega é a recusa do pedido implícito, antes de qualquer escrita.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-14 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-14-AC-1` | uma fonte WAV float 32 descrita no plano e um plano WAV `Int24` sem confirmação, com o `SampleFormat` inteiro pedido explicitamente | `validate` roda sobre esse plano e o erro é inspecionado quanto ao campo culpado | falha com `InvalidParams{field: "sample_format"}` |
| `TASK-E21-14-AC-2` | o mesmo plano com `confirmed = true` e o mesmo plano mantendo `Float32` na saída | os dois passam por `validate` e os dois `ValidatedPlan` são comparados | o plano confirmado é aceito com a conversão declarada em `degradations`, e o plano que mantém `Float32` não escreve linha nenhuma, provando que a regra só alcança a conversão. |
| `TASK-E21-14-AC-3` | um plano WAV `Int24` a partir de uma fonte que já é inteira de 24 bits, sem `Float32` em nenhum dos lados | `validate` roda sobre esse plano e o resultado é comparado com o do critério anterior | o plano é aceito sem confirmação, porque não houve passagem de ponto flutuante para inteiro. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/matriz_formatos.rs`, rodado por `cargo test --test matriz_formatos --locked`, com o `field` esperado copiado de `specs/formatos.md` §5 e a fonte float descrita no plano pelo campo `source_bits` mais a marca de formato de amostra, sem inferir o caso a partir do código sob teste. | `Int24` sem confirmação sobre fonte float 32 falha com `InvalidParams{field: "sample_format", ..}`; o mesmo plano confirmado passa e declara a conversão; manter `Float32` não exige confirmação; e fonte já inteira não é tratada como conversão de família. O que ainda não existe hoje: `SampleFormat` não existe em `libs/encode/src`, `validate` não existe no crate e `tests/` não existe no repo. |
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
