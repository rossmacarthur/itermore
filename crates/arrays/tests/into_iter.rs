#[test]
fn into_iter_as_slice() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.as_slice(), &[1, 2, 3]);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.as_slice(), &[2, 3]);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.as_slice(), &[3]);
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.as_slice(), &[]);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.as_slice(), &[]);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.as_slice(), &[]);
}

#[test]
fn into_iter_as_mut_slice() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.next(), Some(1));
    {
        let s = iter.as_mut_slice();
        s[0] = 4;
    }
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_debug() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(format!("{:?}", iter), "IntoIter([2, 3])");
}

#[test]
fn into_iter_clone() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.next(), Some(1));
    let mut iter2 = iter.clone();
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter2.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter2.next(), Some(3));
    assert_eq!(iter.next(), None);
    assert_eq!(iter2.next(), None);
}

#[test]
fn into_iter_size_hint() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
}

#[test]
fn into_iter_count() {
    let arr: [i32; 0] = [];
    let iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.count(), 0);

    let arr: [i32; 1] = [1];
    let iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.count(), 1);

    let arr: [i32; 2] = [1, 2];
    let iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.count(), 2);

    let arr: [i32; 3] = [1, 2, 3];
    let iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.count(), 3);
}

#[test]
fn into_iter_len() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut iter = arrays::IntoIter::new(arr);
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
}
