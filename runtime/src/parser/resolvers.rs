use std::path::{Path, PathBuf};

/// Splits a path into name and root.
///
/// TODO: I would expect there to be a better way to do this. But I can't find it. sad
pub fn get_root_and_name(path: PathBuf) -> (PathBuf, String) {
    let name = path.file_name().unwrap();
    let name = name.to_str().unwrap();
    let name = &name[..name.len() - ".pink".len()];

    let mut root = path.clone();
    root.pop();

    (root, name.to_string())
}

pub fn file_resolver(root: PathBuf) -> impl Fn(&str) -> Option<String> {
	move |name: &str| {
		let mut path = root.clone();
		path.push(name);
		path.set_extension("pink");
		
		std::fs::read_to_string(path).ok()
	}
}