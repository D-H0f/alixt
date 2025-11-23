#![allow(unused)]
use crate::table::{Table, TableResult};
use colored::Colorize;

#[test]
fn test_create_table() {
    let table = Table::<3>::new()
        .title("Hi!".white())
        .headers(["one".white(), "header".white(), "please".white()])
        .row(["one".white(), "row".white(), "please".white()]);
    assert!(matches!(table, TableResult::Table(_)));
    let table = table.collect();
    assert!(table.is_ok());
    let table = table.unwrap();
    assert_eq!(table.get_column_count(), 3);
    assert_eq!(table.get_row_count(), 1);
    assert_eq!(
        table.get_headers(),
        &["one".white(), "header".white(), "please".white()]
    );
    assert_eq!(table.get_title().as_ref(), Some(&"Hi!".white()).as_ref());
}

#[test]
fn test_create_table_in_loop() {
    let mut table = Table::<3>::new()
        .title("Hi!".white())
        .headers(["one".white(), "header".white(), "please".white()])
        .collect()
        .unwrap();

    for i in (1..=5) {
        table.push_row([
            format!("row{i} col1").white(),
            format!("row{i} col2").white(),
            format!("row{i} col3").white(),
        ]);
    }

    assert_eq!(table.get_row_count(), 5);
    let expect_row = [
            "row1 col1".white(),
            "row1 col2".white(),
            "row1 col3".white(),
        ];

    assert_eq!(table.get_row(0), Some(&expect_row[..]));
}

#[test]
fn test_get_rows() {
    let mut table = Table::<3>::new()
        .title("Hi!".white())
        .headers(["one".white(), "header".white(), "please".white()])
        .collect()
        .unwrap();

    for i in (1..=5) {
        table.push_row([
            format!("row{i} col1").white(),
            format!("row{i} col2").white(),
            format!("row{i} col3").white(),
        ]);
    }

    assert_eq!(table.get_row_count(), 5);
    let expect_row_first = [
            "row1 col1".white(),
            "row1 col2".white(),
            "row1 col3".white(),
        ];
    let expect_row_last = [
            "row5 col1".white(),
            "row5 col2".white(),
            "row5 col3".white(),
        ];

    assert_eq!(table.get_row(0), Some(&expect_row_first[..]));
    assert_eq!(table.get_row(4), Some(&expect_row_last[..]));
}
