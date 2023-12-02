//! Implements logic that is common to both the combinations and array
//! combinations adaptors.

use core::fmt;
use core::iter::Fuse;

#[derive(Clone)]
pub struct GenericCombinations<I, C>
where
    I: Iterator,
{
    /// The underlying iterator.
    iter: Fuse<I>,

    /// A number representing the combination.
    ///
    /// If we consider the iterator as a number of K digits in base N where N is
    /// the length of the iterator (unknown at this point) then each digit
    /// represents a position in the iterator. Incrementing this number by one
    /// will find the next combination with replacement, to find the next
    /// without replacement we just need to find the next case where all digits
    /// are in increasing order.
    comb: C,

    /// A buffer containing already yielded elements that are needed for later
    /// combinations.
    buf: Vec<I::Item>,

    /// The state of the iterator.
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum State {
    First,
    Normal,
    #[cfg(any(feature = "array_combinations_with_reps", feature = "combinations"))]
    Done,
}

impl<I, C> GenericCombinations<I, C>
where
    I: Iterator,
{
    pub fn new(iter: I, comb: C) -> Self {
        Self {
            iter: iter.fuse(),
            comb,
            buf: Vec::new(),
            state: State::First,
        }
    }

    pub fn fmt_with(&self, f: &mut fmt::Formatter<'_>, name: &str) -> fmt::Result
    where
        I: fmt::Debug,
        I::Item: fmt::Debug,
        C: fmt::Debug,
    {
        f.debug_struct(name)
            .field("iter", &self.iter)
            .field("comb", &self.comb)
            .field("buf", &self.buf)
            .field("state", &self.state)
            .finish()
    }

    #[cfg(any(feature = "array_combinations", feature = "combinations"))]
    pub fn fill_next(&mut self) -> Option<impl Iterator<Item = I::Item> + '_>
    where
        I::Item: Clone,
        C: AsRef<[usize]> + AsMut<[usize]>,
    {
        let k = self.comb.as_ref().len();

        match self.state {
            #[cfg(any(feature = "array_combinations_with_reps", feature = "combinations"))]
            State::Done => return None,

            State::First => {
                // Fill the buffer with k elements from the iterator.
                self.buf.reserve(k);
                for _ in 0..k {
                    self.buf.push(self.iter.next()?);
                }
                self.state = State::Normal;
            }

            State::Normal => {
                // If the last digit in the combination points to the last
                // element in the buffer then we need to get another element
                // from the iterator because the next combination will need this
                // element.
                let d = unsafe { self.comb.as_mut().last_mut().unwrap_unchecked() };
                if *d == self.buf.len() - 1 {
                    if let Some(item) = self.iter.next() {
                        self.buf.push(item);
                    }
                }

                // Now we find the digit that needs to be incremented. Looking
                // from the back we find the first digit that is not the final
                // expected combination for that digit.
                //
                // For example given K = 3 and a total N = 5
                //
                // 0 1 3 ^--- finds this because at this point we think N = 4
                //
                // 0 1 4 ^----- finds this because we know N = 5
                //
                // 0 2 3 ^--- finds this again since it is not 4 yet
                //
                // The base case in the above example would be the following
                // which returns `None` and is propagated using `?`.
                //
                // 2 3 4
                //
                let n = self.buf.len();
                let i = self
                    .comb
                    .as_ref()
                    .iter()
                    .enumerate()
                    .rposition(|(i, &d)| d != i + n - k)?;

                // Increment the digit, and reset the ones to its right
                //
                // For example given K = 3 and N = 5 and the following combination.
                //
                // 0 1 4 ^----- i
                //
                // We would increment digit i and then reset all digits to the
                // right.
                //
                // 0 2 3 ^---- was reset to 3 ^------ was incremented
                //
                self.comb.as_mut()[i] += 1;
                for j in (i + 1)..k {
                    self.comb.as_mut()[j] = self.comb.as_ref()[j - 1] + 1;
                }
            }
        }

        Some(self.comb.as_ref().iter().map(|&d| self.buf[d].clone()))
    }

    #[cfg(any(feature = "array_combinations_with_reps", feature = "combinations"))]
    pub fn fill_next_with_reps(&mut self) -> Option<impl Iterator<Item = I::Item> + '_>
    where
        I::Item: Clone,
        C: AsRef<[usize]> + AsMut<[usize]>,
    {
        match self.state {
            State::Done => return None,

            State::First => match self.iter.next() {
                Some(item) => {
                    self.buf.push(item);
                    self.state = State::Normal;
                }
                None => {
                    self.state = State::Done;
                }
            },

            State::Normal => {
                if let Some(item) = self.iter.next() {
                    self.buf.push(item);
                }

                let n = self.buf.len();
                for (i, d) in self.comb.as_mut().iter_mut().enumerate().rev() {
                    *d += 1;
                    if *d < n {
                        break;
                    }
                    *d = 0;
                    if i == 0 {
                        self.buf.clear();
                        self.state = State::Done;
                        return None;
                    }
                }
            }
        }

        Some(self.comb.as_ref().iter().map(|&d| self.buf[d].clone()))
    }
}
