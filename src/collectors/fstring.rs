use crate::{privat::ModPeekable, tokenize, Result, Token, TokenizeError};

pub fn collect_fstring(iter: &mut ModPeekable, tokens: &mut Vec<Token>) -> Result<()> {
    let quot = iter.next().unwrap();

    tokens.push(Token::FStringStart(format!("f{}", quot)));
    while let Some(c) = iter.next_if(|c| *c != quot) {
        match c {
            '{' => {
                let mut inner = String::new();
                inner.push('{');
                while let Some(c) = iter.next_if(|c| *c != '}') {
                    inner.push(c);
                }
                inner.push(iter.next().unwrap());
                tokens.extend(match tokenize(inner) {
                    Ok(i) => i,
                    Err(e) => {
                        return Err({
                            let pos = iter.pos();
                            match e {
                                TokenizeError::EscapeSeq(msg, (iter_num, char_num)) => {
                                    TokenizeError::EscapeSeq(
                                        msg,
                                        (iter_num + pos.0, char_num + pos.1),
                                    )
                                }
                                TokenizeError::String(msg, (iter_num, char_num)) => {
                                    TokenizeError::String(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                TokenizeError::Number(msg, (iter_num, char_num)) => {
                                    TokenizeError::Number(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                TokenizeError::Operator(msg, (iter_num, char_num)) => {
                                    TokenizeError::Operator(
                                        msg,
                                        (iter_num + pos.0, char_num + pos.1),
                                    )
                                }
                                TokenizeError::Char(msg, (iter_num, char_num)) => {
                                    TokenizeError::Char(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                TokenizeError::Indent(msg, (iter_num, char_num)) => {
                                    TokenizeError::Indent(msg, (iter_num + pos.0, char_num + pos.1))
                                }
                                TokenizeError::EndOfFile(msg, (iter_num, char_num)) => {
                                    TokenizeError::EndOfFile(
                                        msg,
                                        (iter_num + pos.0, char_num + pos.1),
                                    )
                                }
                            }
                        })
                    }
                });
                tokens.pop();
                tokens.pop();
            }
            c => {
                let mut fstring_midle = String::new();
                fstring_midle.push(c);
                while let Some(c) = iter.next_if(|c| *c != quot && *c != '{') {
                    if c == '\n' {
                        return Err(TokenizeError::String(
                            "Unclosed string".to_owned(),
                            *iter.pos(),
                        ));
                    }
                    fstring_midle.push(c);
                }
                if !fstring_midle.is_empty() {
                    tokens.push(Token::FStringMiddle(fstring_midle))
                }
            }
        }
    }

    iter.next();
    tokens.push(Token::FStringEnd(quot.to_string()));
    Ok(())
}
