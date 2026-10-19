//! Exact text replacements in several files at once, read from standard input (RET-002, EN-103):
//! each block names a file, the text to find — present exactly once — and its replacement. Nothing
//! is written unless every block is read and every old text matches.
//!
//! ```text
//! @@ path/to/file.ts
//! <<<
//! old text
//! ===
//! new text
//! >>>
//! ```
//!
//! A block is read as `scripts/replace-text.ts` read it: the old text ends at the first `===` line
//! end, the new text at the first `>>>` closing a line, a line end before either left out. Unlike
//! that script, which skipped what it could not read and edited the rest, any line outside a block
//! but blank lines refuses the whole input, and so does a text holding a line `<<<` or `>>>`: a
//! block that lost its `@@` or its `>>>` swallowed or left out its neighbour (F20's run).

/// What `replace` prints when it is called without its blocks: on standard output for `--help`, on
/// standard error otherwise (EN-138).
pub const USAGE: &str = "usage: target/release/replace < <blocks>, or target/release/replace --help
Exact text replacements in several files at once, read on standard input, each block:
  @@ path/to/file
  <<<
  old text
  ===
  new text
  >>>
Nothing is written unless every block is read and every old text is found exactly once.";

/// What a call of `replace` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Call {
    /// Print [`USAGE`] and exit with this code.
    Usage(u8),
    /// Read the blocks on standard input.
    Blocks,
}

/// What a call does, from its arguments and whether its standard input is a terminal: the usage
/// and 0 for `--help` or `-h`; the usage and 2 for any other argument, or none with a terminal for
/// input, which would wait for blocks nobody types (EN-138); otherwise the blocks read.
#[must_use]
pub fn call(arguments: &[String], input_is_terminal: bool) -> Call {
    match arguments {
        [only] if only == "--help" || only == "-h" => Call::Usage(0),
        [] if !input_is_terminal => Call::Blocks,
        _ => Call::Usage(2),
    }
}

/// One replacement in one file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edit {
    pub file: String,
    pub previous: String,
    pub next: String,
}

/// The text up to the first `marker` closing a line (followed by a line end, or the input's end
/// when `at_end` allows it), a line end before it left out; and where the marker's line ends.
fn until<'a>(text: &'a str, marker: &str, at_end: bool) -> Option<(&'a str, usize)> {
    let mut from = 0;

    loop {
        let at = from + text[from..].find(marker)?;
        let after = at + marker.len();
        let closes = text[after..].starts_with('\n') || (at_end && after == text.len());

        if closes {
            let body = &text[..at];

            return Some((body.strip_suffix('\n').unwrap_or(body), after));
        }
        from = at + 1;
    }
}

/// Whether a text holds a line that only a block's frame holds.
fn holds_frame(text: &str) -> bool {
    text.split('\n').any(|line| line == "<<<" || line == ">>>")
}

/// The block starting the text: its edit and the length it takes, up to its `>>>`.
fn block(text: &str) -> Option<(Edit, usize)> {
    let rest = text.strip_prefix("@@ ")?;
    let (file, rest) = rest.split_once('\n')?;
    let body = rest.strip_prefix("<<<\n")?;
    let (previous, after_old) = until(body, "===", false)?;
    let new_text = &body[after_old + 1..];
    let (next, after_new) = until(new_text, ">>>", true)?;

    if file.is_empty() || holds_frame(previous) || holds_frame(next) {
        return None;
    }

    let length = text.len() - new_text.len() + after_new;

    Some((
        Edit {
            file: file.trim().to_owned(),
            previous: previous.to_owned(),
            next: next.to_owned(),
        },
        length,
    ))
}

/// The edits of the input, in order; or why it was refused: the block it could not read, by its
/// rank and its first line.
///
/// # Errors
/// A line outside a block but blank lines, or a block not written `@@ <file>`, `<<<`, old text,
/// `===`, new text, `>>>`.
pub fn parse_edits(spec: &str) -> Result<Vec<Edit>, String> {
    let mut edits = Vec::new();
    let mut at = 0;

    while at < spec.len() {
        let rest = &spec[at..];
        let line_end = rest.find('\n').map_or(rest.len(), |end| end + 1);
        let line = &rest[..line_end];

        if line.trim().is_empty() {
            at += line_end;
            continue;
        }

        let line_number = spec[..at].matches('\n').count() + 1;
        let Some((edit, length)) = block(rest) else {
            return Err(format!(
                "block {}, line {line_number}: cannot read `{}`: each block is `@@ <file>`, `<<<`, old text, `===`, new text, `>>>`, and no text holds a line `<<<` or `>>>`",
                edits.len() + 1,
                line.trim_end()
            ));
        };

        edits.push(edit);
        at += length;
    }

    Ok(edits)
}

/// A text after its edits, each old text present exactly once at the moment it is replaced; or the
/// first problem met, `edit <n>: found <k> times` or `edit <n>: empty old text`, the text left as
/// it was.
///
/// # Errors
/// The first edit whose old text is missing, repeated or empty.
pub fn apply_edits(text: &str, edits: &[&Edit]) -> Result<String, String> {
    let mut current = text.to_owned();

    for (index, edit) in edits.iter().enumerate() {
        if edit.previous.is_empty() {
            return Err(format!("edit {}: empty old text", index + 1));
        }

        let found = current.matches(edit.previous.as_str()).count();

        if found != 1 {
            return Err(format!("edit {}: found {found} times", index + 1));
        }
        current = current.replacen(edit.previous.as_str(), &edit.next, 1);
    }

    Ok(current)
}

