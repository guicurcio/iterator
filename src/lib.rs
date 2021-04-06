use std::vec;

fn main() {
    let mut iter = vec!["a", "b", "c"].into_iter();
    while let Some(e) = iter.next() {}
}

fn second_iter() {
    for x in vec![1, 2, 3, 4, 5] {
        // ...
    }
}

// IntoIterator

// two takes of a vector:

fn something() {
    let vs = vec![1, 2, 3];
    // for v in vs {
    //     // consumes vs, we get owned vs.
    // }
    for v in vs.iter() {
        // This borrows vs and we get references into `v`. So it does not work with the preceeding call to naive owned vs.
    }
    for v in &vs {
        // This borrows vs and we get references into `v`. So it does not work with the preceeding call to naive owned vs.
        // equivalent to above; being explicit.
    }
}

pub fn flatten<I>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
    // We may not have an inner iterator at the start.
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Self {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
// where the O implements the trait Iterator.
where
    O: Iterator,
    // The items of the outer type to implement an IntoIterator. So we can iterate over *those* items.
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner_iter) = self.inner {
            if let Some(i) = inner_iter.next() {
                return Some(i);
            }
            self.inner = None;
        }

        let inner_item = self.outer.next()?;
        let mut inner_iter = inner_item.into_iter();
        inner_iter.next()
        // we are dropping the iterator :point_up:
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
    }
    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        )
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1)
    }
    #[test]
    // fails if we expect two.
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![0, 1])).count(), 1)
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec![1], vec![2]].into_iter()).count(), 2)
    }
}
