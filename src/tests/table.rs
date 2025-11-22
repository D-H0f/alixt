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
}
