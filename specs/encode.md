# Spec — vynamp-encode · encode

| Campo | Valor |
|---|---|
| Módulo | `vynamp-encode` |
| Crate(s) | `vynamp-encode` (módulo `encode`; crate único até uma TASK justificar divisão) |
| Tipo | lib |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `reference/tools/02-vynamp-converter.md` §1 a §6 e das TASKs E21-01 a E21-09, E08-02, E11-03 e E10-03. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-encode` |
| Depende de | `vynamp-dsp` (opcional, feature `resample`) |
| É dependido por | engine; converter |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de
> `reference/tools/02-vynamp-converter.md`, que continua sendo a fonte normativa do
> módulo. Estar escrito não é estar aprovado: até o mantenedor aprovar, nenhum número
> aqui é contrato, e toda TASK que dependa dele continua `planned`. Esta spec cobre o
> contrato do encoder (sessão de escrita, priming/padding, cancelamento); a matriz de
> formatos e parâmetros vive em `formatos.md`.

## 1. Propósito

Codificar PCM em arquivo de midia, com muxing e metadados de encoder corretos (priming/padding).

**Não faz:** Nao decodifica, nao toca audio, nao gerencia fila de conversao.

## 2. Fronteira

| Permitido | Proibido |
|---|---|
| std; `vynamp-dsp` opcional atras de feature `resample` | qualquer crate de dominio `vynamp-*`; qualquer tipo de dominio |

## 3. API pública

Tipos próprios, sem importar nada de `vynamp-decode` nem do engine: o chamador
converte o que tem para estes tipos. Os tipos de formato (`Format`, `EncodeParams`)
são definidos em `formatos.md` e reexportados na raiz.

```rust
/// Bloco de PCM entregue ao encoder: f32 intercalado, faixa nominal [-1.0, 1.0].
/// O encoder não reamostra nem remixa por conta própria (ver ENCODE-REQ-04).
pub struct PcmBlock<'a> {
    pub samples: &'a [f32],      // len = frames * channels
    pub channels: u16,
    pub sample_rate: u32,
}

/// Registry de encoders presentes nesta build. Um formato ausente
/// (ex.: MP3 sem o encoder compilado) aparece como `Availability::Absent`.
pub struct Registry { /* privado */ }

impl Registry {
    pub fn with_builtin() -> Self;
    pub fn availability(&self, format: Format) -> Availability;
    pub fn formats(&self) -> impl Iterator<Item = (Format, Availability)>;
    /// Valida `params` contra `formatos.md` e abre uma sessão sobre `sink`.
    pub fn open<W: std::io::Write + std::io::Seek>(
        &self,
        params: &EncodeParams,
        sink: W,
        cancel: &CancelToken,
    ) -> Result<EncodeSession<W>, EncodeError>;
}

