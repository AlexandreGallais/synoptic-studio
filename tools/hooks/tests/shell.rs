//! Tests of the shell word reader (AUD-023): words unquoted as POSIX's quoting rules read them,
//! redirections apart, substitutions and subshells read as commands of their own.

use hooks::forbidden::{Arguments, abbreviates, invocations, is_hooks_path, program_name};
use hooks::shell::{Simple, commands};

/// The words of each simple command of a line.
fn words(line: &str) -> Vec<Vec<String>> {
    commands(line)
        .into_iter()
        .map(|simple| simple.words)
        .collect()
}

/// Owned words.
fn owned(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

#[test]
fn unquotes_each_word_as_the_shell_does() {
    // Single quotes keep a backslash; double quotes drop it before `"`, `\`, `$`, `` ` `` only;
    // outside quotes it keeps the next character and drops a line feed.
    assert_eq!(
        words(r#"a 'b\' "c\"d\x" e\ f g\"#),
        [owned(&["a", "b\\", "c\"d\\x", "e f", "g\\"])]
    );
    assert_eq!(words("a \\\n-f"), [owned(&["a", "-f"])]);
    // Quotes glued to a word, an empty quoted word, `$'…'` escapes.
    assert_eq!(
        words(r#"--"for"'ce' "" $'\x2df\'\n\101' $'\q'"#),
        [owned(&["--force", "", "-f'\nA", "q"])]
    );
    // An unclosed quote runs to the end.
    assert_eq!(words("a 'b c"), [owned(&["a", "b c"])]);
    // `$x`, `${x}`, `$((1+2))` are text, a lone `$` too.
    assert_eq!(
        words("echo $x ${a:-b c} $((1 + (2))) $"),
        [owned(&["echo", "$x", "${a:-b c}", "$((1 + (2)))", "$"])]
    );
}

#[test]
fn splits_at_separators_subshells_and_comments() {
    assert_eq!(
        words("a; b && c || d | e & f\ng"),
        [
            owned(&["a"]),
            owned(&["b"]),
            owned(&["c"]),
            owned(&["d"]),
            owned(&["e"]),
            owned(&["f"]),
            owned(&["g"])
        ]
    );
    assert_eq!(
        words("(a; b) && c"),
        [owned(&["a"]), owned(&["b"]), owned(&["c"])]
    );
    // A comment runs to the end of its line; a `#` inside a word is text.
    assert_eq!(words("a#b # c d\ne"), [owned(&["a#b"]), owned(&["e"])]);
    assert_eq!(words(""), [] as [Vec<String>; 0]);
    assert_eq!(words(" ;; "), [] as [Vec<String>; 0]);
}

#[test]
fn reads_the_commands_of_substitutions() {
    let simples = commands("x=$(git push -f) \"$(a \"b\")\" `c \\`d\\``");

    assert_eq!(
        simples
            .iter()
            .map(|simple| simple.words.clone())
            .collect::<Vec<_>>(),
        [
            owned(&["git", "push", "-f"]),
            owned(&["a", "b"]),
            owned(&["d"]),
            owned(&["c", ""]),
            owned(&["x=", "", ""])
        ]
    );
    assert!(simples[4].substituted);
    assert!(!simples[0].substituted);
    assert_eq!(
        words("diff <(a) >(b)"),
        [owned(&["a"]), owned(&["b"]), owned(&["diff", "", ""])]
    );
}

#[test]
fn keeps_redirections_apart_from_the_words() {
    let [simple]: [Simple; 1] = commands("a 2>&1 >out <in 3>> log &>all >&- b <<'EOF' c >|f")
        .try_into()
        .expect("one command");

    assert_eq!(simple.words, owned(&["a", "b", "c"]));
    assert_eq!(
        simple.redirections,
        owned(&[
            "2>&1", ">out", "<in", "3>>log", "&>all", ">&-", "<<EOF", ">|f"
        ])
    );
    // A word of digits glued to the operator is its descriptor; spaced, it is a word.
    assert_eq!(commands("a 2 >x")[0].words, owned(&["a", "2"]));
}

#[test]
fn reads_options_as_git_does() {
    let read = Arguments::read(
        &owned(&[
            "-am",
            "msg",
            "-vn",
            "--author",
            "me",
            "--no-veri",
            "--file=x",
            "--",
            "-f",
        ]),
        "m",
        &["--author"],
    );

    assert_eq!(read.shorts, ['a', 'm', 'v', 'n']);
    assert_eq!(read.longs, owned(&["--author", "--no-veri", "--file"]));
    assert_eq!(read.operands, owned(&["-f"]));
    // A value glued to its short option takes no next word.
    assert_eq!(
        Arguments::read(&owned(&["-mx", "y"]), "m", &[]).operands,
        owned(&["y"])
    );
    assert!(abbreviates("--no-v", "--no-verify"));
    assert!(abbreviates("--force", "--force"));
    assert!(!abbreviates("--", "--force"));
    assert!(!abbreviates("--force-with-lease", "--force"));
    assert!(is_hooks_path(" CORE.HOOKSPATH"));
    assert!(!is_hooks_path("core.hooksPathX"));
    assert_eq!(program_name("/usr/bin/git"), "git");
}

#[test]
fn looks_through_each_wrapper() {
    let programs = |line: &str| -> Vec<String> {
        invocations(line)
            .into_iter()
            .filter_map(|invocation| invocation.words.first().cloned())
            .collect()
    };

    assert_eq!(programs("env -i -C /x A=1 -- git x"), ["env", "git"]);
    assert_eq!(
        programs("sudo -u me -- nice -10 git x"),
        ["sudo", "nice", "git"]
    );
    assert_eq!(programs("timeout -s KILL 5 git x"), ["timeout", "git"]);
    assert_eq!(programs("sh -c 'a; b'"), ["sh", "a", "b"]);
    // A script file, or a shell without `-c`, runs nothing the guard reads.
    assert_eq!(programs("bash script.sh"), ["bash"]);
    assert_eq!(programs("cargo run -p page -- REF-X"), ["cargo", "REF-X"]);
    assert_eq!(programs("cargo build"), ["cargo"]);
    assert_eq!(
        programs("find . -name x -exec a {} + -ok b \\;"),
        ["find", "a", "b"]
    );
    // An assignment alone, and an `export`, keep their assignments.
    let alone = invocations("A=1");

    assert_eq!(alone[0].assignments, owned(&["A=1"]));
    assert_eq!(invocations("export B=2 C")[1].assignments, owned(&["B=2"]));
}
