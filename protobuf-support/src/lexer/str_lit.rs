use std::fmt;
use std::string::FromUtf8Error;

use crate::lexer::lexer_impl::Lexer;
use crate::lexer::parser_language::ParserLanguage;

#[derive(Debug, thiserror::Error)]
pub enum StrLitDecodeError {
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("String literal decode error")]
    OtherError,
}

pub type StrLitDecodeResult<T> = Result<T, StrLitDecodeError>;

/// String literal, both `string` and `bytes`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct StrLit {
    pub char_or_escape_seq: String,
}

impl fmt::Display for StrLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", &self.char_or_escape_seq)
    }
}

impl StrLit {
    pub fn to_string(&self) -> StrLitDecodeResult<String> {
	// XXX: need a treat for escaped characters.
	Ok(String::from(&self.char_or_escape_seq))
    }

    pub fn to_bytes(&self) -> StrLitDecodeResult<Vec<u8>> {
	Ok(self.char_or_escape_seq.as_bytes().to_vec())
    }

    /// May fail if not valid UTF8
    pub fn decode_utf8(&self) -> StrLitDecodeResult<String> {
        let mut lexer = Lexer::new(&self.char_or_escape_seq, ParserLanguage::Json);
        let mut r = Vec::new();
        while !lexer.eof() {
            r.push(
                lexer
                    .next_byte_value()
                    .map_err(|_| StrLitDecodeError::OtherError)?,
            );
        }
        Ok(String::from_utf8(r)?)
    }

    pub fn decode_bytes(&self) -> StrLitDecodeResult<Vec<u8>> {
        let mut lexer = Lexer::new(&self.char_or_escape_seq, ParserLanguage::Json);
        let mut r = Vec::new();
        while !lexer.eof() {
            r.push(
                lexer
                    .next_byte_value()
                    .map_err(|_| StrLitDecodeError::OtherError)?,
            );
        }
        Ok(r)
    }

    pub fn quoted(&self) -> String {
        format!("\"{}\"", self.char_or_escape_seq)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::str_lit::StrLit;

    #[test]
    fn decode_utf8() {
        assert_eq!(
            "\u{1234}".to_owned(),
            StrLit {
                char_or_escape_seq: "\\341\\210\\264".to_owned()
            }
            .decode_utf8()
            .unwrap()
        )
    }
}
