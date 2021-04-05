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
}
