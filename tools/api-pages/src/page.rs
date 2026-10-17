//! One export's page, in the Markdown `TypeDoc`'s Markdown plugin wrote (`REF-TYPEDOC-MARKDOWN`):
//! a breadcrumb, a title, the signature, `Defined in`, the summary, then one section per type
//! parameter, parameter, return value, property and remaining block tag.

use std::collections::BTreeMap;

use crate::model::{
    Body, Doc, Export, Kind, Member, Parameter, Signature, Target, Type, TypeParameter,
};
use crate::order::sorted_by_name;

/// The exports by name, to link a type to its page.
pub type Names = BTreeMap<String, Kind>;

/// The tags shown in their own sections before the remaining ones.
const SHOWN_TAGS: [&str; 3] = ["param", "returns", "template"];

/// Text escaped for Markdown, outside code: `_`, `*`, `<`, `>`, `[`, `]` and `|`.
#[must_use]
pub fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if matches!(character, '_' | '*' | '<' | '>' | '[' | ']' | '|') {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

/// The path from a page of one kind to the page of an export.
fn href(from: Kind, to: Kind, name: &str) -> String {
    if from == to {
        format!("{name}.md")
    } else {
        format!("../{}/{name}.md", to.folder())
    }
}

/// The renderer of one page: the exports it may link to and the kind of the page.
pub struct Page<'n> {
    names: &'n Names,
    kind: Kind,
}

impl<'n> Page<'n> {
    /// A renderer for the page of an export of a given kind.
    #[must_use]
    pub fn new(names: &'n Names, kind: Kind) -> Self {
        Self { names, kind }
    }

    /// A name as code, linked to its page when it is an export of the module's.
    fn name(&self, name: &str, target: Target) -> String {
        match (target, self.names.get(name)) {
            (Target::Module, Some(kind)) => format!("[`{name}`]({})", href(self.kind, *kind, name)),
            _ => format!("`{name}`"),
        }
    }

    /// A type as `TypeDoc`'s Markdown plugin writes it.
    #[must_use]
    pub fn ty(&self, ty: &Type) -> String {
        match ty {
            Type::Intrinsic(name) | Type::Literal(name) => format!("`{name}`"),
            Type::Reference {
                name,
                target,
                arguments,
            } => {
                let text = self.name(name, *target);
                if arguments.is_empty() {
                    text
                } else {
                    format!("{text}\\<{}\\>", self.list(arguments, ", "))
                }
            }
            Type::Query { name, target } => format!("*typeof* {}", self.name(name, *target)),
            Type::Array(element) => match **element {
                Type::Union(_) | Type::Intersection(_) | Type::Function(_) => {
                    format!("({})[]", self.ty(element))
                }
                _ => format!("{}[]", self.ty(element)),
            },
            Type::Tuple(elements) => format!("\\[{}\\]", self.list(elements, ", ")),
            Type::Union(types) => self.list(types, " \\| "),
            Type::Intersection(types) => self.list(types, " & "),
            Type::Object(_) => "`object`".to_owned(),
            Type::Function(signature) => format!(
                "({}) => {}",
                parameter_list(&signature.parameters),
                self.ty(&signature.returns)
            ),
            Type::IndexedAccess(object, index) => {
                format!("{}\\[{}\\]", self.ty(object), self.ty(index))
            }
            Type::Operator(operator, inner) => format!("{operator} {}", self.ty(inner)),
        }
    }

