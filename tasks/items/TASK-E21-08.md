# TASK-E21-08 — Reembutir metadados e relatar o que foi descartado

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-08` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-08` |
| Critérios da spec | `specs/encode.md` §3, §4 e §5 (ENCODE-AC-08) |
| Fase | M3 |
| Prioridade | P2 |
| Estado | planned |
| Depende de | `TASK-E21-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

Metadados são reembutidos no contêiner de destino; campo que o contêiner não suporta aparece em `MetadataReport::dropped` com motivo, e falha de metadado não aborta o áudio.

## Boundary

- **Não** extrai metadados da origem nem monta o `Metadata` do arquivo de entrada: a extração com as regras do Restore e o re-embedding por arquivo no lote são de `TASK-E08-02`, no converter. Aqui a sessão recebe um `Metadata` pronto e responde o que o contêiner de destino aceitou.

- **Não** decide quais formatos existem na build: a enumeração do registry é de `TASK-E21-01`. Aqui o que varia por formato é só o que cada contêiner comporta, e a tabela citada vem de `specs/formatos.md` §3, não de uma segunda lista de formatos mantida nesta TASK.

- **Não** edita tags de arquivo existente nem define o estado de letras e a persistência de metadados no player: escrever tag sobre fonte existente é de `TASK-E03-09`, no player. Aqui só se escreve no contêiner que esta sessão está criando, e o arquivo de origem nunca é aberto para escrita.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-08 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-08-AC-1` | um `Metadata` com título, artista, o campo desconhecido `X-FOO` e uma capa JPEG pequena, e uma sessão com destino WAV | `set_metadata` é chamado e a sessão termina com `finish`, e o arquivo gerado é decodificado de volta | título e artista estão em `written`; os campos que o WAV/LIST-INFO não comporta estão em `dropped` com motivo; o arquivo de áudio é válido |
| `TASK-E21-08-AC-2` | o mesmo `Metadata` e uma sessão com destino FLAC, que comporta mais campos que o WAV | `set_metadata` é chamado nos dois destinos e os dois `MetadataReport` são comparados campo a campo | o conjunto de `dropped` do FLAC é subconjunto do do WAV, e nenhum campo aparece em `dropped` num destino que o comporta. |
| `TASK-E21-08-AC-3` | um `Metadata` com valor inválido para o contêiner, com byte de controle no valor do título, e um `sink` em memória | `set_metadata` é chamado e, em seguida, a sessão continua a receber blocos de áudio até `finish` | a falha do metadado aparece relatada e não impede o áudio de ser escrito, e o arquivo resultante continua válido. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/metadados.rs`, rodado por `cargo test --test metadados --locked`, que relê o arquivo gerado com `vynamp-decode` e compara as tags escritas com as pedidas, usando como lista de capacidades por contêiner a transcrição de `specs/formatos.md` §3 feita no próprio teste. | título e artista aparecem em `written`; `X-FOO` e o que o WAV/LIST-INFO não comporta aparecem em `dropped` com motivo não vazio; o áudio do arquivo resultante decodifica com a mesma contagem de frames de antes do `set_metadata`; e a lista de capacidades transcrita faz o teste falhar se um campo for descartado num destino que o comporta. O que ainda não existe hoje: `Metadata`, `CoverArt` e `MetadataReport` não existem em `libs/encode/src`, `tests/` não existe no repo, e não há escritor de contêiner nenhum. |
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
