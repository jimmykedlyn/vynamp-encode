# TASK-E21-13 — Exigir ganho declarado na troca mono e estéreo

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-13` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-13` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §6 (FMT-AC-04) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Mudança mono↔estéreo é explícita e exige `channel_gain_db` declarado.

## Boundary

- **Não** é a matriz de canais por formato: quais contagens de canais cada contêiner admite é de `TASK-E21-10`, de que esta depende. Aqui a regra é sobre a mudança entre fonte e destino, que a matriz não vê, e é ela que exige o ganho declarado.

- **Não** é a confirmação de profundidade: a linha de 24→16 bits em `degradations` é de `TASK-E21-11`. As duas moram no mesmo bloco `Degradation`, e por isso o `field` do erro é o que separa as duas obrigações no teste.

- **Não** é o erro de bloco com canais divergentes do plano: `BlockMismatch` é de `TASK-E21-04`, conforme `specs/encode.md` §3, e acontece na escrita. Aqui a decisão é anterior, na validação do plano, e o plano que pede a troca é aceito desde que o ganho venha declarado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-13 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-13-AC-1` | uma fonte mono descrita no plano e um plano estéreo, primeiro sem `channel_gain_db` e depois com o ganho declarado em dB | `validate` roda sobre os dois planos e os resultados são comparados entre si | sem ganho falha; com ganho passa e o ganho aparece em `degradations` |
| `TASK-E21-13-AC-2` | o plano com ganho declarado do critério anterior e um plano que mantém a contagem de canais da fonte | os dois passam por `validate` e o vetor `degradations` de cada um é lido | o plano que mantém os canais não exige ganho e não escreve linha de canais em `degradations`, provando que a exigência só vale para a troca. |
| `TASK-E21-13-AC-3` | o plano estéreo com ganho declarado e um segundo plano que troca de estéreo para mono com o mesmo ganho | os dois passam por `validate` e a linha de canais de cada `degradations` é comparada | as duas direções são aceitas e cada uma escreve a sua própria linha de canais, com o ganho declarado aparecendo no texto. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/degradacao_explicita.rs`, rodado por `cargo test --test degradacao_explicita --locked`, com a linha esperada de `degradations` escrita à mão a partir de `specs/formatos.md` §5 e o valor de `channel_gain_db` conferido no texto em vez de recalculado pelo código sob teste. | sem ganho, `validate` falha no campo da degradação; com ganho, passa e o ganho aparece em `degradations`; o plano que mantém os canais não exige ganho; e as duas direções da troca escrevem linhas distintas. O que ainda não existe hoje: `Degradation` com `channel_gain_db` não existe em `libs/encode/src`, `validate` não existe no crate e `tests/` não existe no repo. |
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
