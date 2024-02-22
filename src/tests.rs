use super::*;

#[test]
fn test_tokenize_1_level_of_indent() {
    let actual_tokens = tokenize("for i in range(10):\n    print(i)").unwrap();
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
    let actual_tokens = tokenize("level_1\n  level_2\n    level_3").unwrap();
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
    let actual_tokens =
        tokenize("'base 1'\"base 2\"r'raw 1'r\"raw \\n 2\"b'byte 1'b\"byte 2\"").unwrap();
    use Token::*;
    let expected_tokens = vec![
        String("'base 1'".to_owned()),
        String("\"base 2\"".to_owned()),
        String("r'raw 1'".to_owned()),
        String("r\"raw \\n 2\"".to_owned()),
        String("b'byte 1'".to_owned()),
        String("b\"byte 2\"".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}

#[test]
fn test_tokenize_numbers() {
    let actual_tokens = tokenize("1234567890 1.234 0.67890 1j 0.2e-2").unwrap();
    use Token::*;
    let expected_tokens = vec![
        Number("1234567890".to_owned()),
        Number("1.234".to_owned()),
        Number("0.67890".to_owned()),
        Number("1j".to_owned()),
        Number("0.2e-2".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}

#[test]
fn test_tokenize_fstring() {
    let actual_tokens = tokenize("f\"midle {2 + 2 = ?}\"").unwrap();
    use Token::*;
    let expected_tokens = vec![
        FStringStart("f\"".to_owned()),
        FStringMiddle("midle ".to_owned()),
        OP("{".to_owned()),
        Number("2".to_owned()),
        OP("+".to_owned()),
        Number("2".to_owned()),
        OP("=".to_owned()),
        OP("?".to_owned()),
        OP("}".to_owned()),
        FStringEnd("\"".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}

#[test]
fn test_tokenize_operators() {
    let actual_tokens = tokenize(OPERATORS).unwrap();
    use Token::*;
    let expected_tokens = vec![
        OP("=".to_owned()),
        OP("+".to_owned()),
        OP("-".to_owned()),
        OP("*".to_owned()),
        OP("/".to_owned()),
        OP("%".to_owned()),
        OP("&".to_owned()),
        OP("|".to_owned()),
        OP("<>".to_owned()),
        OP(">".to_owned()),
        OP("!".to_owned()),
        OP("^".to_owned()),
        OP(":".to_owned()),
        OP(";".to_owned()),
        OP(".".to_owned()),
        OP(",".to_owned()),
        OP("(".to_owned()),
        OP(")".to_owned()),
        OP("[".to_owned()),
        OP("]".to_owned()),
        OP("{".to_owned()),
        OP("}".to_owned()),
        OP("@".to_owned()),
        OP("$".to_owned()),
        OP("?".to_owned()),
        OP("~".to_owned()),
        OP("`".to_owned()),
        NewLine,
        EndMarker,
    ];
    assert_eq!(actual_tokens, expected_tokens);
}
