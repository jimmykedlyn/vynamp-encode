//! Registry de encoders presentes nesta build.
//!
//! O registry não escolhe encoder nem promete o que a compilação não incluiu:
//! ele relata, formato a formato, se há encoder disponível e qual é. A matriz de
//! formatos é `specs/formatos.md` §3; a API é a de `specs/encode.md` §3.

use crate::format::Format;

/// Disponibilidade de um formato na build em que o crate foi compilado.
///
/// Um formato ausente — MP3 sem o encoder compilado, por exemplo — aparece como
/// [`Availability::Absent`] com o motivo, nunca como erro nem como omissão da
/// lista. Quem transforma ausência em erro tipado é `open`, não o registry.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Availability {
    /// O encoder está nesta build. `encoder` e `version` nunca são vazios.
    Present {
        /// Nome do encoder, como o relatório do converter o apresenta.
        encoder: &'static str,
        /// Versão do encoder, para o relatório e para o suporte.
        version: &'static str,
    },
    /// O formato não tem encoder nesta build. `reason` nunca é vazio.
    Absent {
        /// Por que o formato não está disponível, em texto para o usuário.
        reason: &'static str,
    },
}

/// Os oito formatos de `specs/formatos.md` §3, na ordem da matriz.
///
/// A ordem é contrato: é a ordem em que o converter apresenta os formatos ao
/// usuário. Um formato novo entra aqui e em [`Format`] no mesmo commit.
const ALL: [Format; 8] = [
    Format::Flac,
    Format::Wav,
    Format::Aiff,
    Format::Alac,
    Format::Opus,
    Format::Mp3,
    Format::Aac,
    Format::Vorbis,
];

/// Quantas variantes [`Format`] tem, contado por `match` exaustivo.
///
/// É a garantia de que uma variante nova não passa despercebida: sem braço para
/// ela, este `match` não compila. E o `assert` abaixo amarra a contagem a
/// [`ALL`], então acrescentar a variante ao enum e esquecer de listá-la em `ALL`
/// também não compila — o formato novo não pode existir sem ser enumerado.
const VARIANT_COUNT: usize = match Format::Flac {
    Format::Flac
    | Format::Wav
    | Format::Aiff
    | Format::Alac
    | Format::Opus
    | Format::Mp3
    | Format::Aac
    | Format::Vorbis => 8,
};

const _: () = assert!(ALL.len() == VARIANT_COUNT);

/// Registry de encoders desta build.
///
/// Construído por [`Registry::with_builtin`]; não há outro construtor, porque a
/// disponibilidade é propriedade da compilação e não de um argumento.
pub struct Registry {
    /// Impede a construção fora deste módulo: só `with_builtin` monta um
    /// registry. Hoje não guarda estado — a disponibilidade é `match` sobre o
    /// formato —, mas enquanto o campo existir o construtor é o único caminho e
    /// a decisão de onde a build é consultada fica neste módulo.
    _private: (),
}

impl Registry {
    /// Registry da build atual.
    ///
    /// Nesta build nenhum encoder está compilado: o crate ainda não tem
    /// implementação de encoder, e `Cargo.toml` não declara crate de encoder de
    /// terceiro. Por isso os oito formatos saem `Absent`, cada um com o motivo
    /// que diz qual encoder falta. Quando uma TASK trouxer um encoder, é aqui
    /// que ele passa a `Present` — e o teste de `tests/registry.rs` continua
    /// valendo sem mudança, porque ele aceita as duas variantes.
    pub fn with_builtin() -> Self {
        Registry { _private: () }
    }

    /// Disponibilidade de `format` nesta build.
    ///
    /// Responde para qualquer um dos oito formatos, inclusive os que
    /// [`Registry::formats`] enumera; as duas leituras concordam sempre.
    pub fn availability(&self, format: Format) -> Availability {
        match format {
            Format::Flac => Availability::Absent {
                reason: "encoder FLAC não está nesta build",
            },
            Format::Wav => Availability::Absent {
                reason: "escritor WAV não está nesta build",
            },
            Format::Aiff => Availability::Absent {
                reason: "escritor AIFF não está nesta build",
            },
            Format::Alac => Availability::Absent {
                reason: "encoder ALAC não está nesta build",
            },
            Format::Opus => Availability::Absent {
                reason: "encoder Opus não está nesta build",
            },
            Format::Mp3 => Availability::Absent {
                reason: "encoder MP3 não está nesta build",
            },
            Format::Aac => Availability::Absent {
                reason: "encoder AAC não está nesta build",
            },
            Format::Vorbis => Availability::Absent {
                reason: "encoder Vorbis não está nesta build",
            },
        }
    }

    /// Os oito formatos de `specs/formatos.md` §3, na ordem da matriz, cada um
    /// com a disponibilidade desta build.
    ///
    /// A enumeração é finita e sem repetição: o iterador devolve exatamente oito
    /// entradas, uma por formato.
    pub fn formats(&self) -> impl Iterator<Item = (Format, Availability)> {
        ALL.into_iter()
            .map(|format| (format, self.availability(format)))
    }
}
