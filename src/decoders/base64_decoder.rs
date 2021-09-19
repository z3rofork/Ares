use crate::decoders::interface::check_string_success;

///! Decode a base64 string
///! Performs error handling and returns a string
///! Call base64_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use super::interface::Crack;
use super::interface::Decoder;

use log::{info, trace, debug};

/// .decoder is never used, so Rust considers this dead code
/// Really it's just a co-reference to the Decoder in `interface.rs`
#[allow(dead_code)]
pub struct Base64Decoder {
    decoder: Decoder,
}

/// The Base64 decoder, call:
/// `let base64_decoder = Base64Decoder.new()` to create a new instance
/// And then call:
/// `result = base64_decoder.crack(input)` to decode a base64 string
/// The struct generated by new() comes from interface.rs
/// ```compile_fail
/// use ares::decoders::base64_decoder::{Base64Decoder};
/// let decode_base64 = Base64Decoder::new();
/// let result = decode_base64.crack("aGVsbG8gd29ybGQ=").unwrap();
/// assert_eq!(result, "hello world");
/// ```
impl Base64Decoder {
    pub fn new() -> Self {
        Self {
            decoder: Decoder {
                name: "base64",
                description: " Base64 is a group of binary-to-text encoding schemes that represent binary data (more specifically, a sequence of 8-bit bytes) in an ASCII string format by translating the data into a radix-64 representation.",
                link: "https://en.wikipedia.org/wiki/Base64",
                tags: vec!["base64", "decoder", "baser"],
                expected_runtime: 0.01,
                expected_success: 1.0,
                failure_runtime: 0.01,
                normalised_entropy: vec![1.0, 10.0],
                popularity: 1.1,
            },
        }
    }

    fn decode_base64_no_error_handling(text: &str) -> Option<String>{
        // Runs the code to decode base64
        // Doesn't perform error handling, call from_base64
        base64::decode(text.as_bytes())
        .ok()
        .map(|inner| String::from_utf8(inner).ok())?
    }
}

impl Crack for Base64Decoder {
    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str) -> Option<String> {
        trace!("Trying Base64 with text {:?}", text);
        let decoded_text = Base64Decoder::decode_base64_no_error_handling(text);
        
        if decoded_text.is_none() {
            debug!("Failed to decode base64 because Base64Decoder::decode_base64_no_error_handling returned None");
            return None;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!("Failed to decode base64 because check_string_success returned false on string {}", decoded_text);
            return None;
        }

        return Some(decoded_text);
    }
}

#[cfg(test)]
mod tests {
    use super::Base64Decoder;
    use crate::decoders::interface::Crack;

    #[test]
    fn it_works() {
        let base64_decoder = Base64Decoder::new();
        let _result = base64_decoder.crack("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(true, true);
    }

    #[test]
    fn successful_decoding() {
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("aGVsbG8gd29ybGQ=").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn base64_decode_empty_string() {
        // Bsae64 returns an empty string, this is a valid base64 string
        // but returns False on check_string_success
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("");
        assert!(result.is_none());
    }

    #[test]
    fn base64_decode_handles_panics() {
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("hello my name is panicky mc panic face!");
        if result.is_some() {
            panic!("Decode_base64 did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the base64_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_handle_panic_if_empty_string() {
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("");
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_work_if_string_not_base64() {
        // You can base64 decode a string that is not base64
        // This string decodes to:
        // ```.ée¢
        // (uÖ²```
        // https://gchq.github.io/CyberChef/#recipe=From_Base64('A-Za-z0-9%2B/%3D',true)&input=aGVsbG8gZ29vZCBkYXkh
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("hello good day!");
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base64_handle_panic_if_emoji() {
        let base64_decoder = Base64Decoder::new();
        let result = base64_decoder.crack("😂");
        if result.is_some() {
            assert_eq!(true, true);
        }
    }
}