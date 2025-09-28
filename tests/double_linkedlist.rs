use alixt::AList;

#[test]
fn new_list() {
    let l: AList<u8> = AList::new();
    assert_eq!(l.len(), 0);
    assert!(l.is_empty());
}

#[test]
fn push() {
    let mut list: AList<u8> = AList::new();
    list.push(0);
    assert_eq!(list.len(), 1);
    assert_eq!(list.map_head(|x| *x), Some(0));
    list.push(1);
    assert_eq!(list.len(), 2);
    assert_eq!(list.map_head(|x| *x), Some(1));
}

#[test]
fn pop() {
    let mut list: AList<u8> = AList::new();
    (0u8..10u8).for_each(|u| list.push(u));

    assert_eq!(list.len(), 10);

    (0u8..10u8).rev().for_each(|i| {
        assert_eq!(list.len() as u8, i + 1);
        assert_eq!(list.pop(), Some(i));
    });

    assert_eq!(list.len(), 0usize);
    assert_eq!(list.pop(), None);
    assert_eq!(list.pop(), None);
    assert_eq!(list.len(), 0usize);
}

#[test]
fn push_back() {
    let mut list: AList<u8> = AList::new();

    (0u8..10u8).for_each(|u| {
        assert_eq!(list.len() as u8, u);
        list.push_back(u);
    });

    assert_eq!(list.len() as u8, 10);

    (0u8..10u8).for_each(|u| {
        assert_eq!(list.pop(), Some(u));
    });
}

#[test]
fn pop_back() {
    let mut list: AList<u8> = AList::new();

    (0u8..10u8).for_each(|u| {
        assert_eq!(list.len() as u8, u);
        list.push(u);
    });

    assert_eq!(list.len(), 10);

    (0u8..10u8).for_each(|u| {
        assert_eq!(list.len() as u8, 10 - u);
        assert_eq!(list.pop_back(), Some(u));
    });

    assert_eq!(list.len(), 0);

    (0..10).for_each(|_| {
        list.pop_back();
    });
    assert_eq!(list.len(), 0);
}

#[test]
fn drops_many_without_overflow() {
    let mut list: AList<u8> = AList::new();

    (0..1_000_000).for_each(|_| {
        list.push(0u8)
    });
    assert_eq!(list.len(), 1_000_000);
}

#[test]
fn basic_reversed() {
    let mut list: AList<u8> = AList::new();
    (0..10).for_each(|i| list.push(i));
    list.reversed();
    (0..10).for_each(|i| assert_eq!(Some(i), list.pop()));
}

#[test]
fn reversed_edge_cases() {
    let mut list: AList<u8> = AList::new();
    // empty list
    list.reversed();
    assert_eq!(list.len(), 0);
    // single item list
    list.push(101);
    list.reversed();
    assert_eq!(list.pop_back(), Some(101));
    // idk, some more fun stuff
    (0..10).for_each(|i| {
        list.push(i);
        list.reversed();
    });
    let mut switch = true;
    (0..10).rev().for_each(|i| {
        if switch {
            assert_eq!(list.pop_back(), Some(i));
            switch = false;
        } else {
            assert_eq!(list.pop(), Some(i));
            switch = true;
        }
    });
}
