use std::panic;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[test]
fn next_chunk_fewer() {
    let iter = [1, 2, 3].into_iter();
    let res: Result<[i32; 4], _> = arrays::next_chunk(iter);
    let mut rem = res.unwrap_err();
    assert_eq!(rem.next(), Some(1));
    assert_eq!(rem.next(), Some(2));
    assert_eq!(rem.next(), Some(3));
}

#[test]
fn next_chunk_panic() {
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let iter = [Foo, Foo, Foo].into_iter().enumerate().map(|(i, f)| {
        if i == 2 {
            panic!("😱");
        }
        f
    });

    let res = panic::catch_unwind(|| {
        let _: [Foo; 3] = arrays::next_chunk(iter).unwrap();
    });
    assert!(res.is_err());
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);
}

#[test]
fn next_chunk_unchecked_panic() {
    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug)]
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    let iter = [Foo, Foo, Foo].into_iter().enumerate().map(|(i, f)| {
        if i == 2 {
            panic!("😱");
        }
        f
    });

    let res = panic::catch_unwind(|| {
        let _: [Foo; 3] = unsafe { arrays::next_chunk_unchecked(iter) };
    });
    assert!(res.is_err());
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3);
}
