mod tests;
mod vault;

use crate::vault::{Submission, Vault};
use std::thread;

const PUZZLE: ([i16; 9], [i16; 6]) = ([5, 4, 6, 5, 3, 6, 4, 5, 5], [8, 6, 4, 5, 3, 9]);

fn main() {
    let mut thread_id = 1;
    let mut threads = vec![];

    for v0 in Vault::ALL {
        if Vault::should_prune([v0]) {
            continue;
        }
        for v1 in Vault::ALL {
            if Vault::should_prune([v0, v1]) {
                continue;
            }
            for v2 in Vault::ALL {
                if Vault::should_prune([v0, v1, v2]) {
                    continue;
                }
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
