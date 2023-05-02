//! Tests for parsing

use gitcc_convco::ConvcoMessage;

#[test]
fn parse_ex_1() {
    let ex = include_str!("files/ex_1.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "feat");
    assert!(msg.scope.is_none());
    assert!(!msg.is_breaking);
    assert_eq!(
        msg.desc,
        "allow provided config object to extend other configs"
    );
    assert!(msg.footer.unwrap().contains_key("BREAKING CHANGE"));
}

#[test]
fn parse_ex_2() {
    let ex = include_str!("files/ex_2.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "feat");
    assert!(msg.scope.is_none());
    assert!(msg.is_breaking);
    assert_eq!(
        msg.desc,
        "send an email to the customer when a product is shipped"
    );
    assert!(msg.body.is_none());
    assert!(msg.footer.is_none());
}

#[test]
fn parse_ex_3() {
    let ex = include_str!("files/ex_3.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "feat");
    assert_eq!(msg.scope.unwrap(), "api");
    assert!(msg.is_breaking);
    assert_eq!(
        msg.desc,
        "send an email to the customer when a product is shipped"
    );
    assert!(msg.body.is_none());
    assert!(msg.footer.is_none());
}

#[test]
fn parse_ex_4() {
    let ex = include_str!("files/ex_4.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "chore");
    assert!(msg.scope.is_none());
    assert!(msg.is_breaking);
    assert_eq!(msg.desc, "drop support for Node 6");
    assert!(msg.body.is_none());
    assert!(msg.footer.unwrap().contains_key("BREAKING CHANGE"));
}

#[test]
fn parse_ex_5() {
    let ex = include_str!("files/ex_5.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "docs");
    assert!(msg.scope.is_none());
    assert!(!msg.is_breaking);
    assert_eq!(msg.desc, "correct spelling of CHANGELOG");
    assert!(msg.body.is_none());
    assert!(msg.footer.is_none());
}

#[test]
fn parse_ex_6() {
    let ex = include_str!("files/ex_6.txt");
    let msg = ex.parse::<ConvcoMessage>().unwrap();
    assert_eq!(msg.r#type, "fix");
    assert!(msg.scope.is_none());
    assert!(!msg.is_breaking);
    assert_eq!(msg.desc, "prevent racing of requests");
    assert_eq!(
        msg.body.unwrap(),
        "
Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.
"
        .trim()
    );
    assert!(msg.footer.clone().unwrap().contains_key("Reviewed-by"));
    assert!(msg.footer.unwrap().contains_key("Refs"));
}
