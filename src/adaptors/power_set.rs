use crate::adaptors::generic_combinations::GenericCombinations;

pub trait IterPowerSet: Iterator {
    /// Return an iterator that iterates through the powerset of the elements from an
    /// iterator.
    fn power_set(self) -> PowerSet<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        PowerSet::new(self)
    }
}

pub struct PowerSet<I>
where
    I: Iterator,
{
    combs: GenericCombinations<I, Vec<usize>>,
}

impl<I> PowerSet<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        Self {
            combs: GenericCombinations::new(iter, vec![]),
        }
    }
}
