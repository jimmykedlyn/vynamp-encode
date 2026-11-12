# TASK-E21-05 — Provar bit-exatidão dos formatos lossless

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-05` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-05` |
| Critérios da spec | `specs/encode.md` §4 e §5 (ENCODE-AC-05) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E21-01`, `TASK-E10-03` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Formatos lossless (WAV, AIFF, FLAC, ALAC) preservam as amostras: decodificar a saída devolve o PCM de entrada quantizado na profundidade pedida, bit a bit.

## Boundary

- **Não** constrói o harness numérico de áudio nem o oráculo de comparação de PCM: a referência independente de medição é de `TASK-E10-03`, no umbrella, e é dependência declarada. Aqui a comparação é amostra a amostra contra o PCM de entrada, e reimplementar o harness daria dois donos do mesmo critério numérico.

- **Não** cobre priming e padding de formato lossy: a contagem de frames depois do atraso do encoder é de `TASK-E21-03`. A confusão é plausível porque as duas decodificam a saída e contam frames, mas em WAV, AIFF, FLAC e ALAC não há atraso a declarar, e o resumo devolve `priming` e `padding` zero por contrato da spec.

- **Não** é a recusa de conversão implícita: o erro de bloco que não bate com o plano é de `TASK-E21-04`. Aqui o plano já foi aceito e a pergunta é outra, sobre se as amostras que entraram são exatamente as que saem depois de decodificar o arquivo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-05 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-05-AC-1` | o fixture `sine-24-stereo.flac` já citado em `specs/encode.md` §9, decodificado para PCM de 4 800 frames em 2 canais, com o SHA-256 do arquivo de origem registrado antes do teste | o PCM é codificado em WAV 24, AIFF 24, FLAC 24 e ALAC 24, cada saída é decodificada e as quatro são comparadas com o PCM de entrada quantizado em 24 bits | o PCM decodificado é idêntico, amostra a amostra, ao de entrada quantizado em 24 bits |
| `TASK-E21-05-AC-2` | as mesmas quatro saídas do critério anterior e o mesmo PCM de entrada, agora comparados byte a byte pelo CRC32 do bloco de amostras | o CRC32 de cada saída decodificada é calculado e confrontado com o CRC32 do PCM de entrada quantizado em 24 bits | os cinco CRC32 são idênticos, o que prova que a igualdade vale para todas as amostras e não só para as bordas do arquivo. |
| `TASK-E21-05-AC-3` | um trecho com o maior e o menor valor representável em 24 bits e um silêncio digital absoluto, gerado no teste e não presente no fixture | o trecho é codificado nos quatro formatos lossless e decodificado de volta | os valores extremos e o silêncio voltam idênticos, sem clipping e sem dithered, provando que a profundidade pedida foi respeitada nas bordas da faixa dinâmica. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/lossless_bitexato.rs`, rodado por `cargo test --test lossless_bitexato --locked`, com `vynamp-decode` como dev-dependency e `sha2` sobre o PCM de entrada, mais um CRC32 calculado no próprio teste sobre o bloco decodificado — nenhum dos dois usa o caminho de escrita do crate sob teste para derivar o valor esperado. | o PCM decodificado de cada um dos quatro formatos é idêntico, amostra a amostra, ao de entrada quantizado em 24 bits; os cinco CRC32 coincidem; os valores extremos e o silêncio voltam sem alteração. O que ainda não existe hoje: nenhuma das quatro variantes de `Format` existe em `libs/encode/src`, `tests/` não existe no repo, e `specs/encode.md` §10 registra que `src/lib.rs` tem apenas 6 linhas e só `pub const VERSION`. |
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
