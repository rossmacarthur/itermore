use itermore::IterCircularArrayWindows;

#[test]
fn circular_array_windows_infer() {
    let s = [0, 1, 0, 1];
    for [a, b] in s.iter().copied().circular_array_windows() {
        assert_eq!(a + b, 1);
    }
    for [a, b, c, d, e, f] in s.iter().copied().circular_array_windows() {
        assert_eq!(a + b + c + d + e + f, 3);
    }
}

#[test]
fn circular_array_windows_size_hint() {
    let iter = (0..6).circular_array_windows::<1>();
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.count(), 6);

    let iter = (0..6).circular_array_windows::<3>();
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.count(), 6);

    let iter = (0..6).circular_array_windows::<5>();
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.count(), 6);

    let iter = (0..6).circular_array_windows::<7>();
    assert_eq!(iter.size_hint(), (6, Some(6)));
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.count(), 6);

    let mut iter = (0..6).circular_array_windows::<4>();
    assert_eq!(iter.next(), Some([0, 1, 2, 3]));
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(iter.len(), 5);
    assert_eq!(iter.count(), 5);
}
