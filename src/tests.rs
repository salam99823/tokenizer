use super::*;

#[test]
fn test_tokenize_1_level_of_indent() {
    let tokenizer = Tokenizer::new("for i in range(10):\n    print(i)");
    let actual_tokens = tokenizer.tokenize().unwrap();
    use Token::*;
    let expected_tokens = vec![
        Name("for".to_owned()),
        Name("i".to_owned()),
        Name("in".to_owned()),
        Name("range".to_owned()),
        OP("(".to_owned()),
        Number("10".to_owned()),
        OP(")".to_owned()),
        OP(":".to_owned()),
        NewLine,
        Indent("    ".to_owned()),
        Name("print".to_owned()),
        OP("(".to_owned()),
        Name("i".to_owned()),
        OP(")".to_owned()),
        NewLine,
        Dedent,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}
#[test]
fn test_tokenize_different_indent_levels() {
    let tokenizer = Tokenizer::new("level_1\n  level_2\n    level_3");
    let actual_tokens = tokenizer.tokenize().unwrap();
    use Token::*;
    let expected_tokens = vec![
        Name("level_1".to_owned()),
        NewLine,
        Indent("  ".to_owned()),
        Name("level_2".to_owned()),
        NewLine,
        Indent("    ".to_owned()),
        Name("level_3".to_owned()),
        NewLine,
        Dedent,
        Dedent,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}
#[test]
fn test_tokenize_different_strings() {
    let tokenizer =
        Tokenizer::new("'base 1'\"base 2\"r'raw 1'r\"raw 2\"b'byte 1'b\"byte 2\"");
    let actual_tokens = tokenizer.tokenize().unwrap();
    use Token::*;
    let expected_tokens = vec![
        String("'base 1'".to_owned()),
        String("\"base 2\"".to_owned()),
        String("r'raw 1'".to_owned()),
        String("r\"raw 2\"".to_owned()),
        String("b'byte 1'".to_owned()),
        String("b\"byte 2\"".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}
#[test]
fn test_tokenize_numbers() {
    let tokenizer =
        Tokenizer::new("'base 1'\"base 2\"r'raw 1'r\"raw 2\"b'byte 1'b\"byte 2\"");
    let actual_tokens = tokenizer.tokenize().unwrap();
    use Token::*;
    let expected_tokens = vec![
        String("'base 1'".to_owned()),
        String("\"base 2\"".to_owned()),
        String("r'raw 1'".to_owned()),
        String("r\"raw 2\"".to_owned()),
        String("b'byte 1'".to_owned()),
        String("b\"byte 2\"".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}
