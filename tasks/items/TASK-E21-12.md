# TASK-E21-12 — Resample só com confirmação e resampler registrado

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-12` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-12` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §6 (FMT-AC-03) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Mudança de taxa de amostragem só é aceita com confirmação; o resampler é escolhido pelo `vynamp-encode`, atrás da feature `resample`, e registrado em `resampler`.

## Boundary

- **Não** é a faixa de taxas admitidas por formato: a matriz de §3 é de `TASK-E21-10`, de que esta depende. Aqui a entrega é a decisão sobre um plano que a matriz já aceita, ou seja, se a taxa pedida difere da fonte, se a confirmação veio e qual resampler respondeu.

- **Não** é a redução de profundidade: o registro de 24→16 bits em `degradations` é de `TASK-E21-11`, e usar o mesmo `confirmed` não torna as duas a mesma regra. O que esta TASK escreve em `degradations` é a linha de taxa, com o par de valores que a spec fixa.

- **Não** aceita bloco em taxa diferente da declarada: o `BlockMismatch` de um bloco que não bate com o plano é de `TASK-E21-04`, como diz `specs/encode.md` §3. Aqui o resample, quando confirmado, acontece na validação e na sessão sobre o plano declarado, e um bloco fora do plano continua sendo erro.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-12 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-12-AC-1` | uma fonte de 48 kHz descrita no plano, um plano FLAC de 44,1 kHz confirmado e a build com a feature `resample` ligada | `validate` roda sobre o plano e os campos `degradations` e `resampler` do `ValidatedPlan` são lidos | passa, `degradations` contém “48 000→44 100 Hz” e `resampler` é `Some` com nome não vazio |
| `TASK-E21-12-AC-2` | o mesmo plano e a mesma fonte, agora numa build sem a feature `resample` | `validate` roda na build mínima e o erro é comparado com o da build completa | sem a feature, a mudança de taxa é recusada com `InvalidParams` no campo da taxa e motivo que nomeia a ausência do resample, conforme `specs/formatos.md` §6. |
| `TASK-E21-12-AC-3` | o mesmo plano confirmado e um plano em que a taxa pedida é igual à da fonte | os dois passam por `validate` com a feature ligada e os dois `ValidatedPlan` são comparados | o plano sem mudança de taxa devolve `resampler` nulo e nenhuma linha de taxa em `degradations`, provando que o resampler só é escolhido quando há mudança. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/degradacao_explicita.rs`, rodado por `cargo test --test degradacao_explicita --locked --features resample` e repetido sem a feature, comparando `resampler` com `Some(nome)` de nome não vazio e a linha de `degradations` com o par de valores escrito à mão a partir de `specs/formatos.md` §5. | com a feature, o plano confirmado passa, `degradations` contém a linha de 48 000→44 100 Hz e `resampler` é `Some` com nome não vazio; sem a feature, o mesmo plano é recusado no campo da taxa; e sem mudança de taxa `resampler` é nulo. O que ainda não existe hoje: a feature `resample` não existe no `Cargo.toml` do repo, `validate` e `ValidatedPlan` não existem em `libs/encode/src`, e `vynamp-dsp` não está declarado como dependência. |
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
