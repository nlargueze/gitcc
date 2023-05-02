//! Assemble

use gitcc_convco::{ConvcoMessage, BREAKING_CHANGE_KEY};
use indexmap::IndexMap;

#[test]
fn assemble_ex_1() {
    let ex_1 = include_str!("files/ex_1.txt");
    let ex_1 = ex_1.strip_suffix('\n').unwrap_or(ex_1);

    let mut footer = IndexMap::new();
    footer.insert(
        BREAKING_CHANGE_KEY.to_string(),
        "`extends` key in config file is now used for extending other config files".to_string(),
    );
    let msg = ConvcoMessage {
        r#type: "feat".to_string(),
        scope: None,
        is_breaking: false,
        desc: "allow provided config object to extend other configs".to_string(),
        body: None,
        footer: Some(footer),
    }
    .to_string();

    assert_eq!(msg, ex_1);
}

#[test]
fn assemble_ex_2() {
    let ex_2 = include_str!("files/ex_2.txt");
    let ex_2 = ex_2.strip_suffix('\n').unwrap_or(ex_2);

    let msg = ConvcoMessage {
        r#type: "feat".to_string(),
        scope: None,
        is_breaking: true,
        desc: "send an email to the customer when a product is shipped".to_string(),
        body: None,
        footer: None,
    }
    .to_string();

    assert_eq!(msg, ex_2);
}

#[test]
fn assemble_ex_6() {
    let ex_6 = include_str!("files/ex_6.txt");
    let ex_6 = ex_6.strip_suffix('\n').unwrap_or(ex_6);

    let body = "
Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.
"
    .trim();
    let mut footer = IndexMap::new();
    footer.insert("Reviewed-by".to_string(), "Z".to_string());
    footer.insert("Refs".to_string(), "#123".to_string());
    let msg = ConvcoMessage {
        r#type: "fix".to_string(),
        scope: None,
        is_breaking: false,
        desc: "prevent racing of requests".to_string(),
        body: Some(body.to_string()),
        footer: Some(footer),
    }
    .to_string();

    assert_eq!(msg, ex_6);
}
