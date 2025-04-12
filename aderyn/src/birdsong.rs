use indoc::indoc;

pub const MESSAGES: [&str; 8] = [
    indoc! {"
        Aderyn saved you hours of headaches.
        ☕ Buy us a virtual coffee with a GitHub star:
    "},
    indoc! {"
        🐦 Aderyn finds bugs. You write safer code.
        ⭐ Star us on GitHub if that sounds fair:
    "},
    indoc! {"
        🛡️ Together we make Solidity safer.
        ✨ Support open-source security — star Aderyn:
    "},
    indoc! {"
        ✔  Done! Fast, clean, effective.
        ⭐ Support the project with a star:
    "},
    indoc! {"
        🌱 Aderyn is growing with community support.
        Help others discover it — drop a star on GitHub:
    "},
    indoc! {"
        ✔ Aderyn did its job. Solidity's a bit safer now.
        💫 Like the tool? A star on GitHub goes a long way:
    "},
    indoc! {"
        🪶 Aderyn spotted all it could. Stay secure.
        If our little bird helped you out, give it a perch with a GitHub Star ⭐
    "},
    indoc! {"
        💡 Did Aderyn help you today?
        Show some love with a quick star on GitHub Star ⭐
    "},
];

use std::time::{SystemTime, UNIX_EPOCH};

pub fn print_last_words() {
    let random_in_range = |min: usize, max: usize| -> usize {
        let now =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos();

        let hash = (now ^ (now >> 3) ^ (now << 7)) as usize;
        min + (hash % (max - min))
    };
    let message_idx = random_in_range(0, MESSAGES.len());
    let message = MESSAGES[message_idx];
    print!("\n{}", message);
    println!("https://github.com/Cyfrin/aderyn");
}
