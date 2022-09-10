//! Tests git tags

#[test]
fn get_tags() {
    let _tags = gitext::git::git_get_tags().unwrap();
    println!("--- TAGS ---");
    for tag in _tags {
        println!("{}", tag);
    }
    println!("------------");
}
