#![no_main]

use libfuzzer_sys::fuzz_target;
use solana_parser::parse_transaction;

// Feed arbitrary bytes interpreted as a base64-encoded transaction.
// Goals: no panics in deserialization, instruction dispatch, or IDL lookup.
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_transaction(s.to_string(), false, None);
    }
});
