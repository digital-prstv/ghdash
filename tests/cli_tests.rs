#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        // .case("README.md")
        .case("tests/cmd/*.trycmd");
}
