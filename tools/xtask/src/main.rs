//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not expressible with just
//! `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in `.cargo/config`.

#![allow(unreachable_pub, unexpected_cfgs)]

use xshell::{Shell, cmd};

mod flags;

fn run_command(args: &str) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let mut cmd = cmd!(sh, "cargo run");
    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd.args(args.split(" ")).run()?;
    Ok(())
}

fn run_command_with_env(args: &str, key: &str, val: &str) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let mut cmd = cmd!(sh, "cargo run");
    cmd = cmd.env(key, val);
    cmd = cmd.arg("--").arg("--skip-update-check");
    cmd.args(args.split(" ")).run()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();
    match flags.subcommand {
        flags::XtaskCmd::R(choice) => {
            if choice.cpg || choice.all {
                run_command("-i src/ -x lib/ ./tests/contract-playground -o ./reports/report.md")?;
            }
            if choice.adhoc || choice.all {
                run_command("./tests/adhoc-sol-files -o ./reports/adhoc-sol-files-report.md")?;
            }
            if choice.sablier || choice.all {
                run_command(
                    "./tests/2024-05-Sablier -o ./reports/sablier-aderyn-toml-nested-root.md",
                )?;
            }
            if choice.fnft || choice.all {
                run_command("./tests/foundry-nft-f23 -i src/ -x lib/ -o ./reports/nft-report.md")?;
            }
            if choice.fnft_icm || choice.all {
                run_command("./tests/foundry-nft-f23-icm -o ./reports/nft-report-icm.md")?;
            }

            if choice.ccip || choice.all {
                run_command(
                    "tests/ccip-contracts/contracts --src src/v0.8/functions/ -x tests/,test/,mocks/ -o ./reports/ccip-functions-report.md",
                )?;
            }
            if choice.cpgu || choice.all {
                run_command_with_env(
                    "tests/contract-playground/ -o ./reports/uniswap_profile.md",
                    "FOUNDRY_PROFILE",
                    "uniswap",
                )?;
            }
            if choice.prb_math || choice.all {
                run_command("tests/prb-math -o reports/prb-math-report.md")?;
            }
            if choice.tg || choice.all {
                run_command("tests/2024-07-templegold/protocol -o reports/templegold-report.md")?;
            }
            if choice.hhpg || choice.all {
                run_command("tests/hardhat-js-playground -o reports/hardhat-playground-report.md")?;
            }
        }
    }
    Ok(())
}
