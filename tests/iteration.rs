use alixt::AList;

#[test]
fn into_iterator_works() {
    let mut list: AList<String> = AList::new();
    list.push("Hello".to_string());
    list.push(",".to_string());
    list.push("world".to_string());
    list.push("!".to_string());

    let mut results: Vec<String> = Vec::new();
    list.into_iter().for_each(|s| results.push(s));
    assert_eq!(results, vec![
        "!".to_string(),
        "world".to_string(),
        ",".to_string(),
        "Hello".to_string(),
    ]);
}

#[test]
fn rev_on_into_iter() {
    let mut list: AList<usize> = AList::new();

    (0..10).for_each(|i| list.push(i));

    list
        .into_iter()
        .rev()
        .enumerate()
        .for_each(|(i, v)| assert_eq!(i, v));
}
