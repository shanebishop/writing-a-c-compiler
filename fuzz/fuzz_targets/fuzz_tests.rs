#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(tokens) = lexer::tokenize_str(s) {
            let _ = parser::parse(&tokens);
        }
    }
});
