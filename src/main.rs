mod tests;
mod vault;

use crate::vault::{Submission, Vault};
use std::thread;

const PUZZLE: ([i16; 9], [i16; 6]) = ([7, 7, 10, 9, 9, 7, 6, 9, 7], [12, 9, 11, 18, 6, 14]);

fn main() {
    let mut thread_id = 1;
    let mut threads = vec![];

    for v0 in Vault::ALL {
        for v1 in Vault::ALL {
            for v2 in Vault::ALL {
                let slice = [v0, v1, v2];
                thread_id += 1;
                threads.push(thread::spawn(move || Vault::run_all(slice, thread_id)));
            }
        }
    }

    let (vaults, result) = threads
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
