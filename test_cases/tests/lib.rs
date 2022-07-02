


#[test]
fn diesel() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/diesel/*.rs");
}

