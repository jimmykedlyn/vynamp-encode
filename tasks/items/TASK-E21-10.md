# TASK-E21-10 — Validar o plano contra a matriz de formatos

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-10` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-10` |
| Critérios da spec | `specs/formatos.md` §3, §4 e §5 (FMT-AC-01) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | `TASK-E21-11`, `TASK-E21-12`, `TASK-E21-13`, `TASK-E21-14` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

`validate` aceita exatamente a matriz de `specs/formatos.md` §3 e recusa qualquer combinação fora dela com `InvalidParams` nomeando o campo.

## Boundary

- **Não** decide quando a redução de profundidade é aceita nem o texto de `degradations`: a exigência de `degradation.confirmed` é de `TASK-E21-11`. Aqui `validate` só recusa a profundidade que a matriz de §3 não admite por formato, e confundir as duas regras faria 24→16 bits ser recusado como combinação inválida em vez de pendente de confirmação.

- **Não** escolhe resampler nem aceita mudança de taxa: a confirmação de resample e o campo `resampler` são de `TASK-E21-12`. Esta TASK cobre a faixa de taxas que cada formato admite, não a decisão de mudar a taxa da fonte.

- **Não** é a recusa de DSD: `Format` não tem variante DSF nem DFF e o pedido é recusado antes de montar `EncodeParams`, o que é de `TASK-E21-16`. A confusão é plausível porque as duas terminam em recusa, mas aqui a pergunta é se a combinação de campos está na matriz, e lá é se o formato existe no enum.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-10 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-10-AC-1` | a tabela de casos gerada no próprio teste a partir da matriz de §3, com cada formato e cada valor dentro da faixa mais um valor de cada lado fora dela, incluindo Opus 63 e 257 kbps, FLAC 32 bits e AIFF float | `validate` roda sobre cada caso da tabela gerada e o resultado é comparado com o esperado de cada linha | todo caso dentro passa e todo caso fora volta `InvalidParams` com o `field` correto |
| `TASK-E21-10-AC-2` | os mesmos casos fora da faixa, com o campo que os torna inválidos anotado na tabela gerada antes da chamada | o erro de cada caso é inspecionado quanto ao `field` que nomeia o campo culpado | todo caso fora volta `InvalidParams` com o `field` igual ao anotado, e nenhum erro culpa um campo que estava válido. |
| `TASK-E21-10-AC-3` | um plano com dois campos inválidos ao mesmo tempo, taxa fora da faixa e profundidade não admitida pelo formato | `validate` roda sobre esse plano e o erro é comparado com o de um plano que tem só o primeiro campo inválido | o erro nomeia um campo determinístico, o mesmo entre execuções, e não muda de campo conforme a ordem interna da validação. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/matriz_formatos.rs`, rodado por `cargo test --test matriz_formatos --locked`, com a matriz de §3 transcrita como tabela de teste editada à mão e os casos de borda calculados a partir dela, sem derivar os limites de constantes do código sob teste. | todo caso dentro da matriz passa e todo caso fora volta `InvalidParams` com o `field` correto; os valores de borda citados na spec — Opus 63 e 257 kbps, FLAC 32 bits, AIFF float — são recusados; e um plano com dois erros nomeia sempre o mesmo campo. O que ainda não existe hoje: `validate`, `Format`, `SampleFormat`, `Rate` e `EncodeParams` não existem em `libs/encode/src`, `tests/` não existe no repo, e a matriz de §3 é hoje texto de spec, sem nenhuma transcrição executável. |
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
