//! `command` run as under the pre-push hook: the test binary starts itself again with `GIT_DIR`
//! and another `GIT_*` variable set, and the inner test checks neither reaches the command.

use std::env;
use std::fs;
use std::process::Command;

use test_support::command;

/// The inner test, run only by the outer one, which plants the variables.
#[test]
#[ignore = "run by f21_ac9_a_git_dir_set_by_the_caller_does_not_reach_the_command"]
fn inner_the_planted_variables_stay_out() {
    // Planted by the outer test: without them this test proves nothing.
    assert!(env::var_os("GIT_DIR").is_some(), "GIT_DIR planted");
    assert!(
        env::var_os("GIT_INDEX_FILE").is_some(),
        "GIT_INDEX_FILE planted"
    );

    let output = command("sh")
        .args([
            "-c",
            "printf '%s %s %s' \"${GIT_DIR-unset}\" \"${GIT_INDEX_FILE-unset}\" \"${OTHER-unset}\"",
        ])
        .output()
        .expect("sh runs");

    // Every GIT_* variable removed, any other inherited.
    assert_eq!(String::from_utf8_lossy(&output.stdout), "unset unset kept");

    // Git itself, in a folder of its own, works on that folder's repository.
    let folder = env::temp_dir().join(format!("test-support-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).expect("folder");
    let init = command("git")
        .args(["init", "-q"])
        .current_dir(&folder)
        .status()
        .expect("git runs");
    let set = command("git")
        .args(["config", "core.bare", "true"])
        .current_dir(&folder)
        .status()
        .expect("git runs");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(init.success() && set.success());
}

#[test]
fn f21_ac9_a_git_dir_set_by_the_caller_does_not_reach_the_command() {
    let decoy = env::temp_dir().join(format!("test-support-decoy-{}", std::process::id()));
    let _ = fs::remove_dir_all(&decoy);
    fs::create_dir_all(&decoy).expect("decoy");
    let init = command("git")
        .args(["init", "-q"])
        .current_dir(&decoy)
        .status()
        .expect("git runs");
    assert!(init.success());
    let config = decoy.join(".git/config");
    let before = fs::read(&config).expect("config");

    let run = Command::new(env::current_exe().expect("test binary"))
        .args([
            "--exact",
            "inner_the_planted_variables_stay_out",
            "--ignored",
        ])
        .env("GIT_DIR", decoy.join(".git"))
        .env("GIT_INDEX_FILE", decoy.join(".git/index"))
        .env("OTHER", "kept")
        .output()
        .expect("inner test runs");
    let after = fs::read(&config).expect("config");

    fs::remove_dir_all(&decoy).expect("cleaned");
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stdout)
    );
    assert!(String::from_utf8_lossy(&run.stdout).contains("1 passed"));
    // The decoy's config, which a leaked GIT_DIR would have made bare, is unchanged.
    assert_eq!(after, before);
}
