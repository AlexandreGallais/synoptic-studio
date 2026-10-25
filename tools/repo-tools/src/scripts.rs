//! A shell command started as npm starts a script's, without npm's own start (about 107 ms,
//! research note 0025): `check-all` runs each check's command so. The functions that read and ran
//! the scripts of `package.json` went with the checks' scripts (ADR-0103, AUD-054).

use std::path::Path;
use std::process::Command;

/// A shell command, as npm starts a script's: `sh -c <command>` in a folder.
#[must_use]
pub fn shell(command: &str, folder: &Path) -> Command {
    let mut shell = Command::new("sh");

    shell.arg("-c").arg(command).current_dir(folder);
    shell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_a_command_in_the_folder_given() {
        let folder = std::env::temp_dir().join(format!("repo-tools-shell-{}", std::process::id()));

        std::fs::create_dir_all(&folder).expect("folder");
        std::fs::write(folder.join("marker"), "").expect("marker");

        let found = shell("test -f marker", &folder).status().expect("runs");
        let failed = shell("exit 3", &folder).status().expect("runs");

        std::fs::remove_dir_all(&folder).expect("cleaned");
        assert!(found.success());
        assert_eq!(failed.code(), Some(3));
    }
}
