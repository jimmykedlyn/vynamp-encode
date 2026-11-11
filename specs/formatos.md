# Spec — vynamp-encode · formatos

| Campo | Valor |
|---|---|
| Módulo | `vynamp-encode` |
| Crate(s) | `vynamp-encode` (módulo `formatos`; crate único até uma TASK justificar divisão) |
| Tipo | lib |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `reference/tools/02-vynamp-converter.md` §1, §3 e §4 e das TASKs E18-02, E08-02 e E11-03. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-encode` |
| Depende de | `vynamp-dsp` (opcional, feature `resample`) |
| É dependido por | engine; converter |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de
> `reference/tools/02-vynamp-converter.md` §3, que continua sendo a fonte normativa da
> matriz. Estar escrito não é estar aprovado: até o mantenedor aprovar, nenhum número
> aqui é contrato, e toda TASK que dependa dele continua `planned`. Esta spec cobre a
> matriz de formatos, os limites de parâmetro e a regra de degradação explícita; a
> sessão de escrita vive em `encode.md`.

## 1. Propósito

Codificar PCM em arquivo de midia, com muxing e metadados de encoder corretos (priming/padding).

**Não faz:** Nao decodifica, nao toca audio, nao gerencia fila de conversao.

## 2. Fronteira

| Permitido | Proibido |
|---|---|
| herda a fronteira de `specs/encode.md` | herda as proibicoes de `specs/encode.md` |

## 3. API pública

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum Format { Flac, Wav, Aiff, Alac, Opus, Mp3, Aac, Vorbis }

impl Format {
    pub fn extension(self) -> &'static str;   // "flac", "wav", "aiff", "m4a", "opus", "mp3", "m4a", "ogg"
    pub fn is_lossy(self) -> bool;            // true para Opus, Mp3, Aac, Vorbis
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SampleFormat { Int16, Int24, Int32, Float32 }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rate { Cbr(u32), Vbr(u32), Quality(f32) } // kbps ou qualidade Vorbis

/// Plano completo de um arquivo de saída. Degradação só por campo explícito.
pub struct EncodeParams {
    pub format: Format,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: Option<SampleFormat>, // só lossless
    pub rate: Option<Rate>,                  // só lossy
    pub degradation: Degradation,
}

/// Mudanças em relação à fonte que o chamador declarou e confirmou.
#[derive(Default)]
pub struct Degradation {
    pub source_rate: Option<u32>,
    pub source_bits: Option<u8>,
    pub source_channels: Option<u16>,
    pub confirmed: bool,
    /// Ganho aplicado na mudança mono↔estéreo, em dB, declarado.
    pub channel_gain_db: Option<f32>,
}

/// Valida um plano contra a matriz desta spec, sem abrir encoder.
pub fn validate(params: &EncodeParams) -> Result<ValidatedPlan, EncodeError>;

pub struct ValidatedPlan {
    pub params: EncodeParams,
    /// Linhas legíveis para o relatório do converter (“24→16 bits, confirmado”).
    pub degradations: Vec<String>,
    /// Resampler escolhido pelo vynamp-encode, se houver mudança de taxa.
    pub resampler: Option<&'static str>,
}
```

### Matriz de formatos

| Formato | Contêiner | Profundidade / taxa de bits | Observação |
|---|---|---|---|
| FLAC | `.flac` | 16 ou 24 bits | lossless |
| WAV | `.wav` | 16/24/32 int; 32 float | sem conversão implícita float→int |
| AIFF | `.aiff` | 16 ou 24 bits, big-endian | lossless |
| ALAC | `.m4a` (MP4) | 16 ou 24 bits | lossless |
| Opus | `.opus` | 64–256 kbps VBR | lossy; saída interna 48 kHz |
| MP3 | `.mp3` | 128–320 kbps CBR ou VBR | lossy; só se o encoder estiver na build |
| AAC | `.m4a` | 96–256 kbps | lossy; sem ressalva de build |
| Vorbis | `.ogg` | VBR por qualidade | lossy |

Não há saída DSD. Entrada DSD chega ao encoder já convertida em PCM pelo chamador.

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| FMT-REQ-01 | `validate` aceita exatamente a matriz de §3 e recusa qualquer combinação fora dela com `InvalidParams` nomeando o campo — planned (M3) | matriz de formatos | E18-02 | — (FMT-AC-01 planejado) |
| FMT-REQ-02 | Redução de profundidade (ex.: 24→16) só é aceita com `degradation.confirmed = true` e aparece em `degradations` — planned (M3) | degradação explícita | E18-02 | — (FMT-AC-02 planejado) |
| FMT-REQ-03 | Mudança de taxa de amostragem só é aceita com confirmação; o resampler é escolhido pelo `vynamp-encode` (feature `resample`) e registrado em `resampler` — planned (M3) | resample declarado | E18-02 | — (FMT-AC-03 planejado) |
| FMT-REQ-04 | Mudança mono↔estéreo é explícita e exige `channel_gain_db` declarado — planned (M3) | canais explícitos | E18-02 | — (FMT-AC-04 planejado) |
| FMT-REQ-05 | WAV float 32 nunca vira inteiro sem pedido explícito de `SampleFormat` inteiro com confirmação — planned (M3) | sem conversão implícita | E18-02 | — (FMT-AC-05 planejado) |
| FMT-REQ-06 | `Format::is_lossy` é verdadeiro para Opus, MP3, AAC e Vorbis, e é a única fonte do rótulo lossy usado pelo converter — planned (M3) | rótulo honesto | E18-02, E08-02 | — (FMT-AC-06 planejado) |
| FMT-REQ-07 | Não existe saída DSD: `Format` não tem variante DSF/DFF, e o pedido é recusado como erro de uso antes de chegar à lib — planned (M3) | sem DSD na saída | E18-02 | — (FMT-AC-07 planejado) |

