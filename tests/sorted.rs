#![cfg(feature = "sorted")]

use itermore::IterSorted;

#[test]
fn sorted() {
    let v: Vec<_> = [1, 3, 2].into_iter().sorted().collect();
    assert_eq!(v, [1, 2, 3]);
}
