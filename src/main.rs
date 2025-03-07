mod tests;
mod vault;

use crate::vault::{Submission, Vault};

const PUZZLE: ([i16; 9], [i16; 6]) = ([5, 4, 6, 5, 3, 6, 4, 5, 5], [8, 6, 4, 5, 3, 9]);

fn main() {
    let (vaults, result) = Vault::run_threads()
        .into_iter()
        .map(|x| x.join().expect("Thread panics should propagate"))
        .max_by(|(_, x), (_, y)| {
            x.iter()
                .sum::<Submission>()
                .cmp(&y.iter().sum::<Submission>())
                .then_with(|| {
                    x.iter()
                        .product::<Submission>()
                        .cmp(&y.iter().product::<Submission>())
                })
        })
        .expect("Thread collection should be populated");

    println!("Best result: {result:?} from {vaults:?}");
}
