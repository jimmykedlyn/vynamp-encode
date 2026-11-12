# TASK-E21-02 — Recusar MP3 ausente com erro tipado e zero byte escrito

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E21-02` |
| Módulo dono | `vynamp-encode` |
| Repo | `jimmykedlyn/vynamp-encode` |
| REQ coberto | `REQ-E21-02` |
| Critérios da spec | `specs/encode.md` §4 e §5 (ENCODE-AC-02) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E21-01`, `TASK-E11-03` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

`Format::Mp3` sem encoder na build devolve `EncodeError::FormatAbsent(Mp3)` em `open`, sem escrever byte no `sink` e sem trocar de formato.

## Boundary

- **Não** reescreve a enumeração de `formats()` nem a variante `Availability`: o registry das oito linhas é de `TASK-E21-01`, de que esta depende. Aqui a entrega é o caminho de recusa que converte `Absent` em `EncodeError::FormatAbsent` dentro de `open`, e refazer a enumeração daria dois donos da mesma tabela.

- **Não** decide se MP3 e AAC entram na build pública nem resolve patente e licença: o inventário de codecs e as políticas de distribuição são de `TASK-E11-03`, em `vynamp`, e esta só consome a conclusão. Nesta lib, quando o encoder estiver na build significa uma feature de compilação, nunca uma permissão consultada a cada uso.

- **Não** é a recusa geral de formato fora da matriz: `TASK-E21-16` recusa DSD porque não existe variante em `Format`, e essa ausência é estrutural. A confusão é plausível porque as duas recusam antes de abrir encoder, mas MP3 existe na matriz e pode faltar na build, enquanto DSD não existe na matriz em build nenhuma.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree --task task-e21-02 --slug <slug>`)
- [ ] Fixtures, features de encoder e hardware disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E21-02-AC-1` | uma build sem a feature do encoder MP3, um `sink` em memória que conta os bytes recebidos e o plano `EncodeParams` com `Format::Mp3` | `open` é chamado com esse plano e, logo depois, `availability(Format::Mp3)` é consultado no mesmo registry | volta `Err(EncodeError::FormatAbsent(Mp3))` e nenhum byte é escrito no `sink` |
| `TASK-E21-02-AC-2` | a mesma chamada com a feature do encoder MP3 ligada, no mesmo teste e no mesmo registry de fábrica | `open` é chamado com `Format::Mp3` e o resultado é comparado com o da build sem a feature | com a feature ligada a chamada não devolve `FormatAbsent`, o que prova que a recusa depende da build e não é incondicional. |
| `TASK-E21-02-AC-3` | um plano com `Format::Mp3` e um `EncodeParams` cujo `format` é o único campo divergente do plano aceito noutro teste | `open` falha e o `sink` é inspecionado depois da falha, comparando o plano pedido com o formato que qualquer escrita teria usado | o formato de saída nunca diverge do pedido, e nenhum byte de cabeçalho de outro contêiner aparece no `sink`. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-encode` | verde no CI Gate |
| Oráculo independente | um teste de integração novo em `tests/mp3_ausente.rs`, rodado duas vezes pelo mesmo comando com e sem a feature, e um `Write` de teste que embrulha um `Vec<u8>` e conta os bytes efetivamente recebidos, comparando o contador antes e depois da chamada. | sem a feature, `open` devolve `Err(EncodeError::FormatAbsent(Format::Mp3))` com o contador de bytes em 0 e `availability` devolvendo `Absent` com motivo não vazio; com a feature ligada a mesma chamada não devolve `FormatAbsent`; em nenhum caso o primeiro byte escrito é de contêiner diferente do pedido. O que ainda não existe hoje: `EncodeError` não existe em `libs/encode/src`, a feature de encoder MP3 não existe no `Cargo.toml`, e `grep -rn FormatAbsent libs/encode/src` devolve 0 linhas. |
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
