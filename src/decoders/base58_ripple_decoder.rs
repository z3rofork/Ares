use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
///! Decodes a base58 ripple string
///! Performs error handling and returns a string
///! Call base58_ripple_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use super::interface::Crack;
use super::interface::Decoder;

use log::{debug, info, trace};

/// The Base58_ripple decoder, call:
/// `let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new()` to create a new instance
/// And then call:
/// `result = base58_ripple_decoder.crack(input)` to decode a base58_ripple string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::base58_ripple_decoder::{Base58RippleDecoder};
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base58_ripple = Decoder::<Base58RippleDecoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base58_ripple.crack("StVrDLaUATiyKyV", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap(), "hello world");
/// ```
pub struct Base58RippleDecoder;

impl Crack for Decoder<Base58RippleDecoder> {
    fn new() -> Decoder<Base58RippleDecoder> {
        Decoder {
            name: "base58_ripple",
            description: "Base58 is a group of binary-to-text encoding schemes that represent binary data (more specifically, a sequence of 8-bit bytes) in an ASCII string format by translating the data into a radix-32 representation.",
            link: "https://en.wikipedia.org/wiki/Base58",
            tags: vec!["base58_ripple", "base58", "ripple", "cryptocurrency", "decoder", "base"],
            expected_runtime: 0.01,
            expected_success: 0.7,
            failure_runtime: 0.01,
            normalised_entropy: vec![1.0, 10.0],
            popularity: 0.8,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Base58_ripple with text {:?}", text);
        let decoded_text = decode_base58_ripple_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base58_ripple because Base58RippleDecoder::decode_base58_ripple_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base58_ripple because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(decoded_text);

        results.update_checker(&checker_result);

        results
    }
}

/// helper function
fn decode_base58_ripple_no_error_handling(text: &str) -> Option<String> {
    // Runs the code to decode base58_ripple
    // Doesn't perform error handling, call from_base58_ripple
    if let Ok(decoded_text) = bs58::decode(text)
        .with_alphabet(bs58::Alphabet::RIPPLE)
        .into_vec()
    {
        return Some(String::from_utf8_lossy(&decoded_text).to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Base58RippleDecoder;
    use crate::{
        checkers::{
            athena::Athena,
            checker_type::{Check, Checker},
            CheckerTypes,
        },
        decoders::interface::{Crack, Decoder},
    };

    // helper for tests
    fn get_athena_checker() -> CheckerTypes {
        let athena_checker = Checker::<Athena>::new();
        CheckerTypes::CheckAthena(athena_checker)
    }

    #[test]
    fn successful_decoding() {
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();

        let result = base58_ripple_decoder.crack("StVrDLaUATiyKyV", &get_athena_checker());
        let decoded_str = &result
            .unencrypted_text
            .expect("No unencrypted text for base58_ripple");
        assert_eq!(decoded_str, "hello world");
    }

    #[test]
    fn base58_ripple_decode_empty_string() {
        // Bsae58_ripple returns an empty string, this is a valid base58_ripple string
        // but returns False on check_string_success
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();
        let result = base58_ripple_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base58_ripple_decode_handles_panics() {
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();
        let result = base58_ripple_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        if result.is_some() {
            panic!("Decode_base58_ripple did not return an option with Some<t>.")
        } else {
            // If we get here, the test passed
            // Because the base58_ripple_decoder.crack function returned None
            // as it should do for the input
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base58_ripple_handle_panic_if_empty_string() {
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();
        let result = base58_ripple_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base58_ripple_work_if_string_not_base58_ripple() {
        // You can base58_ripple decode a string that is not base58_ripple
        // This string decodes to:
        // ```.ée¢
        // (uÖ²```
        // https://gchq.github.io/CyberChef/#recipe=From_Base58('A-Za-z0-9%2B/%3D',true)&input=aGVsbG8gZ29vZCBkYXkh
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();
        let result = base58_ripple_decoder
            .crack("hello good day!", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }

    #[test]
    fn base58_ripple_handle_panic_if_emoji() {
        let base58_ripple_decoder = Decoder::<Base58RippleDecoder>::new();
        let result = base58_ripple_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        if result.is_some() {
            assert_eq!(true, true);
        }
    }
}