pub enum Availability {
    Present { encoder: &'static str, version: &'static str },
    Absent { reason: &'static str },
}

/// Sessão de escrita de um arquivo. Não é Send entre threads de RT:
/// roda em worker, nunca no callback de áudio.
pub struct EncodeSession<W> { /* privado */ }

impl<W: std::io::Write + std::io::Seek> EncodeSession<W> {
    pub fn write(&mut self, block: PcmBlock<'_>) -> Result<(), EncodeError>;
    pub fn set_metadata(&mut self, tags: &Metadata) -> Result<MetadataReport, EncodeError>;
    /// Esvazia o encoder, escreve padding/trailer e devolve o resumo.
    pub fn finish(self) -> Result<EncodeSummary, EncodeError>;
}

/// Token compartilhado; `cancel()` é idempotente e visto no próximo `write`.
#[derive(Clone)]
pub struct CancelToken { /* Arc<AtomicBool> */ }
impl CancelToken {
    pub fn new() -> Self;
    pub fn cancel(&self);
    pub fn is_cancelled(&self) -> bool;
}

pub struct Metadata {
    pub fields: Vec<(String, String)>,   // chave canônica → valor, ordem preservada
    pub cover: Option<CoverArt>,
}

pub struct CoverArt { pub mime: String, pub data: Vec<u8> }

/// O que o contêiner aceitou e o que ficou de fora — nunca silencioso.
pub struct MetadataReport {
    pub written: Vec<String>,
    pub dropped: Vec<(String, &'static str)>, // (chave, motivo)
}

pub struct EncodeSummary {
    pub format: Format,
    pub frames_in: u64,
    /// Amostras de atraso do encoder declaradas no contêiner (0 para PCM/FLAC/ALAC).
    pub priming: u32,
    /// Amostras de padding no fim, declaradas no contêiner.
    pub padding: u32,
    pub encoder: &'static str,
    pub encoder_version: &'static str,
    pub bytes_written: u64,
    pub lossy: bool,
}
```

Regras de uso que fazem parte do contrato:

- Bloco com `channels` ou `sample_rate` diferente do declarado em `EncodeParams` é
  erro (`EncodeError::BlockMismatch`), não conversão implícita.
- `finish` é a única forma de produzir arquivo válido; `drop` sem `finish` deixa o
  `sink` em estado indefinido, e quem cuida do temporário é o chamador (converter).
- A sessão não abre, renomeia nem apaga arquivo: escreve só no `W` recebido. A
  escrita atômica (temporário → fsync → validação → rename) é do converter.

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| ENCODE-REQ-01 | `vynamp-encode` é a única fonte de encoders do produto; o registry expõe cada formato como presente ou ausente nesta build, com nome e versão do encoder — planned (M3) | encoder registry | E21-01 | — (ENCODE-AC-01 planejado) |
| ENCODE-REQ-02 | MP3 fica `Absent` quando o encoder não está na build, e pedir MP3 nesse caso é erro tipado, nunca troca silenciosa de formato — planned (M3) | política de codecs | E21-02, E11-03 | — (ENCODE-AC-02 planejado) |
| ENCODE-REQ-03 | Formatos lossy declaram no contêiner o priming e o padding do encoder, de modo que um decoder conforme recupere exatamente `frames_in` amostras — planned (M3) | gapless na saída | E21-03 | — (ENCODE-AC-03 planejado) |
| ENCODE-REQ-04 | O encoder não reamostra, não muda profundidade nem remixa canais por conta própria: qualquer degradação é pedida explicitamente nos `EncodeParams` e devolvida no resumo — planned (M3) | sem degradação silenciosa | E21-04 | — (ENCODE-AC-04 planejado) |
| ENCODE-REQ-05 | Formatos lossless (WAV, AIFF, FLAC, ALAC) preservam as amostras: decodificar a saída devolve o PCM de entrada quantizado na profundidade pedida, bit a bit — planned (M3) | integridade lossless | E21-05, E10-03 | — (ENCODE-AC-05 planejado) |
| ENCODE-REQ-06 | Cancelamento por `CancelToken` interrompe a sessão no próximo `write` ou durante `finish`, devolve `EncodeError::Cancelled` e não escreve mais nada no `sink` — planned (M3) | cancelamento | E21-06, E08-02 | — (ENCODE-AC-06 planejado) |
| ENCODE-REQ-07 | Erro de I/O do `sink` (inclusive `StorageFull`) volta como `EncodeError::Io` com o `ErrorKind` original, sem `panic!` — planned (M3) | disco cheio | E21-07, E08-02 | — (ENCODE-AC-07 planejado) |
| ENCODE-REQ-08 | Metadados são reembutidos no contêiner de destino; campo que o contêiner não suporta aparece em `MetadataReport::dropped` com motivo, e falha de metadado não aborta o áudio — planned (M3) | metadados | E21-08 | — (ENCODE-AC-08 planejado) |
| ENCODE-REQ-09 | O resumo marca `lossy = true` para Opus, MP3, AAC e Vorbis; nenhuma API do crate devolve rótulo hi-res ou lossless para saída lossy — planned (M3) | rótulo honesto | E21-09, E08-02 | — (ENCODE-AC-09 planejado) |

Os rótulos da coluna “REQ de produto” são descritivos; o ID de produto correspondente
é o da TASK citada no backlog.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| ENCODE-AC-01 | uma build com o conjunto padrão de features | `Registry::with_builtin().formats()` é lido | os 8 formatos de `formatos.md` aparecem, cada um com `Present{encoder, version}` não vazio ou `Absent{reason}` não vazio |
| ENCODE-AC-02 | uma build sem o encoder MP3 | `open` é chamado com `Format::Mp3` | volta `Err(EncodeError::FormatAbsent(Mp3))` e nenhum byte é escrito no `sink` |
| ENCODE-AC-03 | um seno de 1 kHz, 48 kHz estéreo, 48 000 frames, codificado em Opus 128 kbps e em AAC 192 kbps | cada saída é decodificada por `vynamp-decode` respeitando priming/padding do contêiner | a contagem de frames decodificada é 48 000 exatos e o resumo informa `priming` e `padding` iguais aos lidos do contêiner |
| ENCODE-AC-04 | `EncodeParams` FLAC 24 bits 48 kHz e um bloco de 44,1 kHz | `write` recebe o bloco | volta `Err(BlockMismatch)`; nenhum resample implícito acontece |
| ENCODE-AC-05 | o fixture `sine-24-stereo.flac` decodificado para PCM (4 800 frames, 2 canais) | ele é codificado em WAV 24, AIFF 24, FLAC 24 e ALAC 24 e cada saída é decodificada | o PCM decodificado é idêntico, amostra a amostra, ao de entrada quantizado em 24 bits |
| ENCODE-AC-06 | uma sessão FLAC no meio de um arquivo de 10 s | `cancel()` é chamado entre dois `write` | o `write` seguinte devolve `Cancelled` e `bytes_written` do `sink` não cresce depois disso |
| ENCODE-AC-07 | um `sink` de teste que devolve `io::ErrorKind::StorageFull` na metade | a sessão escreve até o erro | volta `Err(EncodeError::Io{kind: StorageFull, ..})`, sem pânico |
| ENCODE-AC-08 | `Metadata` com título, artista, um campo desconhecido `X-FOO` e capa JPEG, destino WAV | `set_metadata` é chamado e a sessão termina | título e artista estão em `written`; os campos que o WAV/LIST-INFO não comporta estão em `dropped` com motivo; o arquivo de áudio é válido |
| ENCODE-AC-09 | um FLAC 24 bits 96 kHz codificado em Opus 256 kbps | o `EncodeSummary` é lido | `lossy == true` e nenhum campo do resumo contém “hi-res” ou “lossless” |

**Nenhum destes testes existe ainda.** O nome é o compromisso; a TASK citada em §4 é
quem os escreve, e o teste nasce vermelho antes da implementação.

### Critérios que esta spec **não** fixa

- Qualidade perceptual de cada encoder lossy: o relatório a declara como estimativa
  (`02-vynamp-converter.md` §4), e não há limiar de PEAQ/ViSQOL aqui.
- Escolha de biblioteca de cada encoder (C nativa via FFI ou Rust puro): fica para a
  TASK E21-02, que decide o caminho do MP3, e para o inventário de licenças de E11-03.
- Desempenho (tempo real × N): não há meta numérica até haver medição no perfil de
  referência.

## 6. Erros e cancelamento

**Erro é dado, não `panic!`.**

```rust
#[non_exhaustive]
pub enum EncodeError {
    FormatAbsent(Format),
    InvalidParams { field: &'static str, reason: String },
    BlockMismatch { expected_channels: u16, expected_rate: u32, got_channels: u16, got_rate: u32 },
    Io { kind: std::io::ErrorKind, message: String },
    Encoder { encoder: &'static str, message: String },
    Cancelled,
    AlreadyFinished,
}
```

| Situação | Erro | Efeito no `sink` |
|---|---|---|
| Formato pedido não está na build | `FormatAbsent` | nada escrito |
| Parâmetro fora da matriz de `formatos.md` | `InvalidParams` | nada escrito |
| Bloco com canais/taxa diferentes do plano | `BlockMismatch` | o que já foi escrito fica; sessão segue utilizável |
| `Write`/`Seek` falha (disco cheio, permissão) | `Io` | indefinido; o chamador descarta o temporário |
| Biblioteca do encoder reporta falha interna | `Encoder` | indefinido; mapeia para código de saída 5 do converter |
| Token cancelado | `Cancelled` | nada mais é escrito; mapeia para código 4 |
| `write` depois de `finish` | `AlreadyFinished` | nenhum |

**Cancelamento é explícito, não timeout.** O encoder nunca desiste por tempo; quem
decide parar é o chamador, pelo `CancelToken`. O token é verificado a cada `write` e
entre etapas de `finish`, então a latência máxima de cancelamento é a de um bloco.

## 7. Fora de escopo

AAC entra sem ressalva; MP3 so quando o encoder estiver na build (C7). UI de conversao e fila persistida sao do converter

## 8. Dependências

Depende de: `vynamp-dsp` (opcional, feature `resample`)

É dependido por: engine; converter

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| `sine-24-stereo.flac` (48 kHz, 24 bits, 2 canais, 4 800 frames) | fixture do umbrella, hoje em `libs/decode/crates/vynamp-decode-flac/tests/fixtures/` | AC-05, AC-08 |
| seno 1 kHz 48 kHz estéreo, 48 000 frames | gerado no teste | AC-03, AC-06 |
| `sink` que falha com `StorageFull` | `std::io::Write` de teste | AC-07 |

Oráculos: para lossless, igualdade bit a bit após decodificar com `vynamp-decode`
(dev-dependency, não dependência de produção); para lossy, contagem exata de frames
após aplicar priming/padding, que é verificável sem métrica perceptual. Todos os
fixtures são pequenos: não há `#[ignore]`.

```
cargo test --test encode_contrato --locked
cargo test --test lossless_bitexato --locked
cargo test --test gapless_lossy --locked
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/lib.rs` hoje é só `pub const VERSION`; nenhum encoder existe | nenhum critério desta spec pode ser verificado ainda | E21-01 a E21-09 |
| A decisão de qual implementação de MP3/AAC entra na build depende do inventário de licenças | MP3 pode ficar `Absent` em todas as builds públicas | E11-03 |
| Validar a saída exige `vynamp-decode` como dev-dependency, inclusive o adapter Opus ainda planejado | AC-03 para Opus só fecha depois do adapter | E01-07 |
| Qualidade perceptual não tem oráculo numérico | relatório só pode declará-la como estimativa | fora desta spec |
