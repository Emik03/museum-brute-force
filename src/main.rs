mod tests;
mod vault;

use crate::vault::{Submission, Vault};
use konst::iter::collect_const;
use konst::primitive::parse_i16;
use konst::{string, unwrap_ctx};

const PUZZLE: ([i16; collect_const!(&str => string::split(string::split(include_str!("../puzzle.txt"), "\n").next().expect("File must have a newline.").0, ",")).len()],
    [i16; collect_const!(&str => string::split(string::split(include_str!("../puzzle.txt"), "\n").rev().next().expect("File must have a newline.").0, ",")).len()]) = (
    collect_const!(i16 =>
        string::split(string::split(include_str!("../puzzle.txt"), "\n").next().expect("File must have a newline.").0, ","),
        map(string::trim),
        map(|s| unwrap_ctx!(parse_i16(s)))
    ),
    collect_const!(i16 =>
        string::split(string::split(include_str!("../puzzle.txt"), "\n").rev().next().expect("File must have a newline.").0, ","),
        map(string::trim),
        map(|s| unwrap_ctx!(parse_i16(s)))
    ),
);

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
