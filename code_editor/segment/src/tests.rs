use crate::test_data;

#[test]
fn words() {
    use crate::StrExt;

    for test in test_data::WORD {
        let string = test.join("");
        assert_eq!(string.words().collect::<Vec<_>>(), test);
    }
}

#[test]
fn words_rev() {
    use crate::StrExt;

    for test in test_data::WORD {
        let string = test.join("");
        assert_eq!(
            string.words().rev().collect::<Vec<_>>(),
            test.iter().rev().cloned().collect::<Vec<_>>()
        );
    }
}
