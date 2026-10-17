//! The three checks `TypeDoc`'s configuration made (`REF-TYPEDOC-VALIDATION`): every
//! export and every property documented, every `{@link}` naming an export, and no internal
//! name reachable from the API. Each failure names its file, its line and its export.

use crate::doc::links;
use crate::model::{Body, Doc, Export, Member, Signature, Target, Type};
use crate::page::Names;

/// The members of the object types within a type, nested ones included.
fn members<'t>(ty: &'t Type, found: &mut Vec<&'t Member>) {
    match ty {
        Type::Object(object) => {
            for member in object {
                found.push(member);
                members(&member.value, found);
            }
        }
        Type::Intersection(parts) | Type::Union(parts) | Type::Tuple(parts) => {
            for part in parts {
                members(part, found);
            }
        }
        Type::Array(inner) | Type::Operator(_, inner) => members(inner, found),
        _ => {}
    }
}

/// Every type an export's declaration writes.
fn types(export: &Export) -> Vec<&Type> {
    fn signature<'s>(signature: &'s Signature, found: &mut Vec<&'s Type>) {
        for parameter in &signature.type_parameters {
            found.extend(parameter.constraint.as_ref());
        }
        found.extend(
            signature
                .parameters
                .iter()
                .map(|parameter| &parameter.value),
        );
        found.push(&signature.returns);
    }

    let mut found = Vec::new();
    match &export.body {
        Body::Function(written) => signature(written, &mut found),
        Body::TypeAlias {
            type_parameters,
            value,
        } => {
            found.extend(
                type_parameters
                    .iter()
                    .filter_map(|each| each.constraint.as_ref()),
            );
            found.push(value);
        }
        Body::Variable { value, .. } => found.push(value),
    }
    found
}

/// The names of the module a type refers to, in order, a name once per place it stands.
fn module_names<'t>(ty: &'t Type, found: &mut Vec<&'t str>) {
    match ty {
        Type::Reference {
            name,
            target,
            arguments,
        } => {
            if *target == Target::Module {
                found.push(name.split('.').next().unwrap_or(name));
            }
            for argument in arguments {
                module_names(argument, found);
            }
        }
        Type::Query { name, target } => {
            if *target == Target::Module {
                found.push(name.split('.').next().unwrap_or(name));
            }
        }
        Type::Array(inner) | Type::Operator(_, inner) => module_names(inner, found),
        Type::Tuple(parts) | Type::Union(parts) | Type::Intersection(parts) => {
            for part in parts {
                module_names(part, found);
            }
        }
        Type::Object(object) => {
            for member in object {
                module_names(&member.value, found);
            }
        }
        Type::Function(written) => {
            for parameter in &written.type_parameters {
                if let Some(constraint) = &parameter.constraint {
                    module_names(constraint, found);
                }
            }
            for parameter in &written.parameters {
                module_names(&parameter.value, found);
            }
            module_names(&written.returns, found);
        }
        Type::IndexedAccess(object, index) => {
            module_names(object, found);
            module_names(index, found);
        }
        Type::Intrinsic(_) | Type::Literal(_) => {}
    }
}

/// Whether a block documents anything: a summary or a tag.
fn documents(doc: Option<&Doc>) -> bool {
    doc.is_some_and(|doc| !doc.summary.is_empty() || !doc.tags.is_empty())
}

/// The texts of a block where a `{@link}` may stand.
fn texts(doc: &Doc) -> impl Iterator<Item = &str> {
    std::iter::once(doc.summary.as_str()).chain(doc.tags.iter().map(|tag| tag.text.as_str()))
}

/// The parameter names a function's `@param` tags must match.
fn parameter_names(export: &Export) -> Option<Vec<&str>> {
    let signature = match &export.body {
        Body::Function(signature) => signature,
        Body::TypeAlias {
            value: Type::Function(signature),
            ..
        } => signature,
        _ => return None,
    };
    Some(
        signature
            .parameters
            .iter()
            .map(|each| each.name.as_str())
            .collect(),
    )
}

/// Every export and every property of its object types documented; every `@param` naming a
/// parameter, as `TypeDoc` warns otherwise.
#[must_use]
pub fn undocumented(exports: &[Export]) -> Vec<String> {
    let mut problems = Vec::new();

    for export in exports {
        let place = format!("{}:{}", export.file, export.line);
        if !documents(export.doc.as_ref()) {
            problems.push(format!("{place}: `{}` has no TSDoc block", export.name));
        }
        let mut found = Vec::new();
        for ty in types(export) {
            members(ty, &mut found);
        }
        for member in found {
            if !documents(member.doc.as_ref()) {
                problems.push(format!(
                    "{}:{}: `{}.{}` has no TSDoc block",
                    export.file, member.line, export.name, member.name
                ));
            }
        }
        if let (Some(names), Some(doc)) = (parameter_names(export), &export.doc) {
            for tag in doc.tags.iter().filter(|tag| tag.name == "param") {
                let target = tag.target.as_deref().unwrap_or("");
                if !names.contains(&target) {
                    problems.push(format!(
                        "{place}: `@param {target}` of `{}` names no parameter",
                        export.name
                    ));
                }
            }
        }
    }

    problems
}

