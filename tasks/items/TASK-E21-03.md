# TASK-E21-03 — Declarar priming e padding do encoder no contêiner

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-03` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-03` |
| Critérios da spec | `specs/encode.md` §4 e §5 (ENCODE-AC-03) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E21-01`, `TASK-E01-07` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Formatos lossy declaram no contêiner o priming e o padding do encoder, de modo que um decoder conforme recupere exatamente `frames_in` amostras.

## Boundary

- **Não** define o registry nem a disponibilidade dos formatos: a enumeração por build é de `TASK-E21-01`, de que esta depende. Aqui a entrega é o que a sessão escreve no contêiner e o que `EncodeSummary` devolve em `priming` e `padding`, não a escolha de qual encoder responde.

- **Não** implementa o decoder que lê a saída: a verificação usa o adapter Opus e os codecs modernos de `TASK-E01-07`, que é dependência declarada, e `specs/encode.md` §10 registra que AC-03 para Opus só fecha depois dele. Escrever um leitor de teste aqui provaria o espelho da implementação, não o contêiner.

- **Não** é a prova de bit-exatidão lossless: WAV, AIFF, FLAC e ALAC preservam amostras, e essa prova é de `TASK-E21-05`. Aqui o alvo são os quatro formatos lossy, onde o que engana é justamente o atraso do encoder, e as duas TASKs só compartilham o ato de decodificar a saída.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-03 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-03-AC-1` | um seno de 1 kHz, 48 kHz, 2 canais, 48 000 frames, gerado no próprio teste, e os planos Opus 128 kbps e AAC 192 kbps montados a partir dele | cada saída é decodificada por `vynamp-decode` respeitando o priming e o padding que o contêiner declara, e o `EncodeSummary` é comparado com os valores lidos do arquivo | a contagem de frames decodificada é 48 000 exatos e o resumo informa `priming` e `padding` iguais aos lidos do contêiner |
| `TASK-E21-03-AC-2` | os dois arquivos lossy do critério anterior, com os campos de atraso lidos direto do cabeçalho do contêiner por uma varredura independente da API do crate | a varredura do contêiner é confrontada com `priming` e `padding` do resumo, campo a campo, sem passar pelo decoder | os valores do resumo coincidem com os do contêiner, e nenhum dos dois é zero por omissão quando o encoder de fato declara atraso. |
| `TASK-E21-03-AC-3` | a mesma entrada de 48 000 frames com um bloco final menor que o bloco nominal, forçado pela entrega em pedaços de tamanho irregular | `finish` é chamado depois do último bloco e o arquivo é decodificado até o fim | a contagem de frames decodificada continua 48 000 exatos, sem sobra nem falta de uma amostra no último bloco. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/gapless_lossy.rs`, rodado por `cargo test --test gapless_lossy --locked`, com `vynamp-decode` como dev-dependency e uma varredura própria do cabeçalho do contêiner que não usa nenhuma função do crate sob teste. | as duas saídas decodificam exatamente 48 000 frames, o resumo informa `priming` e `padding` iguais aos lidos do contêiner pela varredura independente, e o bloco final irregular não muda a contagem. O que ainda não existe hoje: nenhum encoder Opus ou AAC existe em `libs/encode/src`, `tests/` não existe no repo, e `specs/encode.md` §10 registra que a verificação de Opus depende do adapter de `TASK-E01-07`, que segue planned. |
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
