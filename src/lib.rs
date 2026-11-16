//! vynamp-encode
//!
//! Codificação de áudio para arquivo de mídia. Específico deste módulo: ver
//! `specs/` no próprio repo.
//!
//! A fronteira é a de `specs/encode.md` §2: só `std`, mais `vynamp-dsp` opcional
//! atrás da feature `resample`. Nenhum tipo de domínio atravessa esta fronteira.

mod format;
mod registry;

pub use format::Format;
pub use registry::{Availability, Registry};

/// Versão do módulo. O contrato de release do umbrella fixa a tag.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
