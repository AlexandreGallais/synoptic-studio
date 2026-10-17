//! `local/kind-in-layer`: a function's kind is allowed in its file's layer
//! (docs/conventions/architecture.md).

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::{LAYERS, Layer};
use crate::source::Source;

/// The rule.
pub struct KindInLayer;

/// The layer holding a file, by the prefix of its path.
fn layer_of(path: &str) -> Option<&'static Layer> {
    LAYERS.iter().find(|layer| {
        path.strip_prefix(layer.path)
            .is_some_and(|rest| rest.starts_with('/'))
    })
}

impl Rule for KindInLayer {
    fn name(&self) -> &'static str {
        "local/kind-in-layer"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let Some(layer) = layer_of(source.path) else {
            return;
        };

        for function in source.top_level_functions() {
            let Some(kind) = source.kind(&function) else {
                continue;
            };
            if !layer.kinds.contains(&kind) {
                let message = format!(
                    "Kind `{kind}` is not allowed in layer `{}` (allowed: {}).",
                    layer.name,
                    layer.kinds.join(", ")
                );
                findings.push(source.finding(self.name(), Source::name_offset(&function), message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::KindInLayer;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code at a path.
    fn found(path: &str, code: &str) -> Vec<String> {
        messages(&lint(&KindInLayer, path, code))
    }

    #[test]
    fn reports_a_kind_the_layer_does_not_allow() {
        assert_eq!(
            found(
                "src/math/vector.ts",
                "/** @kind procedure */\nfunction f(): void {}"
            ),
            ["Kind `procedure` is not allowed in layer `math` (allowed: math)."]
        );
    }

    #[test]
    fn accepts_an_allowed_kind_a_file_outside_the_layers_and_a_missing_kind() {
        let procedure = "/** @kind procedure */\nfunction f(): void {}";

        assert_eq!(
            found(
                "src/math/vector.ts",
                "/** @kind math */\nfunction f(): void {}"
            ),
            NONE
        );
        assert_eq!(found("src/other/vector.ts", procedure), NONE);
        assert_eq!(found("src/mathematics/vector.ts", procedure), NONE);
        assert_eq!(found("src/math/vector.ts", "function f(): void {}"), NONE);
    }
}