Os rótulos da coluna “REQ de produto” são descritivos; o ID de produto correspondente
é o da TASK citada no backlog.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| FMT-AC-01 | a tabela de casos gerada da matriz de §3: cada formato × cada valor dentro e um valor de cada lado fora da faixa (ex.: Opus 63 e 257 kbps, FLAC 32 bits, AIFF float) | `validate` roda sobre cada caso | todo caso dentro passa e todo caso fora volta `InvalidParams` com o `field` correto |
| FMT-AC-02 | fonte 24 bits, plano FLAC 16 bits, `confirmed = false` e depois `true` | `validate` roda nos dois | o primeiro falha com `InvalidParams{field: "degradation"}`; o segundo passa e `degradations` contém “24→16 bits” |
| FMT-AC-03 | fonte 48 kHz, plano FLAC 44,1 kHz, confirmado, build com feature `resample` | `validate` roda | passa, `degradations` contém “48 000→44 100 Hz” e `resampler` é `Some` com nome não vazio |
| FMT-AC-04 | fonte mono, plano estéreo, com e sem `channel_gain_db` | `validate` roda | sem ganho falha; com ganho passa e o ganho aparece em `degradations` |
| FMT-AC-05 | fonte WAV float 32, plano WAV `Int24` sem confirmação | `validate` roda | falha com `InvalidParams{field: "sample_format"}` |
| FMT-AC-06 | os 8 valores de `Format` | `is_lossy` e `extension` são lidos | Opus, Mp3, Aac e Vorbis são lossy, os outros quatro não; as extensões batem com §3 |
| FMT-AC-07 | um plano que pede DSF ou DFF como saída | o converter tenta montar `EncodeParams` | não existe variante `Format` para DSD e o parsing do formato devolve erro de uso |

**Nenhum destes testes existe ainda.** O nome é o compromisso; a TASK citada em §4 é
quem os escreve, e o teste nasce vermelho antes da implementação.

### Critérios que esta spec **não** fixa

- Qual resampler é usado e sua qualidade: é escolha do `vynamp-encode` sobre o
  `vynamp-dsp`, só precisa ser registrada.
- Valores padrão de bitrate por formato: são do CLI do converter
  (`02-vynamp-converter.md` §2), não da lib.
- Dither na redução de profundidade: a spec exige declaração, não o algoritmo.

## 6. Erros e cancelamento

**Erro é dado, não `panic!`.** `validate` usa o mesmo `EncodeError` de `encode.md`
§6; os casos desta spec são:

| Situação | Erro |
|---|---|
| Combinação fora da matriz | `InvalidParams { field, reason }` |
| Degradação sem `confirmed` | `InvalidParams { field: "degradation", .. }` |
| Mudança de taxa sem feature `resample` | `InvalidParams { field: "sample_rate", reason: "resample indisponível nesta build" }` |
| Formato fora da build | `FormatAbsent` |

**Cancelamento é explícito, não timeout.** `validate` é puro e síncrono, não tem o
que cancelar; o cancelamento da escrita está em `encode.md` §6.

## 7. Fora de escopo

AAC entra sem ressalva; MP3 so quando o encoder estiver na build (C7). UI de conversao e fila persistida sao do converter

## 8. Dependências

Depende de: `vynamp-dsp` (opcional, feature `resample`)

É dependido por: engine; converter

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| tabela de casos da matriz | gerada no teste a partir de §3 | FMT-AC-01, FMT-AC-06 |
| planos de degradação | literais no teste | FMT-AC-02 a FMT-AC-05 |

Oráculo: a própria matriz de §3, transcrita como tabela de teste; não há áudio
envolvido, então não há `#[ignore]`.

```
cargo test --test matriz_formatos --locked
cargo test --test degradacao_explicita --locked --features resample
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| Nenhum tipo desta spec existe em `src/` | nenhum critério verificável hoje | E18-02 |
| Sem a feature `resample`, toda mudança de taxa é recusada | build mínima só converte mantendo a taxa | decisão de build em E18-02 |
| Opus codifica internamente a 48 kHz; outra taxa pedida passa por resample | o plano declara a mudança mesmo quando o usuário não pediu taxa | registrado em `degradations` |
