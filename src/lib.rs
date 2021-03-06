//! Implements a way to iterate over the [`Powerset`](Powerset) of some type.
//! Each type needs to have implemented `Index<usize>` and the trait [`SizableContainer`](SizableContainer), which should
//! in essence return the length of the container.
use std::ops::Index;

/// This trait needs to be implemented for the thing you want to have your powerset over.
/// In the example of a vec, it only needs to return the len of the vec.
/// In general, it has to return the greatest possible value to be indexed by plus one
pub trait SizableContainer {
    fn num_elements(&self) -> usize;
}

/// This is the powerset trait. It is implemented for everything that implements Index and
/// SizableContainer.
///
/// Example usage:
/// ```
///     use crate::powerset::{SizableContainer, Powerset};
///     let items = vec![1, 2, 3, 4];
///
///     for subset in items.powerset() {
///         println!("Got a new subset");
///         for item in subset {
///             println!("Got the item {}", item);
///         }
///     }
///
/// ```
pub trait Powerset<'a, I: Index<usize> + SizableContainer>
where
    I::Output: Sized,
{
    fn powerset(&'a self) -> SubsetIterator<'a, I>;
}

/// The iterator returned from the [`Powerset`](Powerset) trait
pub struct SubsetIterator<'a, I: Index<usize>>
where
    I::Output: Sized,
{
    items: &'a I,
    subset: u64,
}

impl<'a, I: Index<usize> + SizableContainer> Iterator for SubsetIterator<'a, I>
where
    I::Output: Sized,
{
    type Item = Subset<'a, I>;
    fn next(&mut self) -> Option<Self::Item> {
        // Check before increment, to avoid unbounded incrementation
        if self.subset >= 1 << self.items.num_elements() {
            return None;
        }

        self.subset += 1;

        Some(Subset {
            items: self.items,
            subset: self.subset - 1,
            next: 0,
        })
    }
}

/// The subset that is the element of the [`SubsetIterator`](SubsetIterator)
pub struct Subset<'a, I: Index<usize>> {
    items: &'a I,
    subset: u64,
    next: usize,
}

impl<'a, I: Index<usize>> Iterator for Subset<'a, I>
where
    I::Output: Sized,
{
    type Item = &'a I::Output;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check before increment, to avoid unbounded incrementation
            if 1 << self.next > self.subset {
                return None;
            }

            self.next += 1;

            if 1 << (self.next - 1) & self.subset != 0 {
                // return the item
                return Some(&self.items[self.next - 1]);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let max_size = (self.subset >> self.next).count_ones() as usize;
        (max_size, Some(max_size))
    }
}

impl<'a, I: Index<usize> + SizableContainer> Powerset<'a, I> for I
where
    I::Output: Sized,
{
    fn powerset(&'a self) -> SubsetIterator<'a, I> {
        SubsetIterator {
            items: self,
            subset: 0,
        }
    }
}

impl<T> SizableContainer for Vec<T> {
    fn num_elements(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Powerset;

    #[test]
    fn correct_subsets() {
        let items = vec![1, 2, 3, 4];
        let mut powerset = items.powerset();

        // Test that we get all subsets of the vector, and nothing more
        assert_eq!(
            Vec::<i32>::new(),
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![2],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![3],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 3],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![2, 3],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2, 3],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![2, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![3, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 3, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![2, 3, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![1, 2, 3, 4],
            powerset.next().unwrap().cloned().collect::<Vec<i32>>()
        );
        assert!(powerset.next().is_none());
    }

    #[test]
    fn size_hint() {
        let items = vec![1, 2, 3, 4, 5, 6];

        for mut set in items.powerset() {
            let mut count = 0;
            let size_hint = set.size_hint();
            loop {
                let elem = set.next();
                if elem.is_none() {
                    break;
                }
                count += 1;
                assert_eq!(size_hint.0 - count, set.size_hint().0);
            }

            assert_eq!(count, size_hint.0);
            assert_eq!(Some(count), size_hint.1);
        }
    }
}
