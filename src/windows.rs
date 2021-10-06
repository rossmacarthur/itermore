use crate::array;

/// An iterator that yields overlapping chunks of `N` elements of `T` at a time.
///
/// This struct is created by the [`windows`][crate::IterMore::windows] method
/// on iterators.
#[derive(Debug, Clone)]
pub struct Windows<I, T, const N: usize>
where
    I: Iterator<Item = T>,
{
    iter: I,
    last: Option<[T; N]>,
}

impl<I, T, const N: usize> Windows<I, T, N>
where
    I: Iterator<Item = T>,
{
    #[inline]
    pub fn new(iter: I) -> Self {
        Self { iter, last: None }
    }
}

impl<I, T, const N: usize> Iterator for Windows<I, T, N>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, last } = self;

        match last {
            Some(last) => {
                let item = iter.next()?;
                last.rotate_left(1);
                if let Some(end) = last.last_mut() {
                    *end = item;
                }
                Some(last.clone())
            }
            None => {
                let tmp = array::collect(iter)?;
                *last = Some(tmp.clone());
                Some(tmp)
            }
        }
    }
}
