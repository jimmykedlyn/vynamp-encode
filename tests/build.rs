//! `TASK-E21-01-AC-3` / `ENCODE-AC-11`: a build padrão não carrega crate de
//! encoder de terceiro, e `vynamp-dsp` aparece só atrás da feature `resample`,
//! desligada por padrão.
//!
//! A prova é o próprio Cargo respondendo, e cada asserção é sobre o objeto que o
//! Cargo devolve, não sobre o texto do manifesto:
//!
//! - `cargo tree --edges normal,build` é o grafo resolvido da build padrão. As
//!   arestas `build` entram de propósito: um crate de encoder em
//!   `[build-dependencies]` ainda é crate de terceiro compilado na build padrão,
//!   e `--edges normal` sozinho não o mostraria. `dev` fica de fora porque
//!   dev-dependency não entra na build padrão do crate — `vynamp-decode` será
//!   uma delas em ENCODE-AC-05.
//! - `cargo metadata --no-deps` é o manifesto como o Cargo o interpreta: features
//!   resolvidas e dependências com `optional` e `source` já normalizados. Ler o
//!   JSON evita a fragilidade de `contains` sobre o texto de `Cargo.toml`, que
//!   casa com comentário e só reconhece uma grafia.

use std::path::PathBuf;
use std::process::{Command, Output};

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Roda um subcomando do Cargo no manifesto deste crate e devolve a saída,
/// falhando com o stderr completo se ele não terminar em sucesso.
fn cargo(args: &[&str]) -> String {
    let output: Output = Command::new(env!("CARGO"))
        .args(args)
        .current_dir(manifest_dir())
        .output()
        .expect("executar cargo");
    assert!(
        output.status.success(),
        "cargo {} falhou: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("saída do cargo em UTF-8")
}

/// Nomes dos pacotes do grafo que `cargo tree` resolveu com as arestas pedidas.
///
/// Cada linha de `--prefix none` é `nome versão (origem)`; o primeiro token é o
/// nome do pacote. É o grafo resolvido, não o texto do manifesto.
fn pacotes(args: &[&str]) -> Vec<String> {
    cargo(args)
        .lines()
        .map(str::trim)
        .filter(|linha| !linha.is_empty())
        .filter_map(|linha| linha.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

/// Famílias de crate de encoder de terceiro que, se aparecessem na build padrão,
/// violariam `formatos.md` §3 e `encode.md` §2.
///
/// A comparação é por nome de pacote inteiro ou por prefixo seguido de `-`/`_`,
/// nunca por substring: `contains("ogg")` casaria com `logging`, e uma denylist
/// que reprova por acidente é pior que nenhuma.
const FAMILIAS_DE_ENCODER: [&str; 16] = [
    "mp3lame", "lame", "shine", "minimp3", "libopus", "audiopus", "opus", "fdk-aac", "fdk_aac",
    "vorbis", "lewton", "ogg", "flac", "claxon", "alac", "aac",
];

/// `true` se `nome` é um crate de encoder de terceiro conhecido.
fn eh_crate_de_encoder(nome: &str) -> bool {
    let nome = nome.to_ascii_lowercase();
    FAMILIAS_DE_ENCODER.iter().any(|familia| {
        nome == *familia
            || nome.starts_with(&format!("{familia}-"))
            || nome.starts_with(&format!("{familia}_"))
    })
}

/// `TASK-E21-01-AC-3`: sem feature, o grafo de dependências normais e de build
/// deste crate não tem crate de encoder de terceiro nem outro crate do projeto.
///
/// A asserção é sobre *quais* pacotes entram, não sobre quantos: `thiserror` e
/// qualquer outra dependência legítima podem entrar sem reprovar, que é o que
/// `AGENTS.md` §3 exige (`Result` com `thiserror` por crate) e o que a
/// `TASK-E21-02` vai precisar.
#[test]
fn build_padrao_nao_carrega_crate_de_encoder() {
    let nomes = pacotes(&[
        "tree",
        "--locked",
        "--edges",
        "normal,build",
        "--prefix",
        "none",
    ]);

    assert!(
        nomes.iter().any(|nome| nome == "vynamp_encode"),
        "o grafo da build padrão devia conter vynamp_encode, veio: {nomes:?}"
    );

    for nome in &nomes {
        assert!(
            !eh_crate_de_encoder(nome),
            "crate de encoder `{nome}` entrou na build padrão: {nomes:?}"
        );
        assert!(
            nome == "vynamp_encode" || !nome.starts_with("vynamp_"),
            "crate do projeto `{nome}` entrou na build padrão sem feature: {nomes:?}"
        );
    }
}

/// `TASK-E21-01-AC-3`: `vynamp-dsp` é dependência declarada, opcional, presa à
/// feature `resample`, e `resample` não está no conjunto padrão.
///
/// Lê o `cargo metadata` do próprio manifesto e confere a dependência e a
/// feature no objeto que o Cargo devolve — `optional`, `source` e o conteúdo de
/// `default` e `resample` —, não no texto de `Cargo.toml`. A verificação de que
/// ela *entra* quando a feature é ligada é feita pelo teste seguinte.
#[test]
fn vynamp_dsp_so_entra_atras_da_feature_resample() {
    let metadados: serde_json::Value =
        serde_json::from_str(&cargo(&["metadata", "--no-deps", "--format-version", "1"]))
            .expect("cargo metadata devolve JSON");

    let pacotes_do_manifesto = metadados["packages"]
        .as_array()
        .expect("cargo metadata traz `packages` como lista");
    let pacote = pacotes_do_manifesto
        .iter()
        .find(|pacote| pacote["name"] == "vynamp_encode")
        .expect("cargo metadata devia trazer o pacote vynamp_encode");

    // A dependência existe, é opcional e vem do repo do módulo.
    let dependencias = pacote["dependencies"]
        .as_array()
        .expect("cargo metadata traz `dependencies` como lista");
    let dsp = dependencias
        .iter()
        .find(|dep| dep["name"] == "vynamp_dsp")
        .unwrap_or_else(|| panic!("Cargo.toml não declara vynamp_dsp: {dependencias:?}"));
    assert_eq!(
        dsp["optional"], true,
        "vynamp_dsp não é opcional no manifesto: {dsp}"
    );
    let origem = dsp["source"].as_str().unwrap_or("");
    assert!(
        origem.contains("github.com/jimmykedlyn/vynamp-dsp"),
        "vynamp_dsp não aponta para o repo do módulo: {origem}"
    );

    // A feature existe, liga exatamente essa dependência e não é padrão.
    let features = pacote["features"]
        .as_object()
        .expect("cargo metadata traz `features` como objeto");
    let padrao = features
        .get("default")
        .and_then(|valor| valor.as_array())
        .expect("o manifesto declara a feature `default`");
    assert!(
        padrao.is_empty(),
        "o conjunto padrão de features não pode ligar nada: {padrao:?}"
    );
    let resample = features
        .get("resample")
        .and_then(|valor| valor.as_array())
        .expect("o manifesto declara a feature `resample`");
    assert!(
        resample.iter().any(|item| item == "dep:vynamp_dsp"),
        "a feature `resample` não liga vynamp_dsp: {resample:?}"
    );

    // E o grafo resolvido sem feature não contém a dependência.
    let nomes = pacotes(&[
        "tree",
        "--locked",
        "--edges",
        "normal,build",
        "--prefix",
        "none",
    ]);
    assert!(
        !nomes.iter().any(|nome| nome == "vynamp_dsp"),
        "vynamp_dsp entrou na build padrão: {nomes:?}"
    );
}

/// `TASK-E21-01-AC-3`, a outra metade: com a feature ligada, `vynamp-dsp`
/// aparece. Sem esta metade, uma dependência morta — que nunca entra — também
/// passaria no teste anterior.
#[test]
fn com_a_feature_resample_vynamp_dsp_entra() {
    let nomes = pacotes(&[
        "tree",
        "--locked",
        "--edges",
        "normal,build",
        "--prefix",
        "none",
        "--features",
        "resample",
    ]);

    assert!(
        nomes.iter().any(|nome| nome == "vynamp_dsp"),
        "com --features resample o grafo devia conter vynamp_dsp: {nomes:?}"
    );
}
