//! The npm scripts of `package.json`, read and run without npm: a tool that runs another script
//! starts its command in the shell npm would start it in, without npm's own start (about 107 ms,
//! research note 0025).

use std::io;
use std::path::Path;
use std::process::{Command, ExitStatus};

use hooks::json::{Value, parse};

/// The scripts of a `package.json`, by name in their order; none when it has none or is not JSON.
#[must_use]
pub fn read_scripts(manifest: &str) -> Vec<(String, String)> {
    match parse(manifest).as_ref().map(|value| value.get("scripts")) {
        Some(Value::Object(members)) => members
            .iter()
            .map(|(name, command)| (name.clone(), command.text().to_owned()))
            .collect(),
        _ => Vec::new(),
    }
}

/// The command of a script, `npm run <name>` when the manifest has none of that name.
#[must_use]
pub fn script_command(scripts: &[(String, String)], name: &str) -> String {
    scripts
        .iter()
        .find(|(script, _)| script == name)
        .map_or_else(|| format!("npm run {name}"), |(_, command)| command.clone())
}

/// A shell command, as npm starts a script's: `sh -c <command>` in a folder.
#[must_use]
pub fn shell(command: &str, folder: &Path) -> Command {
    let mut shell = Command::new("sh");

    shell.arg("-c").arg(command).current_dir(folder);
    shell
}

/// Runs a script of the repository's `package.json` in its root, its output shown as it runs.
///
/// # Errors
/// When the manifest cannot be read or the shell cannot start.
pub fn run_script(root: &Path, name: &str) -> io::Result<ExitStatus> {
    let scripts = read_scripts(&std::fs::read_to_string(root.join("package.json"))?);

    shell(&script_command(&scripts, name), root).status()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_scripts_in_their_order() {
        let manifest = r#"{"name":"x","scripts":{"b":"echo b","a":"npm run b"}}"#;

        assert_eq!(
            read_scripts(manifest),
            [
                ("b".to_owned(), "echo b".to_owned()),
                ("a".to_owned(), "npm run b".to_owned())
            ]
        );
        assert_eq!(
            read_scripts(r#"{"name":"x"}"#),
            Vec::<(String, String)>::new()
        );
        assert_eq!(read_scripts("not JSON"), Vec::<(String, String)>::new());
    }

    #[test]
    fn falls_back_on_npm_for_a_script_it_does_not_know() {
        let scripts = read_scripts(r#"{"scripts":{"lint":"oxlint ."}}"#);

        assert_eq!(script_command(&scripts, "lint"), "oxlint .");
        assert_eq!(script_command(&scripts, "test"), "npm run test");
    }

    #[test]
    fn runs_a_script_in_the_root() {
        let root = std::env::temp_dir().join(format!("repo-tools-scripts-{}", std::process::id()));

        std::fs::create_dir_all(&root).expect("root");
        std::fs::write(
            root.join("package.json"),
            r#"{"scripts":{"ok":"test -f package.json","ko":"exit 3"}}"#,
        )
        .expect("manifest");

        let ok = run_script(&root, "ok").expect("runs");
        let ko = run_script(&root, "ko").expect("runs");

        std::fs::remove_dir_all(&root).expect("cleaned");
        assert!(ok.success());
        assert_eq!(ko.code(), Some(3));
    }
}
