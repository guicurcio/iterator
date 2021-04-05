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

pub fn flatten<I>(iter: I) -> Flatten<I> {
    Flatten::new(iter)
}

pub struct Flatten<I> {
    outer: I,
}

impl<I> Flatten<I> {
    fn new(iter: I) -> Self {
        Self { outer: iter }
    }
}

impl<I> Iterator for Flatten<I> {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
