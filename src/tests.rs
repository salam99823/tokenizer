use super::*;

#[test]
fn tokenizer_work() {
    let tokenizer = Tokenizer::new(
        "some_name\
        \n'in single-quote'\
        \n\"in doble-quote\"\
        \n2 + 2\
        \nfor i in range(10):
    print(i)\n\n".to_owned()
    );
    let tokens = tokenizer.tokenize().unwrap();
    use Token::*;
    let expects = vec![
        Name("some_name".to_owned()),
        NewLine,
        String("'in single-quote'".to_owned()),
        NewLine,
        String("\"in doble-quote\"".to_owned()),
        NewLine,
        Number("2".to_owned()),
        OP("+".to_owned()),
        Number("2".to_owned()),
        NewLine,
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
        NL,
        EndMarker,
    ];
    for (actual, expect) in tokens.iter().zip(expects.iter()) {
        assert_eq!(actual, expect);
    }
}