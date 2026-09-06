use crate::common::*;

#[test]
fn value_without_quotes() {
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", "ABC=DEF GHI\nFOO=BAR BAZ\n");
    let expected_output = fix_output(&[(
        ".env",
        &[
            ".env:1 ValueWithoutQuotes: This value needs to be surrounded in quotes",
            ".env:2 ValueWithoutQuotes: This value needs to be surrounded in quotes",
        ],
    )]);
    testdir.test_command_fix_success(expected_output);

    assert_eq!(
        testfile.contents().as_str(),
        "ABC=\"DEF GHI\"\nFOO=\"BAR BAZ\"\n"
    );

    testdir.close();
}

#[test]
fn valid_values_with_inline_comments_are_unchanged() {
    let content = "A=plain # comment\nB=\"two words\" # comment\nC=€ # comment\n";
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);

    testdir.test_command_fix_success(fix_output(&[(".env", &[])]));

    assert_eq!(content, testfile.contents());
}
