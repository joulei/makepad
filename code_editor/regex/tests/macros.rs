macro_rules! test_regex {
    ($name:ident, $pattern:expr, $haystack:expr, [$($expected:expr),*]) => (
        #[test]
        fn $name() {
            use makepad_regex::Regex;

            let regex = Regex::new($pattern).unwrap();
            let haystack = $haystack;
            let expected: Vec<Option<usize>> = vec![$($expected),*];
            let mut actual = vec![None; expected.len()];
            regex.run(haystack, &mut actual);
            assert_eq!(actual, expected);
        }
    );
}

pub(crate) use test_regex;
