#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    // fuzzed code goes here
    _ = wgsl_parser::parse_file(data);
});
