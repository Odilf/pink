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
    let input = "
		domain { d1, d2, d3 }
		reserve { r1, r2, r3 }

		r1 x r2 = r2 d d2;
	";

    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    let def = definition("r1 x r2 = r2 d d2;", &domain, &reserved)
        .unwrap()
        .1;

    let expected = Structure::new(domain, reserved, vec![def]).unwrap();

    assert_eq!(Ok(expected), parse(input.to_owned()));
}

#[test]
fn parse_with_comments() {
    let input = "
		# These comments should be ignored
		domain { d1, d2, # comment
			 d3 } # comment
		# comment
		# comment
		reserve { r1, r2, r3 } # comment
		# comment

		r1 x r2 = # comment
		r2 d d2;


	";

    let (_, domain) = domain("domain { d1, d2, d3 }").unwrap();
    let (_, reserved) = reserve("reserve { r1, r2, r3 }").unwrap();

    let def = definition("r1 x r2 = r2 d d2;", &domain, &reserved)
        .unwrap()
        .1;

    let expected = Structure::new(domain, reserved, vec![def]).unwrap();

    assert_eq!(Ok(expected), parse(input.to_owned()));
}

#[test]
fn parse_core() {
    dbg!(parse_file("standard_library/Core.pink").unwrap());
}
