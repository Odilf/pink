use super::*;

mod std_resovler_test {
    use super::*;

    #[test]
    fn valid() {
        std_resolver::StdResolver::default()
            .resolve("std/peano")
            .unwrap();
    }

    #[test]
    fn incorrect_prefix() {
        std_resolver::StdResolver::default()
            .resolve("foo")
            .unwrap_err();
    }

    #[test]
    fn not_and_std_module() {
        std_resolver::StdResolver::default()
            .resolve("std/yahooo")
            .unwrap_err();
    }
}