    /// Types joined by a separator.
    fn list(&self, types: &[Type], separator: &str) -> String {
        types
            .iter()
            .map(|each| self.ty(each))
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// A member's declaration line: `` > `readonly` **name**: type ``.
    fn member_line(&self, member: &Member) -> String {
        let readonly = if member.readonly { "`readonly` " } else { "" };
        let optional = if member.optional { "?" } else { "" };
        format!(
            "> {readonly}**{}**{optional}: {}",
            escape(&member.name),
            self.ty(&member.value)
        )
    }

    /// The `## Properties` of an object type alias, each with its place, between rules.
    fn properties(&self, out: &mut Vec<String>, members: &[Member], file: &str) {
        out.push("## Properties".to_owned());
        let sorted = sorted_by_name(members, |member| &member.name);
        for (index, member) in sorted.iter().enumerate() {
            if index > 0 {
                out.push("***".to_owned());
            }
            out.push(format!("### {}", escape(&member.name)));
            out.push(self.member_line(member));
            out.push(format!("Defined in: {file}:{}", member.line));
            push_summary(out, member.doc.as_ref());
        }
    }

    /// The `## Type Declaration` of an inline object, its nested objects one level down.
    fn declaration(&self, out: &mut Vec<String>, members: &[Member], level: usize, prefix: &str) {
        if level == 3 {
            out.push("## Type Declaration".to_owned());
        }
        for member in sorted_by_name(members, |member| &member.name) {
            let heading = "#".repeat(level);
            out.push(format!("{heading} {prefix}{}", escape(&member.name)));
            out.push(self.member_line(member));
            push_summary(out, member.doc.as_ref());
            if let Type::Object(inner) = &member.value {
                let prefix = format!("{prefix}{}.", escape(&member.name));
                self.declaration(out, inner, level + 1, &prefix);
            }
        }
    }

    /// The `## Type Parameters` section.
    fn type_parameters(
        &self,
        out: &mut Vec<String>,
        parameters: &[TypeParameter],
        doc: Option<&Doc>,
    ) {
        if parameters.is_empty() {
            return;
        }
        out.push("## Type Parameters".to_owned());
        for parameter in parameters {
            out.push(format!("### {}", escape(&parameter.name)));
            out.push(match &parameter.constraint {
                Some(constraint) => {
                    format!("`{}` *extends* {}", parameter.name, self.ty(constraint))
                }
                None => format!("`{}`", parameter.name),
            });
            push_text(
                out,
                doc.and_then(|doc| doc.named("template", &parameter.name)),
            );
        }
    }

    /// The `## Parameters` and `## Returns` sections of a signature.
    fn signature_sections(&self, out: &mut Vec<String>, signature: &Signature, doc: Option<&Doc>) {
        self.type_parameters(out, &signature.type_parameters, doc);
        if !signature.parameters.is_empty() {
            out.push("## Parameters".to_owned());
            for parameter in &signature.parameters {
                let dots = if parameter.rest { "..." } else { "" };
                let mark = if parameter.optional { "?" } else { "" };
                out.push(format!("### {dots}{}{mark}", escape(&parameter.name)));
                out.push(self.ty(&parameter.value));
                push_text(out, doc.and_then(|doc| doc.named("param", &parameter.name)));
            }
        }
        out.push("## Returns".to_owned());
        out.push(self.ty(&signature.returns));
        push_text(out, doc.and_then(|doc| doc.first("returns")));
    }

    /// The whole page of an export, ending with a newline.
    #[must_use]
    pub fn render(&self, export: &Export, title: &str) -> String {
        let name = escape(&export.name);
        let doc = export.doc.as_ref();
        let mut out = vec![format!("[{title}](../index.md) / {name}")];

        match &export.body {
            Body::Function(signature) => {
                out.push(format!("# {}: {name}()", export.kind.title()));
                out.push(format!(
                    "> **{name}**{}: {}",
                    head(signature),
                    self.ty(&signature.returns)
                ));
                push_place_and_summary(&mut out, export, doc);
                self.signature_sections(&mut out, signature, doc);
            }
            Body::TypeAlias {
                type_parameters,
                value,
            } => {
                out.push(format!(
                    "# {}: {name}{}",
                    export.kind.title(),
                    type_parameter_list(type_parameters, false)
                ));
                out.push(format!(
                    "> **{name}**{} = {}",
                    type_parameter_list(type_parameters, true),
                    self.ty(value)
                ));
                push_place_and_summary(&mut out, export, doc);
                match value {
                    Type::Function(signature) => {
                        self.type_parameters(&mut out, type_parameters, doc);
                        self.signature_sections(&mut out, signature, None);
                    }
                    Type::Object(members) => {
                        self.type_parameters(&mut out, type_parameters, doc);
                        self.properties(&mut out, members, &export.file);
                    }
                    _ => {
                        self.type_parameters(&mut out, type_parameters, doc);
                        if let Type::Intersection(parts) = value {
                            for part in parts {
                                if let Type::Object(members) = part {
                                    self.declaration(&mut out, members, 3, "");
                                }
                            }
                        }
                    }
                }
            }
            Body::Variable { value, literal } => {
                out.push(format!("# {}: {name}", export.kind.title()));
                let shown = literal
                    .as_ref()
                    .map_or_else(String::new, |literal| format!(" = `{literal}`"));
                out.push(format!("> `const` **{name}**: {}{shown}", self.ty(value)));
                push_place_and_summary(&mut out, export, doc);
                if let Type::Object(members) = value {
                    self.declaration(&mut out, members, 3, "");
                }
            }
        }

        if let Some(doc) = doc {
            for tag in doc
                .tags
                .iter()
                .filter(|tag| !SHOWN_TAGS.contains(&tag.name.as_str()))
            {
                out.push(format!("## {}", capitalized(&tag.name)));
                push_text(&mut out, Some(&tag.text));
            }
        }

        let text = out.join("\n\n");
        format!("{}\n", self.links(&text))
    }

    /// The `{@link name}` tags of a text as links to their pages, their label kept.
    fn links(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;

        while let Some(start) = rest.find("{@link") {
            let after = &rest[start + "{@link".len()..];
            let Some(end) = after.find('}') else { break };
            out.push_str(&rest[..start]);
            let inside = after[..end].trim();
            let (target, label) = inside
                .split_once('|')
                .map_or((inside, inside), |(target, label)| {
                    (target.trim(), label.trim())
                });
            match self.names.get(target) {
                Some(kind) => {
                    out.push('[');
                    out.push_str(label);
                    out.push_str("](");
                    out.push_str(&href(self.kind, *kind, target));
                    out.push(')');
                }
                None => out.push_str(label),
            }
            rest = &after[end + 1..];
        }

        out.push_str(rest);
        out
    }
}

/// The parameters of a signature as code: `` `a`, ...`b`, `c`? ``.
fn parameter_list(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .map(|parameter| {
            let dots = if parameter.rest { "..." } else { "" };
            let mark = if parameter.optional { "?" } else { "" };
            format!("{dots}`{}`{mark}", parameter.name)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A signature's head: `` \<`S`\>(`a`, `b`) ``.
fn head(signature: &Signature) -> String {
    format!(
        "{}({})",
        type_parameter_list(&signature.type_parameters, true),
        parameter_list(&signature.parameters)
    )
}

/// A list of type parameters, as code in a signature or plain in a title.
fn type_parameter_list(parameters: &[TypeParameter], code: bool) -> String {
    if parameters.is_empty() {
        return String::new();
    }
    let names: Vec<String> = parameters
        .iter()
        .map(|parameter| {
            if code {
                format!("`{}`", parameter.name)
            } else {
                escape(&parameter.name)
            }
        })
        .collect();
    format!("\\<{}\\>", names.join(", "))
}

/// `Defined in: file:line` and the summary.
fn push_place_and_summary(out: &mut Vec<String>, export: &Export, doc: Option<&Doc>) {
    out.push(format!("Defined in: {}:{}", export.file, export.line));
    push_summary(out, doc);
}

/// A block's summary, when it has one.
fn push_summary(out: &mut Vec<String>, doc: Option<&Doc>) {
    push_text(out, doc.map(|doc| doc.summary.as_str()));
}

/// A text, when it is not empty.
fn push_text(out: &mut Vec<String>, text: Option<&str>) {
    if let Some(text) = text.filter(|text| !text.is_empty()) {
        out.push(text.to_owned());
    }
}

/// A tag's name as a heading: `kind` as `Kind`.
fn capitalized(name: &str) -> String {
    let mut characters = name.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::module;

    /// The pages of one module's exports, by name.
    fn pages(source: &str) -> BTreeMap<String, String> {
        let read = module(source, "a/b.ts");
        assert!(read.problems.is_empty(), "{:?}", read.problems);
        let names: Names = read
            .exports
            .iter()
            .map(|each| (each.name.clone(), each.kind))
            .collect();
        read.exports
            .iter()
            .map(|each| {
                (
                    each.name.clone(),
                    Page::new(&names, each.kind).render(each, "editor"),
                )
            })
            .collect()
    }

    #[test]
    fn writes_a_function_page_as_typedoc_did() {
        // TypeDoc's page of `add` on 2026-10-17, its line and file those of this source.
        let pages = pages(
            "/** P. */\nexport type Point = number;\n\n/**\n * Point moved by a vector, component-wise.\n *\n * @kind math\n * @param point - start point\n * @param vector - displacement\n * @returns the moved point\n * @see REF-MATHWORLD-VECTOR-ADDITION\n */\nexport function add(point: Point, vector: Point): Point {\n  return point;\n}\n",
        );

        assert_eq!(
            pages["add"],
            "[editor](../index.md) / add\n\n# Function: add()\n\n> **add**(`point`, `vector`): [`Point`](../type-aliases/Point.md)\n\nDefined in: a/b.ts:13\n\nPoint moved by a vector, component-wise.\n\n## Parameters\n\n### point\n\n[`Point`](../type-aliases/Point.md)\n\nstart point\n\n### vector\n\n[`Point`](../type-aliases/Point.md)\n\ndisplacement\n\n## Returns\n\n[`Point`](../type-aliases/Point.md)\n\nthe moved point\n\n## Kind\n\nmath\n\n## See\n\nREF-MATHWORLD-VECTOR-ADDITION\n"
        );
    }

    #[test]
    fn writes_type_parameters_their_constraint_and_a_function_type() {
        let pages = pages(
            "/**\n * Step.\n *\n * @template S - type of the states\n */\nexport type Step<S> = (state: S, ...rest: S[]) => S;\n\n/**\n * Scan.\n *\n * @template B - the buffer\n * @param step - one step\n * @param bytes - read\n * @returns states\n */\nexport function scan<B extends ArrayBufferLike>(step: Step<B>, bytes?: Uint8Array<B>): readonly (B | undefined)[] {\n  return [];\n}\n",
        );

        assert_eq!(
            pages["Step"],
            "[editor](../index.md) / Step\n\n# Type Alias: Step\\<S\\>\n\n> **Step**\\<`S`\\> = (`state`, ...`rest`) => `S`\n\nDefined in: a/b.ts:6\n\nStep.\n\n## Type Parameters\n\n### S\n\n`S`\n\ntype of the states\n\n## Parameters\n\n### state\n\n`S`\n\n### ...rest\n\n`S`[]\n\n## Returns\n\n`S`\n"
        );
        assert_eq!(
            pages["scan"],
            "[editor](../index.md) / scan\n\n# Function: scan()\n\n> **scan**\\<`B`\\>(`step`, `bytes`?): readonly (`B` \\| `undefined`)[]\n\nDefined in: a/b.ts:16\n\nScan.\n\n## Type Parameters\n\n### B\n\n`B` *extends* `ArrayBufferLike`\n\nthe buffer\n\n## Parameters\n\n### step\n\n[`Step`](../type-aliases/Step.md)\\<`B`\\>\n\none step\n\n### bytes?\n\n`Uint8Array`\\<`B`\\>\n\nread\n\n## Returns\n\nreadonly (`B` \\| `undefined`)[]\n\nstates\n"
        );
    }

    #[test]
    fn writes_the_properties_of_an_object_alias_sorted_between_rules() {
        let pages = pages(
            "/** Arc. */\nexport type Arc = {\n  /** Discriminant. */\n  readonly kind: \"arc\";\n  /** Center. */\n  readonly center: Arc;\n  size?: number;\n};\n",
        );

        assert_eq!(
            pages["Arc"],
            "[editor](../index.md) / Arc\n\n# Type Alias: Arc\n\n> **Arc** = `object`\n\nDefined in: a/b.ts:2\n\nArc.\n\n## Properties\n\n### center\n\n> `readonly` **center**: [`Arc`](Arc.md)\n\nDefined in: a/b.ts:6\n\nCenter.\n\n***\n\n### kind\n\n> `readonly` **kind**: `\"arc\"`\n\nDefined in: a/b.ts:4\n\nDiscriminant.\n\n***\n\n### size\n\n> **size**?: `number`\n\nDefined in: a/b.ts:7\n"
        );
    }

    #[test]
    fn writes_the_declaration_of_an_intersection_and_a_nested_constant() {
        let pages = pages(
            "/** Size. */\nexport type Size = number;\n/** Turned. */\nexport type Turned = Size & {\n  /** Angle. */\n  readonly angle: number;\n};\n/** Ranges. */\nexport const MY_RANGES: {\n  /** Box. */\n  readonly box: {\n    /** Last. */\n    readonly last: number;\n    /** First. */\n    readonly first: number;\n  };\n} = { box: { first: 0, last: 1 } };\n",
        );

        assert_eq!(
            pages["Turned"],
            "[editor](../index.md) / Turned\n\n# Type Alias: Turned\n\n> **Turned** = [`Size`](Size.md) & `object`\n\nDefined in: a/b.ts:4\n\nTurned.\n\n## Type Declaration\n\n### angle\n\n> `readonly` **angle**: `number`\n\nAngle.\n"
        );
        assert_eq!(
            pages["MY_RANGES"],
            "[editor](../index.md) / MY\\_RANGES\n\n# Variable: MY\\_RANGES\n\n> `const` **MY\\_RANGES**: `object`\n\nDefined in: a/b.ts:9\n\nRanges.\n\n## Type Declaration\n\n### box\n\n> `readonly` **box**: `object`\n\nBox.\n\n#### box.first\n\n> `readonly` **first**: `number`\n\nFirst.\n\n#### box.last\n\n> `readonly` **last**: `number`\n\nLast.\n"
        );
    }

    #[test]
    fn writes_constants_with_their_literal_value_or_their_type() {
        let pages = pages(
            "/** Turn. */\nexport const DEGREES_PER_TURN = 360;\n/** As const. */\nexport const ALIGNMENTS = [\"min\", \"max\"] as const;\n/** An alias. */\nexport type Alignment = (typeof ALIGNMENTS)[number];\n",
        );

        assert_eq!(
            pages["DEGREES_PER_TURN"],
            "[editor](../index.md) / DEGREES\\_PER\\_TURN\n\n# Variable: DEGREES\\_PER\\_TURN\n\n> `const` **DEGREES\\_PER\\_TURN**: `360` = `360`\n\nDefined in: a/b.ts:2\n\nTurn.\n"
        );
        assert!(
            pages["ALIGNMENTS"]
                .contains("> `const` **ALIGNMENTS**: readonly \\[`\"min\"`, `\"max\"`\\]\n")
        );
        assert!(pages["Alignment"].contains(
            "> **Alignment** = *typeof* [`ALIGNMENTS`](../variables/ALIGNMENTS.md)\\[`number`\\]\n"
        ));
    }

    #[test]
    fn writes_an_internal_alias_out_and_a_predicate_as_code() {
        let pages = pages(
            "type Classes = Readonly<Record<number, number>>;\n/** Free. */\nexport type Free = number;\n/** Is. */\nexport function isFree(shape: number, map: Classes): shape is Free {\n  return true;\n}\n",
        );

        assert!(pages["isFree"].contains("> **isFree**(`shape`, `map`): `shape is Free`\n\n"));
        assert!(
            pages["isFree"]
                .contains("### map\n\n`Readonly`\\<`Record`\\<`number`, `number`\\>\\>\n\n")
        );
    }

    #[test]
    fn writes_the_links_of_a_summary() {
        let pages = pages(
            "/** P. */\nexport type Point = number;\n/** Uses {@link Point} and {@link Point | the point}, not {@link gone}. */\nexport function f(): void {}\n",
        );

        assert!(pages["f"].contains(
            "Uses [Point](../type-aliases/Point.md) and [the point](../type-aliases/Point.md), not gone.\n"
        ));
    }

    #[test]
    fn escapes_markdown_outside_code() {
        assert_eq!(escape("A_B*<c>[d]|e"), "A\\_B\\*\\<c\\>\\[d\\]\\|e");
        assert_eq!(capitalized(""), "");
    }
}
