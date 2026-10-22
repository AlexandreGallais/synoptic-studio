//! The project's values the rules read: function kinds and their limits (ADR-0011), the layers
//! and the kinds each allows, the naming vocabularies, where `@see` targets are resolved, and the
//! folder signal (ADR-0043). Moved from `eslint/settings/` with the rules (EN-075).

/// Limits of one function kind: lines of code, statements, `McCabe` complexity, block depth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KindLimit {
    /// The kind's name, as `@kind` writes it.
    pub kind: &'static str,
    /// Most decision points plus one.
    pub max_complexity: usize,
    /// Most nested blocks.
    pub max_depth: usize,
    /// Most lines holding code.
    pub max_lines: usize,
    /// Most statements.
    pub max_statements: usize,
}

/// Function kinds and their limits, in pipeline order (ADR-0011, docs/conventions/functions.md);
/// changed only with an ADR.
pub const KIND_LIMITS: [KindLimit; 5] = [
    limit("math", 3, 1, 10, 4),
    limit("geometry", 5, 2, 20, 8),
    limit("domain", 5, 2, 20, 8),
    limit("format", 4, 1, 20, 8),
    limit("procedure", 3, 1, 40, 15),
];

/// One row of [`KIND_LIMITS`].
const fn limit(
    kind: &'static str,
    max_complexity: usize,
    max_depth: usize,
    max_lines: usize,
    max_statements: usize,
) -> KindLimit {
    KindLimit {
        kind,
        max_complexity,
        max_depth,
        max_lines,
        max_statements,
    }
}

/// The names of the kinds, in pipeline order.
#[must_use]
pub fn kind_names() -> Vec<&'static str> {
    KIND_LIMITS.iter().map(|each| each.kind).collect()
}

/// A layer of the library, by its folder, the kinds its functions may carry and the layers it
/// may import (docs/conventions/architecture.md; its imports are checked by
/// `import-x/no-restricted-paths`, moved from `eslint/settings/layers.ts` by EN-084).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layer {
    /// The layer's name.
    pub name: &'static str,
    /// Its folder from the root.
    pub path: &'static str,
    /// The kinds it allows.
    pub kinds: &'static [&'static str],
    /// The library's layers it may import; the playground imports the public API only.
    pub imports: &'static [&'static str],
}

/// The layers, from the functional core to the playground.
pub const LAYERS: [Layer; 8] = [
    layer("math", "src/math", &["math"], &[]),
    layer("geometry", "src/geometry", &["math", "geometry"], &["math"]),
    layer("model", "src/model", &["domain"], &["math", "geometry"]),
    layer(
        "routing",
        "src/routing",
        &["geometry", "domain"],
        &["math", "geometry", "model"],
    ),
    layer("io", "src/io", &["format"], &["math", "geometry", "model"]),
    layer(
        "render",
        "src/render",
        &["format", "procedure"],
        &["math", "geometry", "model", "routing", "io"],
    ),
    layer(
        "interaction",
        "src/interaction",
        &["domain", "procedure"],
        &["math", "geometry", "model", "routing", "io", "render"],
    ),
    layer("playground", "playground", &["procedure"], &[]),
];

/// The library's entry point: the only module the playground may import.
pub const PUBLIC_API_PATH: &str = "src/index.ts";

/// One row of [`LAYERS`].
const fn layer(
    name: &'static str,
    path: &'static str,
    kinds: &'static [&'static str],
    imports: &'static [&'static str],
) -> Layer {
    Layer {
        name,
        path,
        kinds,
        imports,
    }
}

/// Leading words reserved for functions returning a boolean.
pub const PREDICATE_PREFIXES: [&str; 9] = [
    "are",
    "can",
    "contains",
    "equals",
    "has",
    "includes",
    "intersects",
    "is",
    "should",
];

/// Leading words of boolean variables and parameters (`naming-convention`, docs/conventions/naming.md).
pub const BOOLEAN_PREFIXES: [&str; 9] = [
    "are", "can", "did", "does", "has", "is", "should", "was", "will",
];

