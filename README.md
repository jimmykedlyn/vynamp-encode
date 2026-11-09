# vynamp-encode

**English** · [Português](#português--vynamp-encode)

Audio encoding in Rust, `std` only, with no crates.io dependencies. Depends
optionally on `vynamp-dsp` through the `resample` feature.

> **Status: specification.** This library is not implemented. The repository holds the
> module specification, its task list and the tooling both are built with. It ships no
> running code and makes no measured claim.

Licence: `MIT OR Apache-2.0` — see [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
Security: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-encode/security/advisories/new)
— see [SECURITY.md](SECURITY.md).
Contributing: [CONTRIBUTING.md](CONTRIBUTING.md).

## Scope

Encodes. It does not decode, does not touch an audio device and does not know
the player.

## Builds on its own

```bash
cargo test
```

The agent contract is the umbrella's:
[AGENTS.md](https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md).

# Português — vynamp-encode

[English](#vynamp-encode) · **Português**

Codificação de áudio em Rust, só `std`, sem dependências do crates.io. Depende
opcionalmente de `vynamp-dsp` pela feature `resample`.

> **Estado: especificação.** Este biblioteca não está implementado. O repositório contém
> a especificação do módulo, a lista de tarefas dele e o tooling com que os dois são
> construídos. Não entrega código em execução e não afirma resultado medido.

Licença: `MIT OR Apache-2.0` — ver [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT) e
[LICENSE-APACHE](LICENSE-APACHE).
Segurança: [GitHub Security Advisories](https://github.com/jimmykedlyn/vynamp-encode/security/advisories/new)
— ver [SECURITY.md](SECURITY.md).
Contribuições: [CONTRIBUTING.md](CONTRIBUTING.md).

## Escopo

Codifica. Não decodifica, não toca dispositivo de áudio e não conhece o player.

## Compila sozinho

```bash
cargo test
```

O contrato do agente é o do umbrella:
[AGENTS.md](https://github.com/jimmykedlyn/vynamp/blob/develop/AGENTS.md).
