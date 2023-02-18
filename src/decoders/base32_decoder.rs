use crate::checkers::CheckerTypes;
use crate::decoders::interface::check_string_success;

use super::crack_results::CrackResult;
///! Decodes a base32 string
///! Performs error handling and returns a string
///! Call base32_decoder.crack to use. It returns option<String> and check with
///! `result.is_some()` to see if it returned okay.
///
use super::interface::Crack;
use super::interface::Decoder;

use data_encoding::BASE32_NOPAD;
use log::{debug, info, trace};

/// The Base32 decoder, call:
/// `let base32_decoder = Decoder::<Base32Decoder>::new()` to create a new instance
/// And then call:
/// `result = base32_decoder.crack(input)` to decode a base32 string
/// The struct generated by new() comes from interface.rs
/// ```
/// use ares::decoders::base32_decoder::{Base32Decoder};
/// use ares::decoders::interface::{Crack, Decoder};
/// use ares::checkers::{athena::Athena, CheckerTypes, checker_type::{Check, Checker}};
///
/// let decode_base32 = Decoder::<Base32Decoder>::new();
/// let athena_checker = Checker::<Athena>::new();
/// let checker = CheckerTypes::CheckAthena(athena_checker);
///
/// let result = decode_base32.crack("NBSWY3DPEB3W64TMMQ======", &checker).unencrypted_text;
/// assert!(result.is_some());
/// assert_eq!(result.unwrap()[0], "hello world");
/// ```
pub struct Base32Decoder;

impl Crack for Decoder<Base32Decoder> {
    fn new() -> Decoder<Base32Decoder> {
        Decoder {
            name: "Base32",
            description: "Base32 is a group of binary-to-text encoding schemes that represent binary data (more specifically, a sequence of 8-bit bytes) in an ASCII string format by translating the data into a radix-32 representation.",
            link: "https://en.wikipedia.org/wiki/Base32",
            tags: vec!["base32", "decoder", "base"],
            popularity: 0.8,
            phantom: std::marker::PhantomData,
        }
    }

    /// This function does the actual decoding
    /// It returns an Option<string> if it was successful
    /// Else the Option returns nothing and the error is logged in Trace
    fn crack(&self, text: &str, checker: &CheckerTypes) -> CrackResult {
        trace!("Trying Base32 with text {:?}", text);
        let decoded_text = decode_base32_no_error_handling(text);
        let mut results = CrackResult::new(self, text.to_string());

        if decoded_text.is_none() {
            debug!("Failed to decode base32 because Base32Decoder::decode_base32_no_error_handling returned None");
            return results;
        }

        let decoded_text = decoded_text.unwrap();
        if !check_string_success(&decoded_text, text) {
            info!(
                "Failed to decode base32 because check_string_success returned false on string {}",
                decoded_text
            );
            return results;
        }

        let checker_result = checker.check(&decoded_text);
        results.unencrypted_text = Some(vec![decoded_text]);

        results.update_checker(&checker_result);

        results
    }
    /// Gets all tags for this decoder
    fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }
    /// Gets the name for the current decoder
    fn get_name(&self) -> &str {
        self.name
    }
}

/// helper function
fn decode_base32_no_error_handling(text: &str) -> Option<String> {
    // Strip all padding
    let text = text.replace('=', "");
    // Runs the code to decode base32
    // Doesn't perform error handling, call from_base32
    if let Ok(decoded_text) = &BASE32_NOPAD.decode(text.as_bytes()) {
        return Some(String::from_utf8_lossy(decoded_text).to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::Base32Decoder;
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
    fn base32_decodes_successfully() {
        // This tests if Base32 can decode Base32 successfully
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder.crack("NBSWY3DPEB3W64TMMQ======", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "hello world");
    }

    #[test]
    fn base32_decodes_no_padding_base32_successfully() {
        // This tests if Base32 can decode Base32 with no padding successfully
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result =
            base32_decoder.crack("KRUGS4ZANBQXGID2MVZG6IDQMFSGI2LOM4", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "This has zero padding");
    }

    #[test]
    fn base32_decodes_broken_padding_base32_successfully() {
        // This tests if Base32 can decode Base32 with broken padding successfully
        // Normally this string should have 4 equal signs instead of 2
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder.crack("JFXGG33SOJSWG5BAOBQWIZDJNZTQ==", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "Incorrect padding");
    }

    #[ignore]
    #[test]
    fn base32_decodes_tryhackme_base32_successfully() {
        // This tests if Base32 can decode Base32 with no padding successfully
        // The string is from the "breakit" THM room
        // TODO: Ignoring this until we have quadgrams
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder.crack("GM4HOU3VHBAW6OKNJJFW6SS2IZ3VAMTYORFDMUC2G44EQULIJI3WIVRUMNCWI6KGK5XEKZDTN5YU2RT2MR3E45KKI5TXSOJTKZJTC4KRKFDWKZTZOF3TORJTGZTXGNKCOE", &get_athena_checker());
        assert_eq!(result.unencrypted_text.unwrap()[0], "base16_is_hex");
    }

    #[test]
    fn base32_handles_panics() {
        // This tests if Base32 can handle panics
        // It should return None
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder
            .crack(
                "hello my name is panicky mc panic face!",
                &get_athena_checker(),
            )
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base32_handles_panic_if_empty_string() {
        // This tests if Base32 can handle an empty string
        // It should return None
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder
            .crack("", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }

    #[test]
    fn base32_handles_panic_if_emoji() {
        // This tests if Base32 can handle an emoji
        // It should return None
        let base32_decoder = Decoder::<Base32Decoder>::new();
        let result = base32_decoder
            .crack("😂", &get_athena_checker())
            .unencrypted_text;
        assert!(result.is_none());
    }
}
