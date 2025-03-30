#![allow(clippy::unwrap_used)]
use std::{io::Write, panic::PanicHookInfo};
use termcolor::{Color, ColorSpec, WriteColor};

use std::io::IsTerminal;

use termcolor::{BufferWriter, ColorChoice};

pub fn stderr_buffer_writer() -> BufferWriter {
    // Prefer to add colors to the output only if it is forced via an environment variable or
    // because it's a terminal

    let color_choice = {
        if std::env::var("FORCE_COLOR").is_ok_and(|e| !e.is_empty()) {
            ColorChoice::Always
        } else if std::io::stderr().is_terminal() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }
    };

    BufferWriter::stderr(color_choice)
}

pub fn add_handler() {
    std::panic::set_hook(Box::new(move |info: &PanicHookInfo<'_>| {
        print_compiler_bug_message(info)
    }));
}

fn print_compiler_bug_message(info: &PanicHookInfo<'_>) {
    let message =
        match (info.payload().downcast_ref::<&str>(), info.payload().downcast_ref::<String>()) {
            (Some(s), _) => (*s).to_string(),
            (_, Some(s)) => s.to_string(),
            (None, None) => "unknown error".into(),
        };

    let location = match info.location() {
        None => "".into(),
        Some(location) => format!("{}:{}\n\t", location.file(), location.line()),
    };

    let buffer_writer = stderr_buffer_writer();
    let mut buffer = buffer_writer.buffer();
    buffer.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red))).unwrap();
    write!(buffer, "error").unwrap();
    buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
    write!(buffer, ": Fatal compiler bug!\n\n").unwrap();
    buffer.set_color(&ColorSpec::new()).unwrap();
    writeln!(
        buffer,
        "This is a fatal bug in Aderyn, sorry!

Please report this crash to https://github.com/cyfrin/aderyn/issues/new and include this error message with your report.

Panic: {location}{message}
Aderyn version: {version}
Operating system: {os}

If you can also share your code and say what file you were editing or any
steps to reproduce the crash that would be a great help.

You may also want to try again with the `ADERYN_LOG=trace` environment
variable set.
",
        location = location,
        message = message,
        version = env!("CARGO_PKG_VERSION"),
        os = std::env::consts::OS,
    )
    .unwrap();
    buffer_writer.print(&buffer).unwrap();
}