/// The files of the edits, each once, in the order first named.
#[must_use]
pub fn files_of(edits: &[Edit]) -> Vec<&str> {
    let mut files: Vec<&str> = Vec::new();

    for edit in edits {
        if !files.contains(&edit.file.as_str()) {
            files.push(&edit.file);
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit(file: &str, previous: &str, next: &str) -> Edit {
        Edit {
            file: file.to_owned(),
            previous: previous.to_owned(),
            next: next.to_owned(),
        }
    }

    fn arguments(words: &[&str]) -> Vec<String> {
        words.iter().map(|word| (*word).to_owned()).collect()
    }

    #[test]
    fn f31_ac2_prints_its_usage_rather_than_wait_on_a_terminal() {
        // No argument, a terminal for input: the card's call, which waited 120 s for blocks.
        assert_eq!(call(&[], true), Call::Usage(2));
        // No argument, blocks on a pipe: read as before.
        assert_eq!(call(&[], false), Call::Blocks);
        // `--help` and `-h`, a terminal or a pipe: the usage, exit 0.
        for input_is_terminal in [true, false] {
            assert_eq!(
                call(&arguments(&["--help"]), input_is_terminal),
                Call::Usage(0)
            );
            assert_eq!(call(&arguments(&["-h"]), input_is_terminal), Call::Usage(0));
        }
        // Any other argument: `replace` takes none, the usage and exit 2, even on a pipe.
        assert_eq!(call(&arguments(&["edits.txt"]), false), Call::Usage(2));
        assert_eq!(call(&arguments(&["--help", "x"]), false), Call::Usage(2));
    }

    #[test]
    fn reads_blocks_of_file_old_text_and_new_text_multi_line_and_empty_included() {
        let spec = "@@ a.ts\n<<<\none\ntwo\n===\nthree\n>>>\n@@ b.md\n<<<\nfour\n===\n>>>\n";

        assert_eq!(
            parse_edits(spec),
            Ok(vec![
                edit("a.ts", "one\ntwo", "three"),
                edit("b.md", "four", "")
            ])
        );
        // Blank lines between blocks, a block closing the input, an empty old text read as such.
        assert_eq!(
            parse_edits("\n@@ c \n<<<\n===\nx\n>>>"),
            Ok(vec![edit("c", "", "x")])
        );
        assert_eq!(parse_edits(""), Ok(Vec::new()));
    }

    #[test]
    fn reads_a_marker_inside_a_line_as_the_script_did() {
        // `===` ends the old text at its first line end, even after other text on its line.
        assert_eq!(
            parse_edits("@@ f\n<<<\na===\nb\n>>>\n"),
            Ok(vec![edit("f", "a", "b")])
        );
        // `>>>` not closing its line is new text.
        assert_eq!(
            parse_edits("@@ f\n<<<\na\n===\nb >>> c\n>>>\n"),
            Ok(vec![edit("f", "a", "b >>> c")])
        );
    }

    #[test]
    fn f21_ac2_refuses_a_second_block_without_its_file_line() {
        let spec = "@@ a.md\n<<<\nx\n===\ny\n>>>\n<<<\nz\n===\nw\n>>>\n";

        assert_eq!(
            parse_edits(spec),
            Err("block 2, line 7: cannot read `<<<`: each block is `@@ <file>`, `<<<`, old text, `===`, new text, `>>>`, and no text holds a line `<<<` or `>>>`".to_owned())
        );
    }

    #[test]
    fn f21_ac2_refuses_a_block_that_lost_its_end_or_its_frame() {
        // Its `>>>` lost: the next block would have become its new text.
        let spec = "@@ a.md\n<<<\nx\n===\ny\n@@ b.md\n<<<\nz\n===\nw\n>>>\n";

        assert!(
            parse_edits(spec)
                .is_err_and(|error| error.starts_with("block 1, line 1: cannot read `@@ a.md`"))
        );
        // No `===`, no `<<<`, no file, a stray line.
        for spec in [
            "@@ a\n<<<\nx\n>>>\n",
            "@@ a\nx\n===\ny\n>>>\n",
            "@@ \n<<<\nx\n===\ny\n>>>\n",
            "@@ a",
            "a note\n@@ a\n<<<\nx\n===\ny\n>>>\n",
            "@@ a\n<<<\nx\n===\ny\n>>> \n",
        ] {
            assert!(parse_edits(spec).is_err(), "{spec:?}");
        }
    }

    #[test]
    fn replaces_each_old_text_found_once_in_order_dollar_kept_as_written() {
        let first = edit("f", "a", "b$&");
        let second = edit("f", "b$&", "c");

        assert_eq!(apply_edits("xay", &[&first, &second]), Ok("xcy".to_owned()));
    }

    #[test]
    fn refuses_an_old_text_missing_repeated_or_empty() {
        assert_eq!(
            apply_edits("aa", &[&edit("f", "a", "b")]),
            Err("edit 1: found 2 times".to_owned())
        );
        assert_eq!(
            apply_edits("x", &[&edit("f", "a", "b")]),
            Err("edit 1: found 0 times".to_owned())
        );
        assert_eq!(
            apply_edits("x", &[&edit("f", "x", "y"), &edit("f", "", "b")]),
            Err("edit 2: empty old text".to_owned())
        );
        // Occurrences are counted without overlap, as `split` counted them.
        assert_eq!(
            apply_edits("aaa", &[&edit("f", "aa", "b")]),
            Ok("ba".to_owned())
        );
    }

    #[test]
    fn lists_each_file_once_in_the_order_first_named() {
        let edits = [
            edit("b", "1", "2"),
            edit("a", "1", "2"),
            edit("b", "3", "4"),
        ];

        assert_eq!(files_of(&edits), ["b", "a"]);
    }
}
