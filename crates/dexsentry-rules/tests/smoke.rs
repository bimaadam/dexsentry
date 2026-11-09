use dexsentry_rules::{Severity, parse_rule};

#[test]
fn example_rule_model_loads() {
    let rule = parse_rule("id = \"SMOKE-001\"\ntitle = \"Smoke test\"\nseverity = \"low\"\n");
    assert!(matches!(rule, Ok(parsed) if parsed.severity == Severity::Low));
}
