use super::*;

#[test]
fn match_empty() {
	let pattern = vec![];
	let expression = vec![];

	let bindings = get_match_bindings(&pattern, &expression);

	assert!(bindings.is_some());
}

#[test]
fn match_literals() {
	let pattern = vec![PatternToken::Concrete(Token::Literal("token".to_owned()))];
	let expression = vec![Token::Literal("token".to_owned())];

	let bindings = get_match_bindings(&pattern, &expression);

	assert!(bindings.is_some());
}

#[test]
fn match_3_literals() {
	let pattern = vec![
		PatternToken::Concrete(Token::Literal("token_1".to_owned())),
		PatternToken::Concrete(Token::Literal("token_2".to_owned())),
		PatternToken::Concrete(Token::Literal("token_3".to_owned())),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("token_3".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	assert!(bindings.is_some());
}

#[test]
fn capture_at_middle() {
	let pattern = vec![
		PatternToken::Concrete(Token::Literal("token_1".to_owned())),
		PatternToken::Variable("p".to_owned()),
		PatternToken::Concrete(Token::Literal("token_3".to_owned())),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("token_3".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	let p = vec![Token::Literal("token_2".to_owned())];
	let expected = BTreeMap::from([
		("p".to_owned(), p.as_slice())
	]);

	assert_eq!(bindings, Some(expected));
}

#[test]
fn capture_at_start() {
	let pattern = vec![
		PatternToken::Variable("p".to_owned()),
		PatternToken::Concrete(Token::Literal("token_2".to_owned())),
		PatternToken::Concrete(Token::Literal("token_3".to_owned())),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("token_3".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	let p = vec![Token::Literal("token_1".to_owned())];
	let expected = BTreeMap::from([
		("p".to_owned(), p.as_slice())
	]);

	assert_eq!(bindings, Some(expected));
}

#[test]
fn capture_single() {
	let pattern = vec![
		PatternToken::Variable("x".to_owned()),
	];

	let expression = vec![
		Token::Literal("token".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	let x = vec![Token::Literal("token".to_owned())];
	let expected = BTreeMap::from([
		("x".to_owned(), x.as_slice())
	]);

	assert_eq!(bindings, Some(expected));
}

#[test]
fn capture_at_end() {
	let pattern = vec![
		PatternToken::Concrete(Token::Literal("token_1".to_owned())),
		PatternToken::Concrete(Token::Literal("token_2".to_owned())),
		PatternToken::Variable("p".to_owned()),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("token_3".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	let p = vec![Token::Literal("token_3".to_owned())];
	let expected = BTreeMap::from([
		("p".to_owned(), p.as_slice())
	]);

	assert_eq!(bindings, Some(expected));
}


#[test]
fn match_capture() {
	let pattern = vec![
		PatternToken::Concrete(Token::Literal("token_1".to_owned())),
		PatternToken::Variable("p".to_owned()),
		PatternToken::Concrete(Token::Literal("token_2".to_owned())),
		PatternToken::Variable("p".to_owned()),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("variable".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("variable".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	let p = vec![Token::Literal("variable".to_owned())];
	let expected = BTreeMap::from([
		("p".to_owned(), p.as_slice())
	]);

	assert_eq!(bindings, Some(expected));
}

#[test]
fn match_capture_fail() {
	let pattern = vec![
		PatternToken::Concrete(Token::Literal("token_1".to_owned())),
		PatternToken::Variable("p".to_owned()),
		PatternToken::Concrete(Token::Literal("token_2".to_owned())),
		PatternToken::Variable("p".to_owned()),
	];

	let expression = vec![
		Token::Literal("token_1".to_owned()),
		Token::Literal("variable".to_owned()),
		Token::Literal("token_2".to_owned()),
		Token::Literal("not_the_same_variable".to_owned()),
	];

	let bindings = get_match_bindings(&pattern, &expression);

	assert!(bindings.is_none());
}

