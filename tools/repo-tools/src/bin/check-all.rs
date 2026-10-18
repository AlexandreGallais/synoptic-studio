//! `check-all` (`npm run check:all`, RET-002, EN-103): every check of the repository in parallel
//! lanes, run from the repository's root under the heavy lock `npm run check:all` takes. Each check
//! prints a line with its time, a failed one its output after it; the last line says whether all
//! passed. The release binaries are built first, alone; the docs lane runs only when the branch
//! changed one of its inputs against its base (EN-085): the nearest of `develop` and the feature
//! branches on GitHub for a story, `develop` for a feature; on `main`, `develop`, a detached head
//! or in the CI every lane runs. `.git/config` of the main checkout is read before and after the
//! lanes: a lane that changed a line a test could break fails the run, its lines named, nothing
//! restored (F21.AC9, EN-113).

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{ExitCode, Stdio};
use std::thread;
use std::time::Instant;

use repo_tools::check_all::{
    BEFORE_LANES, change_against_base, command_of, config_report, git_lines, lanes_to_run,
    report_of, skipped_lines, summary,
};
use repo_tools::scripts::{read_scripts, script_command, shell};

/// The result of one check.
struct Checked {
    name: String,
    is_ok: bool,
}

/// Runs one npm script's command in the shell npm runs it in, stdout and stderr kept together in
/// their order, then prints its report line; `check:all` is started by npm, so the command finds
/// the tools of `node_modules/.bin`.
fn run_check(name: &str, scripts: &[(String, String)], root: &Path) -> Checked {
    let start = Instant::now();
    let command = script_command(scripts, name);
    let result = io::pipe().and_then(|(mut reader, writer)| {
        let mut shell = shell(command_of(&command), root);
        let mut child = shell
            .stdin(Stdio::null())
            .stdout(writer.try_clone()?)
            .stderr(writer)
            .spawn()?;
        let mut output = Vec::new();

        // The command holds the pipe's writing ends until dropped: reading would never end.
        drop(shell);
        reader.read_to_end(&mut output)?;
        Ok((child.wait()?, output))
    });
    let (is_ok, output) = match result {
        Ok((status, output)) => (
            status.success(),
            String::from_utf8_lossy(&output).into_owned(),
        ),
        Err(error) => (false, format!("{name}: {error}")),
    };
    let line = report_of(name, is_ok, start.elapsed().as_millis(), &output);

    // One write per report, so that two lanes' lines never interleave.
    let _ = writeln!(io::stdout().lock(), "{line}");
    Checked {
        name: name.to_owned(),
        is_ok,
    }
}

/// Runs the checks of a lane one after the other, every one even after a failure.
fn run_lane(lane: &[&str], scripts: &[(String, String)], root: &Path) -> Vec<Checked> {
    lane.iter()
        .map(|name| run_check(name, scripts, root))
        .collect()
}

/// The main checkout's `.git/config`: the Git common directory's.
fn config_path(root: &Path) -> Option<PathBuf> {
    git_lines(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .ok()?
    .first()
    .map(|common| Path::new(common).join("config"))
}

fn work() -> u8 {
    let start = Instant::now();
    let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let scripts = read_scripts(&fs::read_to_string(root.join("package.json")).unwrap_or_default());
    // In the CI, or when Git fails, the change is unknown: every lane runs.
    let change = if env::var_os("CI").is_some() {
        None
    } else {
        change_against_base(&root).ok().flatten()
    };
    let to_run = lanes_to_run(change.as_ref().map(|(_, paths)| paths.as_slice()));

    if let Some((base, paths)) = &change {
        for line in skipped_lines(paths, base) {
            println!("{line}");
        }
    }

    let config = config_path(&root);
    let before = config.as_ref().and_then(|path| fs::read(path).ok());
    let built = run_check(BEFORE_LANES, &scripts, &root);
    // Without the release binaries every lane's `cargo run` would build them at once: none starts.
    let lanes: Vec<Checked> = if built.is_ok {
        thread::scope(|scope| {
            let handles: Vec<_> = to_run
                .iter()
                .map(|lane| scope.spawn(|| run_lane(lane, &scripts, &root)))
                .collect();

            handles
                .into_iter()
                .flat_map(|handle| handle.join().unwrap_or_default())
                .collect()
        })
    } else {
        Vec::new()
    };
    let mut failed: Vec<String> = std::iter::once(&built)
        .chain(&lanes)
        .filter(|checked| !checked.is_ok)
        .map(|checked| checked.name.clone())
        .collect();

    if let Some(path) = &config {
        let after = fs::read(path).ok();

        if let Some(report) = config_report(
            &path.display().to_string(),
            before.as_deref(),
            after.as_deref(),
        ) {
            println!("{report}");
            failed.push(".git/config".to_owned());
        }
    }

    println!("{}", summary(start.elapsed().as_millis(), &failed));
    u8::from(!failed.is_empty())
}

fn main() -> ExitCode {
    tool_use::run("check-all", "", work)
}
