


#[test]
fn diesel() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/diesel/*.rs");
}

#[test]
fn chumsky() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/chumsky/*.rs");
}

#[test]
fn uom() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/uom/*.rs");
}

#[test]
fn axum() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/axum/*.rs");
}
