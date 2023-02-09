use crate::engine::Token;

use super::*;

#[test]
fn domain_test() {
    assert_eq!(
        (
            "",
            BTreeSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()])
        ),
        domain("domain { a, b, c }").unwrap()
    );
}

#[test]
fn domain_test_rest() {
    assert_eq!(
        (
            " and something else later",
            BTreeSet::from(["a".to_owned(), "b".to_owned(), "c".to_owned()])
        ),
        domain("domain { a, b, c } and something else later").unwrap()
    );
}

#[test]
fn empty_domain() {
    assert_eq!(("", BTreeSet::new()), domain("domain { }").unwrap());
}

#[test]
fn pattern_test() {
    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    assert_eq!(
        vec![PatternToken::Concrete(Token::Literal("r1".to_owned()))],
        pattern("r1", &domain, &reserved)
    );

    assert_eq!(
        vec![
            PatternToken::Concrete(Token::Literal("r1".to_owned())),
            PatternToken::Concrete(Token::Element("d2".to_owned())),
            PatternToken::Variable("x".to_owned()),
            PatternToken::Concrete(Token::Literal("r3".to_owned())),
        ],
        pattern("r1 d2 x r3", &domain, &reserved)
    );
}

#[test]
fn pattern_with_comments() {
    let input = "r1 d2 x # this is a comment
	r3 					 # and should be ignored";

    let input = strip_comments(input.to_owned());

    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    assert_eq!(
        vec![PatternToken::Concrete(Token::Literal("r1".to_owned()))],
        pattern("r1", &domain, &reserved)
    );

    assert_eq!(
        vec![
            PatternToken::Concrete(Token::Literal("r1".to_owned())),
            PatternToken::Concrete(Token::Element("d2".to_owned())),
            PatternToken::Variable("x".to_owned()),
            PatternToken::Concrete(Token::Literal("r3".to_owned())),
        ],
        pattern(input.as_str(), &domain, &reserved)
    );
}

#[test]
fn simple_definition() {
    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    let input = "r1 x r2 = r2 d d2; lol";

    let lhs = pattern("r1 x r2", &domain, &reserved);
    let rhs = pattern("r2 d d2", &domain, &reserved);

    let expected = Definition::new(lhs, rhs);

    assert_eq!(
        (" lol", expected),
        definition(input, &domain, &reserved).unwrap(),
    );
}

#[test]
fn multi_line_definition() {
    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    let input = "r1 x r2 = 
	
	r2 d d2; lol";

    let lhs = pattern("r1 x r2", &domain, &reserved);
    let rhs = pattern("r2 d d2", &domain, &reserved);

    let expected = Definition::new(lhs, rhs);

    assert_eq!(
        (" lol", expected),
        definition(input, &domain, &reserved).unwrap(),
    );
}

#[test]
fn whole_parse_test() {
    let input_path = "src/parser/test_files/test1.pink";

    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    let def = definition("r1 x r2 = r2 d d2;", &domain, &reserved)
        .unwrap()
        .1;

    let expected = Runtime::new(BTreeMap::from([
        ("intrinsic".into(), Structure::intrinsic()),
        (
            "test1".into(),
            Structure::create(domain, reserved, vec![def]).unwrap(),
        ),
    ]));

    let result = parse_file(input_path.into()).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn parse_dependencies() {
    let input_path = "src/parser/test_files/test2.pink";

    let test1_structures = parse_file("src/parser/test_files/test1.pink".into())
        .unwrap()
        .structures;
    let s1 = test1_structures.get("test1").unwrap();

    let (_, domain) = domain("domain { d4, d5, d6 }").unwrap();
    let (_, reserved) = reserve("reserve { r4, r5, r6 }").unwrap();

    let def = definition("r4 = d4 r5;", &domain, &reserved).unwrap().1;

    let expected = Runtime::new(BTreeMap::from([
        ("intrinsic".into(), Structure::intrinsic()),
        ("test1".into(), s1.clone()),
        (
            "test2".into(),
            Structure::create(domain, reserved, vec![def]).unwrap(),
        ),
    ]));

    let result = parse_file(input_path.into()).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn parse_parent() {
    let input_path = "src/parser/test_files/test3/nested.pink";

    let test1_structures = parse_file("src/parser/test_files/test1.pink".into())
        .unwrap()
        .structures;
    let s1 = test1_structures.get("test1").unwrap();

    let domain = BTreeSet::new();
    let reserved = BTreeSet::new();

    let expected = Runtime::new(BTreeMap::from([
        ("intrinsic".into(), Structure::intrinsic()),
        ("../test1".into(), s1.clone()),
        (
            "nested".into(),
            Structure::create(domain, reserved, Vec::new()).unwrap(),
        ),
    ]));

    let result = parse_file(input_path.into()).unwrap();

    assert_eq!(expected, result);
}

#[test]
fn get_name_and_root_test() {
    let (root, name) = get_root_and_name("src/parser/test_files/test1.pink".into()).unwrap();

    assert_eq!("test1", name);
    assert_eq!(PathBuf::from("src/parser/test_files"), root);
}

#[test]
fn parse_cycle() {
    let input_path = "src/parser/test_files/cycle1.pink";

    parse_file(input_path.into()).expect_err("Should find the circular dependency");
}
