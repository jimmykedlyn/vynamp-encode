//! `TASK-E21-01-AC-3` / `ENCODE-AC-11`: a build padrão não carrega crate de
//! encoder de terceiro, e `vynamp-dsp` aparece só atrás da feature `resample`,
//! desligada por padrão.
//!
//! A prova é o próprio Cargo respondendo: `cargo tree` diz o que entra na
//! resolução sem feature nenhuma, `cargo metadata` diz o que o manifesto
//! declara. O teste confere os dois contra o texto de `Cargo.toml`, que é a
//! terceira cópia — se alguém declarar uma dependência e o `cargo tree` não a
//! mostrar, ou o contrário, o teste reprova.

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

/// `TASK-E21-01-AC-3`: sem feature, o grafo de dependências normais deste crate
/// é só ele mesmo. Nenhum crate de encoder de terceiro entra na build padrão.
#[test]
fn build_padrao_nao_carrega_crate_de_encoder() {
    let arvore = cargo(&["tree", "--locked", "--edges", "normal", "--prefix", "none"]);

    let pacotes: Vec<&str> = arvore
        .lines()
        .map(str::trim)
        .filter(|linha| !linha.is_empty())
        .collect();

    assert_eq!(
        pacotes.len(),
        1,
        "a build padrão tem mais de um pacote no grafo:\n{arvore}"
    );
    assert!(
        pacotes[0].starts_with("vynamp_encode "),
        "o único pacote da build padrão devia ser vynamp_encode, veio: {}",
        pacotes[0]
    );

    // Lista de famílias de encoder que, se aparecessem sem feature, violariam
    // `formatos.md` §3 e `encode.md` §2. A asserção acima já barra qualquer
    // pacote extra; esta nomeia o dano quando ele acontece.
    for proibido in [
        "mp3lame", "lame", "shine", "libopus", "audiopus", "fdk-aac", "vorbis",
    ] {
        assert!(
            !arvore.to_ascii_lowercase().contains(proibido),
            "crate de encoder `{proibido}` entrou na build padrão:\n{arvore}"
        );
    }
}

/// `TASK-E21-01-AC-3`: `vynamp-dsp` é dependência declarada, opcional, presa à
/// feature `resample`, e `resample` não está no conjunto padrão.
///
/// Lê o `cargo metadata` do próprio manifesto e o texto de `Cargo.toml`. A
/// verificação de que ela *entra* quando a feature é ligada é feita pelo teste
/// seguinte, com `cargo tree --features resample`.
#[test]
fn vynamp_dsp_so_entra_atras_da_feature_resample() {
    let metadados = cargo(&["metadata", "--no-deps", "--format-version", "1"]);
    let manifesto = include_str!("../Cargo.toml");

    // Declaração no manifesto: a linha existe, é opcional e vem do repo do módulo.
    let linha_dsp = manifesto
        .lines()
        .find(|linha| linha.starts_with("vynamp_dsp ") || linha.starts_with("vynamp-dsp "))
        .unwrap_or_else(|| panic!("Cargo.toml não declara vynamp_dsp:\n{manifesto}"));
    assert!(
        linha_dsp.contains("optional = true"),
        "vynamp_dsp não é opcional no Cargo.toml: {linha_dsp}"
    );
    assert!(
        linha_dsp.contains("github.com/jimmykedlyn/vynamp-dsp"),
        "vynamp_dsp não aponta para o repo do módulo: {linha_dsp}"
    );

    // A feature existe, liga exatamente essa dependência e não é padrão.
    assert!(
        manifesto.contains("resample"),
        "Cargo.toml não declara a feature resample:\n{manifesto}"
    );
    assert!(
        !manifesto.contains("default = [\"resample\"]")
            && !manifesto.contains("default = [\"dep:vynamp_dsp\"]"),
        "resample (ou vynamp_dsp) está no conjunto padrão de features:\n{manifesto}"
    );

    // `cargo metadata` confirma o que o texto diz: a resolução sem feature
    // padrão não tem vynamp_dsp no grafo, e a feature `resample` existe.
    assert!(
        metadados.contains("\"resample\""),
        "cargo metadata não enxerga a feature resample:\n{metadados}"
    );
    assert!(
        metadados.contains("\"name\":\"vynamp_dsp\""),
        "cargo metadata não enxerga a dependência vynamp_dsp:\n{metadados}"
    );
    assert!(
        metadados.contains("\"optional\":true"),
        "cargo metadata não marca vynamp_dsp como opcional:\n{metadados}"
    );

    // E o grafo resolvido sem feature não contém a dependência.
    let arvore_padrao = cargo(&["tree", "--locked", "--edges", "normal", "--prefix", "none"]);
    assert!(
        !arvore_padrao.contains("vynamp_dsp"),
        "vynamp_dsp entrou na build padrão:\n{arvore_padrao}"
    );
}

/// `TASK-E21-01-AC-3`, a outra metade: com a feature ligada, `vynamp-dsp`
/// aparece. Sem esta metade, uma dependência morta — que nunca entra — também
/// passaria no teste anterior.
#[test]
fn com_a_feature_resample_vynamp_dsp_entra() {
    let arvore = cargo(&[
        "tree",
        "--locked",
        "--edges",
        "normal",
        "--prefix",
        "none",
        "--features",
        "resample",
    ]);

    assert!(
        arvore
            .lines()
            .any(|linha| linha.trim_start().starts_with("vynamp_dsp ")),
        "com --features resample o grafo devia conter vynamp_dsp:\n{arvore}"
    );
}
