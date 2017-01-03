
use num::{Num, Zero, ToPrimitive};

use std::cmp::Ord;
use std::hash::Hash;
use std::ops::{Add, Sub, Div};

use super::Block;
use algos::computation::NanMinMax;
use algos::grouper::Grouper;
use groupby::GroupBy;
use traits::{Apply, BasicAggregation, NumericAggregation, ComparisonAggregation};

/// /////////////////////////////////////////////////////////////////////////////
/// Apply
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C, G, W> Apply<'c, Vec<W>> for GroupBy<'i, Block<'v, 'i, 'c, V, I, C>, G>
    where V: Clone,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash,
          G: 'c + Clone + Eq + Hash + Ord,
          W: 'c + Clone
{
    type In = Block<'v, 'i, 'c, V, I, C>;
    type FOut = Vec<W>;
    type Out = Block<'c, 'c, 'c, W, G, C>;

    /// Apply passed function to each group
    fn apply<'f>(&'c self, func: &'f Fn(&Self::In) -> Self::FOut) -> Self::Out {

        let mut new_values: Vec<W> = Vec::with_capacity(self.grouper.len());

        let groups = self.groups();
        for g in groups.iter() {
            let s = self.get_group(&g);
            new_values.append(&mut func(&s));
        }
        let new_columns = self.data.columns.clone();
        Block::from_row_vec(new_values, groups, new_columns.into_owned())
    }
}

/// /////////////////////////////////////////////////////////////////////////////
/// Aggregation
/// /////////////////////////////////////////////////////////////////////////////

impl<'v, 'i, 'c, V, I, C, G> BasicAggregation<'c> for GroupBy<'i, Block<'v, 'i, 'c, V, I, C>, G>
    where V: 'c + Clone + Zero + Add,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash,
          G: 'c + Clone + Eq + Hash + Ord
{
    type Kept = Block<'c, 'c, 'c, V, G, C>;
    type Counted = Block<'c, 'c, 'c, usize, G, C>;

    fn sum(&'c self) -> Self::Kept {
        self.apply(&|x: &Block<V, I, C>| x.sum().values.into_owned())
    }

    fn count(&'c self) -> Self::Counted {
        self.apply(&|x: &Block<V, I, C>| x.count().values.into_owned())
    }
}

impl<'v, 'i, 'c, V, I, C, G> NumericAggregation<'c> for GroupBy<'i, Block<'v, 'i, 'c, V, I, C>, G>
    where V: 'c + Clone + Zero + Add + Sub + Div + ToPrimitive,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash,
          G: 'c + Clone + Eq + Hash + Ord
{
    type Coerced = Block<'c, 'c, 'c, f64, G, C>;

    fn mean(&'c self) -> Self::Coerced {
        self.apply(&|x: &Block<V, I, C>| x.mean().values.into_owned())
    }

    fn var(&'c self) -> Self::Coerced {
        self.apply(&|x: &Block<V, I, C>| x.var().values.into_owned())
    }

    fn unbiased_var(&'c self) -> Self::Coerced {
        self.apply(&|x: &Block<V, I, C>| x.unbiased_var().values.into_owned())
    }

    fn std(&'c self) -> Self::Coerced {
        self.apply(&|x: &Block<V, I, C>| x.std().values.into_owned())
    }

    fn unbiased_std(&'c self) -> Self::Coerced {
        self.apply(&|x: &Block<V, I, C>| x.unbiased_std().values.into_owned())
    }
}


impl<'v, 'i, 'c, V, I, C, G> ComparisonAggregation<'c>
    for GroupBy<'i, Block<'v, 'i, 'c, V, I, C>, G>
    where V: 'c + Clone + NanMinMax<V>,
          I: Clone + Eq + Hash,
          C: Clone + Eq + Hash,
          G: 'c + Clone + Eq + Hash + Ord
{
    type Kept = Block<'c, 'c, 'c, V, G, C>;

    fn min(&'c self) -> Self::Kept {
        self.apply(&|x: &Block<V, I, C>| x.min().values.into_owned())
    }

    fn max(&'c self) -> Self::Kept {
        self.apply(&|x: &Block<V, I, C>| x.max().values.into_owned())
    }
}
