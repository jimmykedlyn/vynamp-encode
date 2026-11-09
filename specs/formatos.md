# Spec — vynamp-encode · formatos

| Campo | Valor |
|---|---|
| Módulo | `vynamp-encode` |
| Crate(s) | a definir na autoria |
| Tipo | lib |
| Versão da spec | 0.1.0 |
| Data | 2026-11-08 |
| Estado | esqueleto — §3 a §10 pendentes de autoria |
| Dono | a definir |
| Depende de | `vynamp-dsp` (opcional, feature `resample`) |
| É dependido por | engine; converter |

> **Esqueleto gerado pelo passo 37.** §1, §2 e §7 foram decididos em
> `01-arvore-final.md` §2 e valem como norma. §3 a §6 e §8 a §10 exigem projeto de
> API e **não** foram escritos: inventá-los aqui seria fabricar a autoridade que o SDD
> exige. Estado só passa a `aprovada` com §5 preenchida.
>
> **A fonte normativa é o repositório principal, não este arquivo.** Ver
> [`specs/README.md`](README.md) para o documento canônico deste módulo.

## 1. Propósito

Codificar PCM em arquivo de midia, com muxing e metadados de encoder corretos (priming/padding).

**Não faz:** Nao decodifica, nao toca audio, nao gerencia fila de conversao.

## 2. Fronteira

| Permitido | Proibido |
|---|---|
| herda a fronteira de `specs/encode.md` | herda as proibicoes de `specs/encode.md` |

## 3. API pública

PENDENTE — projeto de API. Só o que estiver aqui é contrato.

## 4. Requisitos

PENDENTE — cada requisito com ID estável `ENCODE-REQ-nn` e REQ de produto,
rastreável até uma TASK e um teste.

## 5. Critérios de aceite

PENDENTE — Given/When/Then, coerente de fora. Enquanto esta seção estiver vazia,
nenhuma TASK deste módulo pode sair de `planned`.

## 6. Erros e cancelamento

PENDENTE — erro é dado, não `panic!`. Cancelamento é explícito, não timeout.

## 7. Fora de escopo

AAC entra sem ressalva; MP3 so quando o encoder estiver na build (C7). UI de conversao e fila persistida sao do converter

## 8. Dependências

Depende de: `vynamp-dsp` (opcional, feature `resample`)

É dependido por: engine; converter

## 9. Testes e fixtures

PENDENTE — oráculo numérico onde couber. Teste que depende de arquivo grande vive
atrás de `#[ignore]` com motivo escrito e variável de ambiente documentada.

## 10. Limitações conhecidas

PENDENTE — limitação declarada não reprova a entrega; limitação omitida reprova.
