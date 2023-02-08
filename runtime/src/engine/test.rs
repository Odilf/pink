use super::*;

#[test]
fn structures_with_overlap() {
	let domain = BTreeSet::from(["a".to_string(), "b".to_string()]);
	let reserved = BTreeSet::from(["a".to_string(), "c".to_string()]);

	assert_eq!(
		Structure::create(domain, reserved, Vec::new()),
		Err(StructureError::DomainAndReservedOverlap { culprit: "a".to_string() })
	);
}