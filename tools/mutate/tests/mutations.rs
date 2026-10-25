//! Tests of the mutations found in a source text (EN-058).

use mutate::{Kind, Language, Mutation, code_mask, mutations, selected};

/// Each mutation of a script as `kind: original → replacement`.
fn found(source: &str) -> Vec<String> {
    found_in(source, Language::Script)
}

/// Each mutation of a source in a language as `kind: original → replacement`.
fn found_in(source: &str, language: Language) -> Vec<String> {
    mutations(source, language)
        .iter()
        .map(|m| {
            format!(
                "{:?}: {} → {}",
                m.kind,
                &source[m.start..m.end],
                m.replacement
            )
        })
        .collect()
}

#[test]
fn flips_comparisons_and_swaps_operators_written_with_spaces() {
    assert_eq!(
        found("a < b; c >= d; e === f; g + h; i / j;"),
        [
            "Comparison: < → >=",
            "Comparison: >= → <",
            "Comparison: === → !==",
            "Arithmetic: + → -",
            "Arithmetic: / → *",
        ]
    );
}

#[test]
fn takes_no_generic_arrow_increment_nor_unspaced_sign_for_an_operator() {
    // `Array<T>`, `=>`, `i++`, `-x` and `a<b` have no space on both sides of the sign.
    assert_eq!(
        found("const f = (x: Array<T>) => -x; i++; a<b;"),
        Vec::<String>::new()
    );
}

#[test]
fn skips_comments_strings_and_template_literals() {
    let source = "// a < b 1\n/* c + d 2 */ \"e - f 3\" 'g > h 4' `i * j ${5}`;";

    assert_eq!(found(source), Vec::<String>::new());
    // Byte 3 is in the comment, the `;` closing the line is code.
    let mask = code_mask(source, Language::Script);

    assert_eq!((mask[3], mask[source.len() - 1]), (false, true));
}

#[test]
fn keeps_an_escaped_quote_inside_its_string() {
    // The string ends after `\"`: `1` stays inside it, `2` is code.
    assert_eq!(
        found(r#""a \" 1" + 2;"#),
        ["Arithmetic: + → -", "Constant: 2 → 3"]
    );
}

#[test]
fn changes_whole_integers_only() {
    // `1.5`, `x2`, `0x1f` and `3n`-like words are left; `40` becomes `41`.
    assert_eq!(found("f(1.5, x2, 0x1f, 40);"), ["Constant: 40 → 41"]);
}

#[test]
fn negates_a_condition_with_its_nested_brackets() {
    assert_eq!(found("if (ok(a)) {}"), ["Condition: ok(a) → !(ok(a))"]);
}

#[test]
fn replaces_a_returned_value_up_to_its_semicolon() {
    assert_eq!(
        found("function f() { return { a: b }; }"),
        ["Return: { a: b } → undefined"]
    );
    // `return;` and a word ending in `return` are no return of a value.
    assert_eq!(
        found("function f() { return; myreturn x; }"),
        Vec::<String>::new()
    );
}

#[test]
fn applies_a_mutation_and_gives_its_line_and_column() {
    let source = "const a = 1;\nconst b = a + 2;\n";
    let plus = Mutation {
        kind: Kind::Arithmetic,
        start: 25,
        end: 26,
        replacement: "-".to_owned(),
    };

    // Line 2 starts at byte 13; byte 25 is its 13th character.
    assert_eq!(&source[25..26], "+");
    assert_eq!(plus.position(source), (2, 13));
    assert_eq!(plus.apply(source), "const a = 1;\nconst b = a - 2;\n");
}

#[test]
fn selects_the_first_of_each_kind_unless_all_are_asked() {
    let all = mutations("if (a < b) { return 1 + 2; }", Language::Script);
    let kinds: Vec<Kind> = selected(all.clone(), false)
        .iter()
        .map(|m| m.kind)
        .collect();

    // Found: condition, comparison, return, constant 1, plus, constant 2.
    assert_eq!(all.len(), 6);
    assert_eq!(
        kinds,
        [
            Kind::Comparison,
            Kind::Arithmetic,
            Kind::Constant,
            Kind::Condition,
            Kind::Return
        ]
    );
    assert_eq!(selected(all.clone(), true), all);
}

#[test]
fn rev009_reads_a_rust_file_by_its_extension() {
    assert_eq!(Language::of("tools/worktree/src/lib.rs"), Language::Rust);
    assert_eq!(Language::of("src/math/clamp.ts"), Language::Script);
    assert_eq!(Language::of("lib.mjs"), Language::Script);
}

#[test]
fn rev009_takes_a_rust_lifetime_for_no_string_and_skips_its_comments() {
    // Read as a script, `'static` opens a string closed by the comment's `it's`, after which
    // `025` of `(AUD-025)` reads as code (incident card of 2026-10-19). In Rust, `'static` is a
    // lifetime, `'x'` and `'\''` are characters, and the comment stays a comment.
    let source = "fn f(x: &'static str) -> usize {\n    // it's (AUD-025)\n    let c = 'x';\n    let q = '\\'';\n    x.len() + 1\n}\n";

    assert_eq!(
        found_in(source, Language::Script),
        ["Constant: 025 → 26", "Arithmetic: + → -", "Constant: 1 → 2"]
    );
    assert_eq!(
        found_in(source, Language::Rust),
        ["Arithmetic: + → -", "Constant: 1 → 2"]
    );
}

#[test]
fn rev009_replaces_no_rust_return_by_a_script_s_undefined() {
    // A Rust return has no value that fits every type: `undefined` does not build, a build that
    // fails was counted as caught (incident card of 2026-10-19).
    assert_eq!(
        found_in("fn f() -> u8 { return 1; }", Language::Rust),
        ["Constant: 1 → 2"]
    );
}
