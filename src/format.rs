//! Formatos de saída do `vynamp-encode`.
//!
//! A matriz normativa é `specs/formatos.md` §3. Este módulo só carrega o que a
//! matriz diz de cada formato — extensão do contêiner e se ele é lossy —, sem
//! nenhum conhecimento de encoder, arquivo ou dispositivo.

/// Formato de saída suportado pelo produto.
///
/// São exatamente os oito de `specs/formatos.md` §3. Não existe variante para
/// DSD: não há saída DSD (FMT-REQ-07), e o pedido é recusado antes de chegar
/// aqui.
///
/// `#[non_exhaustive]` porque a matriz pode ganhar linha em versão futura sem
/// que isso seja quebra de compatibilidade para quem consome o crate.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum Format {
    /// FLAC, lossless, `.flac`.
    Flac,
    /// WAV, PCM sem compressão, `.wav`.
    Wav,
    /// AIFF, PCM big-endian, `.aiff`.
    Aiff,
    /// ALAC em MP4, lossless, `.m4a`.
    Alac,
    /// Opus, lossy, `.opus`.
    Opus,
    /// MP3, lossy, `.mp3`. Só existe quando o encoder está na build.
    Mp3,
    /// AAC em MP4, lossy, `.m4a`.
    Aac,
    /// Vorbis em Ogg, lossy, `.ogg`.
    Vorbis,
}

impl Format {
    /// Extensão do contêiner, sem o ponto, conforme a coluna 2 da matriz de
    /// `specs/formatos.md` §3.
    ///
    /// ALAC e AAC compartilham `m4a`: os dois são MP4, e quem distingue é o
    /// codec, não a extensão.
    pub fn extension(self) -> &'static str {
        match self {
            Format::Flac => "flac",
            Format::Wav => "wav",
            Format::Aiff => "aiff",
            Format::Alac => "m4a",
            Format::Opus => "opus",
            Format::Mp3 => "mp3",
            Format::Aac => "m4a",
            Format::Vorbis => "ogg",
        }
    }

    /// `true` para os formatos com perda: Opus, MP3, AAC e Vorbis.
    ///
    /// É a única fonte do rótulo lossy usado pelo converter (FMT-REQ-06); quem
    /// precisa do rótulo lê daqui em vez de repetir a lista.
    pub fn is_lossy(self) -> bool {
        match self {
            Format::Flac | Format::Wav | Format::Aiff | Format::Alac => false,
            Format::Opus | Format::Mp3 | Format::Aac | Format::Vorbis => true,
        }
    }
}
