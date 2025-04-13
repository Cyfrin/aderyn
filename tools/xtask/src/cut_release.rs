use std::{io::BufRead, time::Duration};

use xshell::{Shell, cmd};

use crate::flags::CutRelease;

pub fn cut_release(cut_release: CutRelease) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    // Wait for existing release completion
    wait_for_release_completion(&sh)?;

    // Sanity checks and syncs
    sync_tags(&sh)?;
    perform_prechecks(&sh)?;

    // Release process
    dry_run(&sh, &cut_release)?;
    kick_off_release(&sh, &cut_release)?;

    // Wait for release completion
    println!("Waiting for release completion...");
    std::thread::sleep(Duration::from_secs(10 * 60));
    wait_for_release_completion(&sh)?;

    // Regenerate sarif report (it would be broken because version number is contained)
    regenerate_sarif_report(&sh)?;
    Ok(())
}

fn wait_for_release_completion(sh: &Shell) -> anyhow::Result<()> {
    let poll_time = Duration::from_secs(12);

    // Check if actions are still pending
    let actions_completed = {
        let cmd = cmd!(sh, "gh run list --workflow release.yml");
        let x = cmd.output()?.stdout.to_vec();
        let stdout = String::from_utf8_lossy(&x);
        stdout.lines().filter(|l| !l.is_empty()).all(|l| l.starts_with("completed"))
    };

    if !actions_completed {
        println!(
            "A release or a release plan is in progress ... [next poll: {}s]",
            poll_time.as_secs()
        );
        std::thread::sleep(Duration::from_secs(12));
        wait_for_release_completion(sh)?;
        return Ok(());
    }

    Ok(())
}

fn kick_off_release(sh: &Shell, cut_release: &CutRelease) -> anyhow::Result<()> {
    let execute_cmd = if cut_release.patch {
        cmd!(sh, "cargo release patch --no-publish --exclude xtask --execute")
    } else if cut_release.minor {
        cmd!(sh, "cargo release minor --no-publish --exclude xtask --execute")
    } else {
        unreachable!()
    };

    println!("Kick off the release process\n\taderyn\n?[y/N]");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    if line.contains("y") {
        println!("Kicked-off release process!");
        let d = execute_cmd.stdin(line.clone());
        d.run()?;
    } else {
        println!("Declined release process!");
    }

    Ok(())
}

fn dry_run(sh: &Shell, cut_release: &CutRelease) -> anyhow::Result<()> {
    let dry_run_command = if cut_release.patch {
        cmd!(sh, "cargo release patch --no-publish --exclude xtask --no-tag")
    } else if cut_release.minor {
        cmd!(sh, "cargo release minor --no-publish --exclude xtask --no-tag")
    } else {
        unreachable!();
    };
    dry_run_command.run()?;
    Ok(())
}

fn sync_tags(sh: &Shell) -> anyhow::Result<()> {
    let sync = cmd!(sh, "git fetch --prune-tags origin");
    sync.run()?;
    Ok(())
}

fn regenerate_sarif_report(sh: &Shell) -> anyhow::Result<()> {
    let regen = cmd!(sh, "cargo blesspr");
    regen.run()?;
    Ok(())
}

fn perform_prechecks(sh: &Shell) -> anyhow::Result<()> {
    // Exit if not on dev branch
    let curr_branch = {
        let cmd = cmd!(sh, "git rev-parse --abbrev-ref HEAD");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if curr_branch.trim() != "dev" {
        eprintln!("Please switch to dev branch and retry!. Curr branch: {}", curr_branch.trim());
        std::process::exit(1);
    }

    // Error out if there are untracked files/staging changes
    let uncommitted_stuff = {
        let cmd = cmd!(sh, "git status --porcelain");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if !uncommitted_stuff.is_empty() {
        eprintln!("Please clear your staging area and retry!");
        std::process::exit(1);
    }

    // Error if dev branch is not in sync with remote
    let local_commit_hash = {
        let cmd = cmd!(sh, "git rev-parse dev");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    let remote_commit_hash = {
        let cmd = cmd!(sh, "git rev-parse origin/dev");
        let output = cmd.output()?;
        String::from_utf8(output.stdout)?
    };
    if remote_commit_hash != local_commit_hash {
        eprintln!("dev branch is not in sync with origin. Do that and retry!");
        std::process::exit(1);
    }
    Ok(())
}
