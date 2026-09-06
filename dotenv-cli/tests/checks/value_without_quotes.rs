use crate::common::*;

#[test]
fn correct_files() {
    let contents = [
        "A=\"B B\"\nF=\"BAR B\"\nFOO=\"BAR BAZ\"\n",
        "A=\"B B\"\r\nF=\"BAR B\"\r\nFOO=\"BAR BAZ\"\r\n",
        "# comment\nABC=\"DEF GHI\"\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];

        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn incorrect_files() {
    let contents = [
        "A=\"B B\"\nF=\"BAR B\"\nFOO=BAR BAZ\n",
        "A=\"B B\"\r\nF=BAR B\r\nFOO=\"BAR BAZ\"\r\n",
        "# comment\nABC=DEF GHI\n",
    ];
    let expected_line_numbers = [3, 2, 2];

    for (i, content) in contents.iter().enumerate() {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[format!(
                ".env:{} ValueWithoutQuotes: This value needs to be surrounded in quotes",
                expected_line_numbers[i]
            )
            .as_str()],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn multiline_value() {
    let content = "FOO=\"new\\nline value\"\n";

    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["check", testfile.as_str()];
    let expected_output = check_output(&[(".env", &[])]);

    testdir.test_command_success_with_args(with_default_args(args), expected_output);
}

#[test]
fn inline_comments() {
    let contents = [
        "A=plain # comment\nB=\"two words\" # comment\nC=€ # comment\n",
        "A=plain # comment\r\nB=\"two words\" # comment\r\nC=€ # comment\r\n",
        "A=can't # comment\nB=\"has \\\" # inside\" # outside\n",
    ];

    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(".env", &[])]);

        testdir.test_command_success_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn inline_comments_do_not_disable_checks() {
    let content = "# dotenv-linter:off ValueWithoutQuotes\n\
                   A=one two # comment\n\
                   # dotenv-linter:on ValueWithoutQuotes\n\
                   B=one two # dotenv-linter:off ValueWithoutQuotes\n\
                   C=three four # comment\n";
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["check", testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[
            ".env:4 ValueWithoutQuotes: This value needs to be surrounded in quotes",
            ".env:5 ValueWithoutQuotes: This value needs to be surrounded in quotes",
        ],
    )]);

    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}

#[test]
fn inline_comments_do_not_hide_following_assignments() {
    let contents = [
        "A=\"ok\" # first comment\nB=two words # \"quoted note\" here\n",
        "A='ok' # first comment\nB=two words # 'quoted note' here\n",
        "A=\"quoted words\" # comment\nB=two words\nC=\"other words\"\n",
    ];
    for content in contents {
        let testdir = TestDir::new();
        let testfile = testdir.create_testfile(".env", content);
        let args = &["check", testfile.as_str()];
        let expected_output = check_output(&[(
            ".env",
            &[".env:2 ValueWithoutQuotes: This value needs to be surrounded in quotes"],
        )]);

        testdir.test_command_fail_with_args(with_default_args(args), expected_output);
    }
}

#[test]
fn multiline_value_with_inline_comment() {
    let content = "A=\"first\n# still inside the value\nlast\" # outside\nB=two words\n";
    let testdir = TestDir::new();
    let testfile = testdir.create_testfile(".env", content);
    let args = &["check", testfile.as_str()];
    let expected_output = check_output(&[(
        ".env",
        &[".env:4 ValueWithoutQuotes: This value needs to be surrounded in quotes"],
    )]);

    testdir.test_command_fail_with_args(with_default_args(args), expected_output);
}
