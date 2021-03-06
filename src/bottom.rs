use std::error::Error;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/maps.rs"));

#[derive(Debug)]
pub struct TranslationError {
    pub why: String,
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl Error for TranslationError {}

pub fn encode_byte(value: u8) -> &'static str {
    &BYTE_TO_EMOJI[value as usize]
}

pub fn decode_byte(input: &dyn AsRef<str>) -> Result<u8, TranslationError> {
    let input_ref = input.as_ref();
    let result = EMOJI_TO_BYTE.get(input_ref).ok_or_else(|| TranslationError {
        why: format!("Cannot decode character {}", input_ref),
    })?;
    Ok(*result)
}

pub fn encode_string(input: &dyn AsRef<str>) -> String {
    input.as_ref().bytes().map(encode_byte).collect::<String>()
}

pub fn delongate(input: &str) -> String {
    return input
        .replace(":sparkling_heart:", "💖")
        .replace(":pleading_face:", "🥺")
        .replace(":point_left:", "👈")
        .replace(":point_right:", "👉")
        .replace(":sparkles:", "✨");
}

pub fn decode_string_long<'a>(input: &dyn AsRef<str>, do_delongate: bool) -> Result<String, TranslationError> {
    let mut input = input.as_ref();
    let new_input;
    if do_delongate{
        new_input = delongate(input);
        input = new_input.as_ref();
    }
    let result = {
        // Older versions used a ZWSP as a character separator, instead of `👉👈`.
        let split_char = input
            .chars()
            .find(|&c| c == '\u{200b}' || c == '👉');

        if let Some('\u{200b}') = split_char {
            input.trim_end_matches("\u{200B}").split("\u{200B}")
        } else {
            input.trim_end_matches("👉👈").split("👉👈")
        }
    }
    .map(|c| decode_byte(&c))
    .collect::<Result<Vec<u8>, _>>()?;

    Ok(String::from_utf8_lossy(&result).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode_string<'a>(input: &dyn AsRef<str>) -> Result<String, TranslationError> {
        return decode_string_long(input, false);
    }

    #[test]
    fn test_string_encode() {
        assert_eq!(
            encode_string(&"Test"),
            "💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈"
        );
    }

    #[test]
    fn test_byte_encode() {
        assert_eq!(encode_byte(b'h'), "💖💖,,,,👉👈",);
    }

    #[test]
    fn test_char_decode() {
        assert_eq!(decode_byte(&"💖💖,,,,").unwrap(), b'h',);
    }

    #[test]
    fn test_string_decode() {
        // Test that we haven't killed backwards-compat
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,\u{200B}💖💖,\u{200B}💖💖✨🥺\u{200B}💖💖✨🥺,\u{200B}")
                .unwrap(),
            "Test"
        );
        assert_eq!(
            decode_string(&"💖✨✨✨,,,,👉👈💖💖,👉👈💖💖✨🥺👉👈💖💖✨🥺,👉👈").unwrap(),
            "Test"
        );
    }

    #[test]
    fn test_unicode_string_encode() {
        assert_eq!(
            encode_string(&"🥺"),
            "🫂✨✨✨✨👉👈💖💖💖🥺,,,,👉👈💖💖💖✨🥺👉👈💖💖💖✨✨✨🥺,👉👈"
        );
        assert_eq!(
            encode_string(&"がんばれ"),
            "🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈\
            💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈\
            💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈"
        );
    }

    #[test]
    fn test_unicode_string_decode() {
        assert_eq!(
            decode_string(&"🫂✨✨✨✨👉👈💖💖💖🥺,,,,👉👈💖💖💖✨🥺👉👈💖💖💖✨✨✨🥺,👉👈")
                .unwrap(),
            "🥺",
        );
        assert_eq!(
            decode_string(
                &"🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈💖💖✨✨✨✨👉👈🫂✨✨🥺,,👉👈\
            💖💖✨✨✨👉👈💖💖✨✨✨✨🥺,,👉👈🫂✨✨🥺,,👉👈💖💖✨✨🥺,,,,👉👈\
            💖💖💖✨✨🥺,👉👈🫂✨✨🥺,,👉👈💖💖✨✨✨👉👈💖💖✨✨✨✨👉👈"
            )
            .unwrap(),
            "がんばれ",
        );
    }

    #[test]
    fn test_long_names() {
        assert_eq!(
            decode_string_long(&":sparkling_heart::sparkles::sparkles::pleading_face:,:point_right::point_left::sparkling_heart::sparkling_heart::sparkles:,:point_right::point_left::sparkling_heart::sparkling_heart::sparkles::point_right::point_left::sparkling_heart::sparkling_heart:,,,:point_right::point_left:", true)
                .unwrap(),
            "Long",
        );
    }
}