/// Vague leading verbs forbidden on the pure kinds: the name states the concept itself.
pub const VAGUE_PREFIXES: [&str; 9] = [
    "calc",
    "calculate",
    "compute",
    "do",
    "get",
    "handle",
    "make",
    "perform",
    "process",
];

/// The leading words of a `format` function read as a conversion: `toX`, `parseX`…
pub const FORMAT_PREFIXES: [&str; 7] = [
    "to",
    "from",
    "parse",
    "serialize",
    "format",
    "encode",
    "decode",
];

/// Imperative verbs a `procedure` name may start with (docs/conventions/naming.md): add a verb,
/// sorted, when a new action appears, rather than bending a name.
pub const PROCEDURE_VERBS: [&str; 51] = [
    "add",
    "append",
    "apply",
    "attach",
    "bind",
    "choose",
    "clear",
    "click",
    "close",
    "commit",
    "create",
    "detach",
    "dispatch",
    "draw",
    "emit",
    "execute",
    "focus",
    "hide",
    "init",
    "insert",
    "listen",
    "load",
    "log",
    "mount",
    "move",
    "open",
    "press",
    "read",
    "redo",
    "register",
    "remove",
    "render",
    "replace",
    "reset",
    "run",
    "save",
    "schedule",
    "select",
    "set",
    "show",
    "start",
    "stop",
    "subscribe",
    "toggle",
    "type",
    "undo",
    "unmount",
    "unsubscribe",
    "update",
    "wait",
    "write",
];

/// Where `@see` targets are resolved, from the root (docs/conventions/documentation.md).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReferenceSources {
    /// The folder of the ADRs.
    pub adr_directory: &'static str,
    /// The folder of the derivations.
    pub derivations_directory: &'static str,
    /// The bibliography.
    pub references_file: &'static str,
    /// The mark of a row not yet verified.
    pub unverified_marker: &'static str,
}

/// The project's reference sources.
pub const REFERENCE_SOURCES: ReferenceSources = ReferenceSources {
    adr_directory: "docs/adr",
    derivations_directory: "docs/derivations",
    references_file: "docs/references.md",
    unverified_marker: "[unverified]",
};

/// Most modules a folder holds before it asks for a domain to split it (ADR-0043).
pub const FOLDER_SIGNAL: usize = 15;

/// Folders past the signal, each listed with its reason in `docs/conventions/files.md`.
pub const FOLDERS_PAST_SIGNAL: [&str; 2] = ["playground/user-actions", "playground/checklists"];

/// Folders exempt from a barrel: the root, the site's theme, the repository's tests.
pub const FOLDERS_WITHOUT_INDEX: [&str; 3] = [".", "docs/.vitepress", "tests"];

#[cfg(test)]
mod tests {
    use super::{FOLDER_SIGNAL, FOLDERS_PAST_SIGNAL, LAYERS, PROCEDURE_VERBS, kind_names};

    #[test]
    fn keeps_the_verbs_sorted_and_once_each() {
        assert!(PROCEDURE_VERBS.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn names_the_kinds_in_pipeline_order_and_allows_only_known_kinds() {
        let kinds = kind_names();

        assert_eq!(kinds, ["math", "geometry", "domain", "format", "procedure"]);
        assert!(
            LAYERS
                .iter()
                .all(|layer| layer.kinds.iter().all(|kind| kinds.contains(kind)))
        );
    }

    /// The rows of the table of folders past the signal in `docs/conventions/files.md`.
    fn listed_in_the_page() -> Vec<String> {
        let page = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../docs/conventions/files.md"),
        )
        .unwrap_or_default();
        let heading = format!("### Folders past {FOLDER_SIGNAL} modules");
        let table = page.split(&heading).nth(1).unwrap_or_default();

        table
            .lines()
            .skip_while(|line| !line.starts_with('|'))
            .take_while(|line| line.starts_with('|'))
            .filter_map(|line| line.strip_prefix("| `"))
            .filter_map(|line| line.split('`').next())
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn f11_ac4_lists_in_the_page_the_folders_the_lint_lets_past_the_signal() {
        assert_eq!(listed_in_the_page(), FOLDERS_PAST_SIGNAL);
    }
}