/// Every `{@link}` of a block, the export's and its properties', naming an export.
#[must_use]
pub fn unresolved_links(exports: &[Export], names: &Names) -> Vec<String> {
    let mut problems = Vec::new();

    for export in exports {
        let mut blocks: Vec<(usize, &Doc)> =
            export.doc.iter().map(|doc| (export.line, doc)).collect();
        let mut found = Vec::new();
        for ty in types(export) {
            members(ty, &mut found);
        }
        blocks.extend(
            found
                .iter()
                .filter_map(|member| member.doc.as_ref().map(|doc| (member.line, doc))),
        );

        for (line, doc) in blocks {
            for text in texts(doc) {
                for target in links(text) {
                    if !names.contains_key(&target) {
                        problems.push(format!(
                            "{}:{line}: `{{@link {target}}}` in `{}` names no export",
                            export.file, export.name
                        ));
                    }
                }
            }
        }
    }

    problems
}

/// No name of the library's own reachable from the API without being exported by it.
#[must_use]
pub fn internal(exports: &[Export], names: &Names) -> Vec<String> {
    let mut problems = Vec::new();

    for export in exports {
        let mut found = Vec::new();
        for ty in types(export) {
            module_names(ty, &mut found);
        }
        let mut reported: Vec<&str> = Vec::new();
        for name in found {
            if reported.contains(&name) {
                continue;
            }
            reported.push(name);
            if !names.contains_key(name) {
                problems.push(format!(
                    "{}:{}: `{}` uses `{name}`, which the API does not export",
                    export.file, export.line, export.name
                ));
            }
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::module;
    use crate::names;

    /// The failures of the three checks on one module, in their order.
    fn failures(source: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
        let read = module(source, "a.ts");
        assert_eq!(read.problems, Vec::<String>::new());
        let names = names(&read.exports);
        (
            undocumented(&read.exports),
            unresolved_links(&read.exports, &names),
            internal(&read.exports, &names),
        )
    }

    #[test]
    fn passes_a_documented_module_whose_names_are_exported() {
        let (documented, links, internal) = failures(
            "/** P. */\nexport type Point = {\n  /** X. */\n  readonly x: number;\n};\n/**\n * Uses {@link Point}.\n *\n * @param point - a point\n */\nexport function f<T>(point: Point, items: readonly T[]): Readonly<Point> { return point; }\n",
        );

        assert_eq!(documented, Vec::<String>::new());
        assert_eq!(links, Vec::<String>::new());
        assert_eq!(internal, Vec::<String>::new());
    }

    #[test]
    fn names_an_export_and_a_property_without_doc() {
        let (documented, _, _) = failures(
            "export const A = 1;\n/** B. */\nexport type B = number & {\n  readonly x: { /** Y. */ readonly y: number; readonly z: number };\n};\n",
        );

        assert_eq!(
            documented,
            vec![
                "a.ts:1: `A` has no TSDoc block",
                "a.ts:4: `B.x` has no TSDoc block",
                "a.ts:4: `B.z` has no TSDoc block",
            ]
        );
    }

    #[test]
    fn names_a_param_tag_naming_no_parameter() {
        let (documented, _, _) = failures(
            "/**\n * F.\n *\n * @param x - kept\n * @param y - gone\n */\nexport function f(x: number): number { return x; }\n/**\n * S.\n *\n * @param z - gone\n */\nexport type S = (a: number) => number;\n",
        );

        assert_eq!(
            documented,
            vec![
                "a.ts:7: `@param y` of `f` names no parameter",
                "a.ts:13: `@param z` of `S` names no parameter",
            ]
        );
    }

    #[test]
    fn names_each_link_to_nothing_exported() {
        let (_, links, _) = failures(
            "/**\n * F, see {@link gone}.\n *\n * @see {@link alsoGone}\n */\nexport function f(): void {}\n/** T. */\nexport type T = {\n  /** {@link f} and {@link missing}. */\n  readonly x: number;\n};\n",
        );

        assert_eq!(
            links,
            vec![
                "a.ts:6: `{@link gone}` in `f` names no export",
                "a.ts:6: `{@link alsoGone}` in `f` names no export",
                "a.ts:10: `{@link missing}` in `T` names no export",
            ]
        );
    }

    #[test]
    fn names_each_internal_name_reachable_from_the_api() {
        let (_, _, internal) = failures(
            "import type { Hidden } from \"./hidden\";\ninterface Local { x: number }\nconst VALUE = 1;\n/** F. */\nexport function f<T extends Hidden>(local: Local, value: typeof VALUE, step: (x: Local) => T): Map<Hidden, Hidden>[] { return []; }\n/** C. */\nexport const C: { /** X. */ readonly x: [Hidden][number] } = { x: 1 };\n",
        );

        assert_eq!(
            internal,
            vec![
                "a.ts:5: `f` uses `Hidden`, which the API does not export",
                "a.ts:5: `f` uses `Local`, which the API does not export",
                "a.ts:5: `f` uses `VALUE`, which the API does not export",
                "a.ts:7: `C` uses `Hidden`, which the API does not export",
            ]
        );
    }
}
