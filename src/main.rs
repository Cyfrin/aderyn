use aderyn::config::run;

fn main() {
    if let Err(_e) = run() {
        std::process::exit(1);
    }
}