#![no_main]

use libfuzzer_sys::fuzz_target;
use solana_parser::{decode_idl_data, parse_instruction_with_idl};

const IDL_JSON: &str = r#"{
    "instructions": [{
        "name": "transfer",
        "discriminator": [163,52,195,99,237,202,219,100],
        "accounts": [],
        "args": [{"name": "amount", "type": "u64"}]
    }],
    "types": []
}"#;

const PROGRAM_ID: &str = "11111111111111111111111111111111";

fuzz_target!(|data: &[u8]| {
    if let Ok(idl) = decode_idl_data(IDL_JSON) {
        let _ = parse_instruction_with_idl(data, PROGRAM_ID, &idl);
    }
});
