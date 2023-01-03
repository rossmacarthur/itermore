use iterwindows::IterArrayWindows;
use rand::Rng;

/// Randomly call `iter` and `iter_next`.
fn test_iter<A: IntoIterator, E: IntoIterator<Item = A::Item>>(
    prob: f64,
    actual: A,
    expected: E,
) -> proptest::test_runner::TestCaseResult
where
    A::IntoIter: DoubleEndedIterator,
    A::Item: std::fmt::Debug + PartialEq,
{
    let mut rng = rand::thread_rng();
    let expected = Vec::from_iter(expected.into_iter());

    let mut actual_iter = actual.into_iter();
    let mut actual = std::collections::VecDeque::new();
    let mut count = expected.len();
    let mut rotate = 0;
    loop {
        if rng.gen_bool(prob) {
            let Some(item) = actual_iter.next() else { break };
            actual.push_back(item);
            //count -= 1;
            //proptest::prop_assert_eq!(actual_iter.size_hint(), (count, Some(count)));
        } else {
            let Some(item) = actual_iter.next_back() else { break };
            actual.push_front(item);
            rotate += 1;
            // count -= 1;
            // proptest::prop_assert_eq!(actual_iter.size_hint(), (count, Some(count)));
        }
    }

    actual.rotate_left(rotate);
    proptest::prop_assert_eq!(&actual.make_contiguous()[..], &expected[..]);

    Ok(())
}

proptest::proptest! {
    #[test]
    fn test_next(vals: Vec<u8>) {
        let actual = vals.iter().copied().array_windows::<5>().map(|w| w.to_vec());
        let expected = vals.windows(5).map(|w| w.to_vec());
        test_iter(1.0, actual, expected)?;
    }

    #[test]
    fn test_next_back(vals: Vec<u8>) {
        let actual = vals.iter().copied().array_windows::<5>().map(|w| w.to_vec());
        let expected = vals.windows(5).map(|w| w.to_vec());
        test_iter(0.0, actual, expected)?;
    }

    #[test]
    fn test_both_directions(vals: Vec<u8>) {
        let actual = vals.iter().copied().array_windows::<5>().map(|w| w.to_vec());
        let expected = vals.windows(5).map(|w| w.to_vec());
        test_iter(0.5, actual, expected)?;
    }
}
