# TASK-E21-04 — Recusar resample, remix e mudança de profundidade implícitos

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-04` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-04` |
| Critérios da spec | `specs/encode.md` §3, §4 e §5 (ENCODE-AC-04) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

O encoder não reamostra, não muda profundidade nem remixa canais por conta própria: qualquer degradação é pedida explicitamente nos `EncodeParams` e devolvida no resumo.

## Boundary

- **Não** aceita a mudança de taxa declarada: a regra de que resample exige confirmação e resampler registrado é de `TASK-E21-12`, e a matriz que a sustenta está em `specs/formatos.md` §3. Aqui a entrega é só a recusa quando o bloco não bate com o plano, e aceitar taxa diferente aqui daria dois donos da mesma confirmação.

- **Não** decide a regra de canais na validação: a exigência de `channel_gain_db` para mono e estéreo é de `TASK-E21-13`. Esta TASK cobre o bloco que chega com contagem de canais diferente da declarada, que é erro de uso, e não o plano que pede a troca de layout.

- **Não** prova que a profundidade pedida foi respeitada amostra a amostra: a igualdade bit a bit dos formatos lossless é de `TASK-E21-05`. Aqui a prova é que nenhuma conversão acontece sem pedido, e o resumo declara o que foi feito, o que é observável sem decodificar a saída.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-04 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-04-AC-1` | os `EncodeParams` FLAC 24 bits 48 kHz com 2 canais e um bloco de PCM a 44,1 kHz com a mesma contagem de canais | `write` recebe esse bloco e o `sink` é inspecionado depois da chamada | volta `Err(BlockMismatch)`; nenhum resample implícito acontece |
| `TASK-E21-04-AC-2` | o mesmo plano FLAC 24 bits 48 kHz e um bloco na taxa declarada com o dobro de canais | `write` recebe o bloco com contagem de canais divergente e o erro devolvido é comparado com o do critério anterior | volta `Err(BlockMismatch)` nomeando os canais esperados e recebidos, e a sessão continua utilizável para um bloco correto depois disso. |
| `TASK-E21-04-AC-3` | um plano FLAC 16 bits e um bloco na taxa e nos canais declarados, em sessão nova sobre um `sink` vazio | os blocos são escritos, `finish` é chamado e o `EncodeSummary` é lido | o resumo declara a profundidade efetivamente usada e nenhuma linha de `degradations` aparece sem que o plano tenha pedido, provando que nada mudou por conta do encoder. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/encode_contrato.rs`, rodado por `cargo test --test encode_contrato --locked`, que compara o erro devolvido com a variante `BlockMismatch` e os campos `expected_channels`, `expected_rate`, `got_channels` e `got_rate` declarados na spec, sem reconstruir a decisão a partir do código. | o bloco com taxa divergente devolve `Err(BlockMismatch)` e o `sink` não ganha o bloco; o bloco com canais divergentes devolve o mesmo erro com os quatro campos preenchidos; a sessão segue utilizável depois do erro; e nenhuma degradação aparece no resumo sem pedido explícito. O que ainda não existe hoje: `EncodeSession`, `PcmBlock`, `EncodeSummary` e `BlockMismatch` não existem em `libs/encode/src`, e `tests/` não existe no repo. |
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
