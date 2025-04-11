//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not expressible with just
//! `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in `.cargo/config`.

#![allow(unreachable_pub, unexpected_cfgs)]

mod blesspr;
mod cut_release;
mod flags;
mod reportgen;
mod tomlgen;

fn main() -> anyhow::Result<()> {
    let flags = flags::Xtask::from_env_or_exit();
    match flags.subcommand {
        flags::XtaskCmd::Reportgen(choice) => reportgen::reportgen(choice),
        flags::XtaskCmd::CutRelease(cut_release) => cut_release::cut_release(cut_release),
        flags::XtaskCmd::Blesspr(..) => blesspr::blesspr(),
        flags::XtaskCmd::Tomlgen(..) => tomlgen::init_toml_files(),
    }
}
