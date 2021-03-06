use std::hash::Hash;

use super::Series;
use nullvec::prelude::dev::algos::Indexing;
use nullvec::prelude::dev::algos::Sorter;
use indexer::Indexer;
use traits::Slicer;

//**********************************************
//*Soat
//**********************************************

impl<'v, 'i, V, I> Series<'v, 'i, V, I>
    where V: Clone,
          I: Clone + Eq + Hash + Ord
{
    pub fn sort_index(&self) -> Self {
        let (indexer, sorted) = self.index.argsort();
        let new_values = Indexing::reindex(&self.values, &indexer);
        Series::new(new_values, sorted)
    }
}

impl<'v, 'i, V, I> Series<'v, 'i, V, I>
    where V: Clone + Ord,
          I: Clone + Eq + Hash
{
    pub fn sort_values(&self) -> Self {
        let (indexer, sorted) = Sorter::argsort(&self.values);
        let index: Indexer<I> = self.index.reindex(&indexer);
        Series::new(sorted, index)
    }
}

#[cfg(test)]
mod tests {

    use super::super::Series;

    #[test]
    fn test_sort_index_int() {
        let s = Series::new(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]);
        let sorted = s.sort_index();

        let exp = Series::new(vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]);
        assert_eq!(sorted, exp);
    }

    #[test]
    fn test_sort_index_str() {
        let s = Series::new(vec![1, 2, 3, 4], vec!["d", "b", "a", "c"]);
        let sorted = s.sort_index();
        let exp = Series::new(vec![3, 2, 4, 1], vec!["a", "b", "c", "d"]);
        assert_eq!(sorted, exp);
    }
}
