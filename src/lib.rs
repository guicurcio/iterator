use std::vec;

fn main() {
    let mut iter = vec!["a", "b", "c"].into_iter();
    while let Some(e) = iter.next() {}
}

pub fn second_iter() {
    for x in vec![1, 2, 3, 4, 5] {
        // ...
    }
}

// IntoIterator

// two takes of a vector:

pub fn something() {
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

pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_itter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_itter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Self {
            outer: iter,
            front_itter: None,
            back_itter: None,
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
        loop {
            if let Some(ref mut front_itter) = self.front_itter {
                if let Some(i) = front_itter.next() {
                    return Some(i);
                }
                self.front_itter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_itter = Some(next_inner.into_iter());
            } else {
                // if it stops iterating.
                return self.back_itter.as_mut()?.next();
            };
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_itter) = self.back_itter {
                if let Some(i) = back_itter.next_back() {
                    return Some(i);
                }
                self.back_itter = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_itter = Some(next_back_inner.into_iter());
            } else {
                // if it stops iterating.
                return self.front_itter.as_mut()?.next_back();
            };
        }
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
        assert_eq!(flatten(std::iter::once(vec![0, 1])).count(), 2)
    }
    #[test]
    fn two_wide() {
        // A call to flatten should return an iterator.
        assert_eq!(flatten(vec![vec![1], vec![2]]).count(), 2)
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn reverse_wide() {
        // A call to flatten should return an iterator.
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        )
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }
}
