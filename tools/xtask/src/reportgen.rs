use xshell::{Shell, cmd};

use crate::flags::Reportgen;

fn run_command(args: &str, release: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    let mut cmd = cmd!(sh, "cargo run");
    if release {
        cmd = cmd.arg("--release");
    }
    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd.args(args.split(" ")).run()?;
    Ok(())
}

fn run_command_with_env(args: &str, key: &str, val: &str, release: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(env!("CARGO_MANIFEST_DIR"));
    sh.change_dir("../../");

    let mut cmd = cmd!(sh, "cargo run");
    cmd = cmd.env(key, val);
    if release {
        cmd = cmd.arg("--release");
    }
    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd.args(args.split(" ")).run()?;
    Ok(())
}

pub fn reportgen(choice: Reportgen) -> anyhow::Result<()> {
    if choice.all && choice.parallel {
        let sh = Shell::new()?;
        sh.change_dir(env!("CARGO_MANIFEST_DIR"));
        sh.change_dir("../../");

        let cmd = cmd!(sh, "chmod +x ./cli/reportgen.sh");
        cmd.run()?;
        let cmd = cmd!(sh, "./cli/reportgen.sh");
        cmd.run()?;

        return Ok(())
    }
    if choice.cpg || choice.all {
        run_command(
            "-i src/ -x lib/ ./tests/contract-playground -o ./reports/report.md",
            choice.release,
        )?;
    }
    if choice.adhoc || choice.all {
        run_command(
            "./tests/adhoc-sol-files -o ./reports/adhoc-sol-files-report.md",
            choice.release,
        )?;
    }
    if choice.sablier || choice.all {
        run_command(
            "./tests/2024-05-Sablier -o ./reports/sablier-aderyn-toml-nested-root.md",
            choice.release,
        )?;
    }
    if choice.fnft || choice.all {
        run_command(
            "./tests/foundry-nft-f23 -i src/ -x lib/ -o ./reports/nft-report.md",
            choice.release,
        )?;
    }
    if choice.fnft_icm || choice.all {
        run_command("./tests/foundry-nft-f23-icm -o ./reports/nft-report-icm.md", choice.release)?;
    }

    if choice.ccip || choice.all {
        run_command(
            "tests/ccip-contracts/contracts --src src/v0.8/functions/ -x tests/,test/,mocks/ -o ./reports/ccip-functions-report.md",
            choice.release,
        )?;
    }
    if choice.cpgu || choice.all {
        run_command_with_env(
            "tests/contract-playground/ -o ./reports/uniswap_profile.md",
            "FOUNDRY_PROFILE",
            "uniswap",
            choice.release,
        )?;
    }
    if choice.prb_math || choice.all {
        run_command("tests/prb-math -o reports/prb-math-report.md", choice.release)?;
    }
    if choice.tg || choice.all {
        run_command(
            "tests/2024-07-templegold/protocol -o reports/templegold-report.md",
            choice.release,
        )?;
    }
    if choice.hhpg || choice.all {
        run_command(
            "tests/hardhat-js-playground -o reports/hardhat-playground-report.md",
            choice.release,
        )?;
    }
    Ok(())
}
