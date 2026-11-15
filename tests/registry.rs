//! Oráculo independente do registry de encoders.
//!
//! A tabela abaixo é a transcrição à mão da matriz de `specs/formatos.md` §3,
//! feita antes de `src/format.rs` existir. Ela não é gerada de `Format`, não
//! importa a lista de variantes da implementação e não usa `match` sobre o enum:
//! se um formato sumir da enumeração, se a ordem mudar ou se uma variante vier
//! vazia, é esta tabela que reprova. O código só é consultado para o que ele
//! declara — extensão, rótulo lossy e disponibilidade —, nunca para dizer quais
//! formatos existem.

use vynamp_encode::{Availability, Format, Registry};

/// Uma linha de `specs/formatos.md` §3.
struct Row {
    /// Nome do formato em `formatos.md`, só para a mensagem de falha.
    name: &'static str,
    /// Extensão do contêiner, coluna 2 da matriz.
    extension: &'static str,
    /// `true` para Opus, MP3, AAC e Vorbis; `false` para FLAC, WAV, AIFF e ALAC.
    lossy: bool,
}

/// As oito linhas da matriz de §3, na ordem em que a spec as lista.
///
/// A ordem é parte do oráculo: `formats()` tem de enumerar exatamente nesta
/// sequência, porque o converter a apresenta ao usuário nessa ordem.
const MATRIZ: [Row; 8] = [
    Row {
        name: "FLAC",
        extension: "flac",
        lossy: false,
    },
    Row {
        name: "WAV",
        extension: "wav",
        lossy: false,
    },
    Row {
        name: "AIFF",
        extension: "aiff",
        lossy: false,
    },
    Row {
        name: "ALAC",
        extension: "m4a",
        lossy: false,
    },
    Row {
        name: "Opus",
        extension: "opus",
        lossy: true,
    },
    Row {
        name: "MP3",
        extension: "mp3",
        lossy: true,
    },
    Row {
        name: "AAC",
        extension: "m4a",
        lossy: true,
    },
    Row {
        name: "Vorbis",
        extension: "ogg",
        lossy: true,
    },
];

/// As oito variantes de `Format`, uma por linha da matriz, na mesma ordem.
///
/// Lista literal de propósito: é a segunda cópia independente da enumeração, e
/// é ela que pega uma variante a mais ou a menos em `Format` que a matriz não
/// conheça. Se alguém remover uma variante do enum, esta lista para de compilar;
/// se alguém acrescentar uma, o `assert_eq!` de cardinalidade abaixo reprova.
const FORMATOS: [Format; 8] = [
    Format::Flac,
    Format::Wav,
    Format::Aiff,
    Format::Alac,
    Format::Opus,
    Format::Mp3,
    Format::Aac,
    Format::Vorbis,
];

/// `TASK-E21-01-AC-1` / `ENCODE-AC-01`: os oito formatos aparecem em
/// `Registry::with_builtin().formats()`, cada um com `Present{encoder, version}`
/// não vazio ou `Absent{reason}` não vazio, e a tabela transcrita confere com o
/// que a lib declara sobre extensão e rótulo lossy.
#[test]
fn os_oito_formatos_aparecem_com_disponibilidade_nao_vazia() {
    let registry = Registry::with_builtin();
    let enumerados: Vec<(Format, Availability)> = registry.formats().collect();

    assert_eq!(
        enumerados.len(),
        MATRIZ.len(),
        "formats() enumerou {} formatos, a matriz de formatos.md §3 tem {}",
        enumerados.len(),
        MATRIZ.len()
    );

    for (indice, (formato, disponibilidade)) in enumerados.iter().enumerate() {
        let linha = &MATRIZ[indice];
        assert_eq!(
            *formato, FORMATOS[indice],
            "posição {indice}: formats() devolveu {formato:?}, a matriz de §3 lista {}",
            linha.name
        );
        assert_eq!(
            formato.extension(),
            linha.extension,
            "{}: extensão divergente de formatos.md §3",
            linha.name
        );
        assert_eq!(
            formato.is_lossy(),
            linha.lossy,
            "{}: rótulo lossy divergente de formatos.md §3",
            linha.name
        );
        match disponibilidade {
            Availability::Present { encoder, version } => {
                assert!(
                    !encoder.trim().is_empty(),
                    "{}: Present com encoder vazio",
                    linha.name
                );
                assert!(
                    !version.trim().is_empty(),
                    "{}: Present com version vazia",
                    linha.name
                );
            }
            Availability::Absent { reason } => {
                assert!(
                    !reason.trim().is_empty(),
                    "{}: Absent com reason vazio",
                    linha.name
                );
            }
        }
    }

    // A enumeração é finita e não repete formato: os oito da matriz, cada um
    // uma vez. Um `formats()` que devolvesse o mesmo formato duas vezes passaria
    // no laço acima e falharia aqui.
    let mut vistos = FORMATOS.to_vec();
    vistos.sort_by_key(|f| format!("{f:?}"));
    vistos.dedup();
    assert_eq!(vistos.len(), MATRIZ.len(), "formats() repetiu um formato");
}

/// `TASK-E21-01-AC-2` / `ENCODE-AC-10`: `availability(f)` devolve, para cada um
/// dos oito formatos, a mesma variante que `formats()` enumerou para o mesmo
/// formato.
///
/// Os dois laços são separados de propósito: este percorre `FORMATOS` e chama
/// `availability` formato a formato, sem consultar o iterador de `formats()`
/// durante a chamada; o de cima percorre o iterador. Se os dois caminhos
/// divergirem — por exemplo, `availability` respondendo pelo default em vez de
/// pela tabela que `formats()` lê —, é aqui que aparece.
#[test]
fn availability_concorda_com_formats_para_cada_formato() {
    let registry = Registry::with_builtin();
    let enumerados: Vec<(Format, Availability)> = registry.formats().collect();

    for formato in FORMATOS {
        let direto = registry.availability(formato);
        let pela_entrada = enumerados
            .iter()
            .find(|(f, _)| *f == formato)
            .map(|(_, a)| *a)
            .unwrap_or_else(|| panic!("{formato:?} não aparece em formats()"));
        assert_eq!(
            direto, pela_entrada,
            "{formato:?}: availability() devolveu {direto:?}, formats() enumerou {pela_entrada:?}"
        );
    }
}
