pub trait StutteringIterator: Iterator {
    /// Creates an iterator that duplicates each item count times.
    #[inline]
    fn stutter(self, count: usize) -> StutteringIter<Self>
    where
        Self: Sized,
        Self::Item: Copy,
    {
        StutteringIter::new(self, count)
    }
}

impl<T: ?Sized> StutteringIterator for T where T: Iterator {}

pub struct StutteringIter<I: Iterator> {
    iter: I,
    curr: Option<I::Item>,
    nr: usize,
    count: usize,
}

impl<I> StutteringIter<I>
where
    I: Iterator,
{
    pub fn new(iter: I, count: usize) -> Self {
        Self {
            iter,
            curr: None,
            nr: count,  // start at count, so we immediately set curr
            count,
        }
    }
}

impl<I> Iterator for StutteringIter<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.nr == self.count {
            self.curr = self.iter.next();
            self.nr = 0;
        } else {
            self.nr += 1;
        }

        self.curr
    }
}

#[cfg(test)]
mod tests {
    use super::StutteringIterator;

    #[test]
    fn test_0() {
        let a = vec![0, 1, 2];
        let mut iter = a.iter().stutter(0);

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
    }

    #[test]
    fn test_1() {
        let a = vec![0, 1, 2];
        let mut iter = a.iter().stutter(1);

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_2() {
        let a = vec![0, 1, 2];
        let mut iter = a.iter().stutter(2);

        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}
