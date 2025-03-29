use minigrep;

#[test]
fn search_test() {
    let query = String::from("duct");
    let contents = "\n\
Rust:\n\
safe, fast, productive.\n\
Pick three.";
    let results = minigrep::search(query, &contents);
    assert_eq!(results, vec!["safe, fast, productive."])
}

#[test]
fn search_ignore_case_test() {
    let query = String::from("you");
    let contents = "\n\
Rust:\n\
you know\n\
Nice to meet You!\n\
safe, fast, productive.\n\
Pick three.";
    let results = minigrep::search_case_insensitive(query, &contents);
    assert_eq!(results, vec!["you know", "Nice to meet You!"])
}
