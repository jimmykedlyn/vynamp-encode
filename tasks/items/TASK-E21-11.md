# TASK-E21-11 — Exigir confirmação na redução de profundidade

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-11` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-11` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §6 (FMT-AC-02) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Redução de profundidade, como 24→16 bits, só é aceita com `degradation.confirmed = true` e aparece em `degradations`.

## Boundary

- **Não** é a matriz de combinações por formato: quais profundidades cada contêiner admite é de `TASK-E21-10`, de que esta depende. Aqui a pergunta é outra, sobre um plano que a matriz já aceita: se a passagem de 24 para 16 bits foi confirmada pelo chamador e declarada no relatório.

- **Não** cobre mudança de canais: a exigência de `channel_gain_db` em mono e estéreo é de `TASK-E21-13`. As duas usam o mesmo bloco `Degradation`, e tratá-las juntas faria o ganho de canal validar a profundidade, que é justamente a troca que o campo `field` do erro precisa distinguir.

- **Não** cobre mudança de taxa: a confirmação de resample e o registro do resampler são de `TASK-E21-12`. A confusão é plausível porque a spec usa o mesmo `confirmed` para as duas degradações, mas o `field` do erro e a linha de `degradations` são distintos e são o que os critérios medem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-11 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-11-AC-1` | uma fonte de 24 bits descrita no plano e um plano FLAC de 16 bits, primeiro com `confirmed = false` e depois com `confirmed = true`, sobre a mesma fonte | `validate` roda sobre os dois planos e os resultados são comparados entre si | o primeiro falha com `InvalidParams{field: "degradation"}`; o segundo passa e `degradations` contém “24→16 bits” |
| `TASK-E21-11-AC-2` | o plano confirmado do critério anterior e um segundo plano FLAC de 16 bits com a fonte já em 16 bits, sem redução nenhuma | os dois passam por `validate` e o vetor `degradations` de cada um é lido | o plano sem redução devolve `degradations` vazio, provando que a linha 24→16 bits só aparece quando há de fato redução. |
| `TASK-E21-11-AC-3` | o plano confirmado e um plano que reduz de 24 para 16 bits com `confirmed = true` mas sem `source_bits` preenchido | `validate` roda sobre o plano incompleto e o erro é inspecionado | a falta da profundidade de origem é recusada com `InvalidParams` no campo da degradação, e não aceita com uma linha de `degradations` inventada. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/degradacao_explicita.rs`, rodado por `cargo test --test degradacao_explicita --locked`, que compara o texto de `degradations` com a linha esperada escrita à mão a partir de `specs/formatos.md` §5, e o `field` do erro com o valor declarado na §6 da mesma spec. | sem confirmação, `validate` devolve `InvalidParams{field: "degradation", ..}`; com confirmação, passa e `degradations` contém a linha de 24→16 bits; o plano sem redução devolve `degradations` vazio; e o plano confirmado sem profundidade de origem é recusado em vez de completado. O que ainda não existe hoje: `validate`, `Degradation` e `ValidatedPlan` não existem em `libs/encode/src`, `tests/` não existe no repo e nenhum dos dois testes citados em `specs/formatos.md` §9 foi escrito. |
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
