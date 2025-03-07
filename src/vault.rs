use crate::PUZZLE;
use std::fmt::{Display, Formatter};
use std::ops::{AddAssign, SubAssign};
use std::thread;
use std::thread::JoinHandle;

pub type Submission = i16;

pub type Submissions = [Submission; PUZZLE.0.len() - PUZZLE.1.len()];

pub type Vaults = [(Vault, Submission); PUZZLE.0.len()];

#[derive(Copy, Clone, Default, Debug)]
pub enum Vault {
    #[default]
    Bronze,
    Silver,
    Gold,
}

trait ArrayLengthProvider<const T: usize> {
    const LEN: usize = T;
}

impl<T, const N: usize> ArrayLengthProvider<N> for [T; N] {}

impl Display for Vault {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Vault {
    pub(crate) const ALL: [Self; 3] = [Self::Bronze, Self::Silver, Self::Gold];

    pub(crate) const fn factor(self) -> f32 {
        match self {
            Self::Bronze => 1.4,
            Self::Silver => 1.7,
            Self::Gold => 2.0,
        }
    }

    pub(crate) fn run_threads() -> Vec<JoinHandle<(Vaults, Submissions)>> {
        let mut thread_id = 1;
        let mut threads = vec![];

        for v0 in Self::ALL {
            if Self::should_prune([v0]) {
                continue;
            }
            for v1 in Self::ALL {
                if Self::should_prune([v0, v1]) {
                    continue;
                }
                for v2 in Self::ALL {
                    if Self::should_prune([v0, v1, v2]) {
                        continue;
                    }
                    let slice = [v0, v1, v2];
                    thread_id += 1;
                    threads.push(thread::spawn(move || Self::run_all(slice, thread_id)));
                }
            }
        }

        threads
    }

    pub(crate) fn run_all(
        initial_vaults: [Self; 3],
        thread_id: Submission,
    ) -> (Vaults, Submissions) {
        let mut max_sum = Submission::MIN;
        let mut max_product = Submission::MIN;
        let mut best_vaults: Vaults = Default::default();
        let mut best_submissions: Submissions = Default::default();

        Self::do_loop(initial_vaults, |vaults| {
            if let Some(submissions) = Self::run(vaults) {
                let sum = submissions.iter().sum();
                let product = submissions.iter().product();

                if sum > max_sum || (sum == max_sum && product > max_product) {
                    max_sum = sum;
                    max_product = product;
                    best_vaults = vaults;
                    best_submissions = submissions;

                    if option_env!("VERBOSE").is_some() {
                        eprintln!("Thread {thread_id}: Found {submissions:?} from {vaults:?}");
                    }
                }
            }
        });

        eprintln!("Thread {thread_id} has finished computing.");
        (best_vaults, best_submissions)
    }

    pub(crate) fn run(mut vaults: Vaults) -> Option<Submissions> {
        let mut bronze: (&mut Submission, &mut Submission) = (&mut 0, &mut 0);
        let mut silver: (&mut Submission, &mut Submission) = (&mut 0, &mut 0);
        let mut gold: (&mut Submission, &mut Submission) = (&mut 0, &mut 0);

        for (vault, amount) in vaults {
            vault.add_to_vault(amount, &mut bronze, &mut silver, &mut gold);
        }

        for (vault, amount) in &mut vaults {
            vault.payout(amount, &bronze, &silver, &gold);
        }

        (0..PUZZLE.1.len())
            .all(|x| PUZZLE.0[x] - vaults[x].1 == PUZZLE.1[x])
            .then(|| {
                let mut best: Submissions = Default::default();

                for i in 0..Submissions::LEN {
                    const OFFSET: usize = Vaults::LEN - Submissions::LEN;
                    best[i] = PUZZLE.0[i + OFFSET] - vaults[i + OFFSET].1;
                }

                best
            })
    }

    pub(crate) fn add_amount_to_vault(
        self,
        amount: Submission,
        bronze_amount: &mut Submission,
        silver_amount: &mut Submission,
        gold_amount: &mut Submission,
    ) {
        match self {
            Self::Bronze => bronze_amount,
            Self::Silver => silver_amount,
            Self::Gold => gold_amount,
        }
        .add_assign(amount);
    }

    pub(crate) fn should_prune<const T: usize>(vaults: [Self; T]) -> bool {
        if option_env!("NO_PRUNING").is_some() {
            return false;
        }

        let mut max_bronze = Submission::default();
        let mut max_silver = Submission::default();
        let mut max_gold = Submission::default();
        let mut bronze = Submission::default();
        let mut silver = Submission::default();
        let mut gold = Submission::default();

        vaults.iter().zip(PUZZLE.0).for_each(|(v, s)| {
            v.add_amount_to_vault(s, &mut max_bronze, &mut max_silver, &mut max_gold);
        });

        PUZZLE.0[vaults.len()..].iter().for_each(|v| {
            max_bronze += v;
            max_silver += v;
            max_gold += v;
        });

        vaults.iter().zip(PUZZLE.1).for_each(|(v, s)| {
            v.add_amount_to_vault(s, &mut bronze, &mut silver, &mut gold);
        });

        f32::from(max_bronze) * Self::Bronze.factor() < f32::from(bronze)
            || f32::from(max_silver) * Self::Silver.factor() < f32::from(silver)
            || f32::from(max_gold) * Self::Gold.factor() < f32::from(gold)
    }

    fn add_to_vault<'a>(
        self,
        amount: Submission,
        (bronze_amount, bronze_count): &mut (&'a mut Submission, &'a mut Submission),
        (silver_amount, silver_count): &mut (&'a mut Submission, &'a mut Submission),
        (gold_amount, gold_count): &mut (&'a mut Submission, &'a mut Submission),
    ) {
        let (current_amount, current_count) = match self {
            Self::Bronze => (bronze_amount, bronze_count),
            Self::Silver => (silver_amount, silver_count),
            Self::Gold => (gold_amount, gold_count),
        };

        current_amount.add_assign(amount);
        current_count.add_assign(1);
    }

    fn payout(
        self,
        amount: &mut Submission,
        (bronze_amount, bronze_count): &(&mut Submission, &mut Submission),
        (silver_amount, silver_count): &(&mut Submission, &mut Submission),
        (gold_amount, gold_count): &(&mut Submission, &mut Submission),
    ) {
        #[allow(clippy::cast_possible_truncation)]
        amount.sub_assign(match self {
            Self::Bronze => {
                f32::from(**bronze_amount) * Self::Bronze.factor() / f32::from(**bronze_count)
            }
            Self::Silver => {
                f32::from(**silver_amount) * Self::Silver.factor() / f32::from(**silver_count)
            }
            Self::Gold => f32::from(**gold_amount) * Self::Gold.factor() / f32::from(**gold_count),
        } as Submission);
    }

    fn do_loop<T>([v0, v1, v2]: [Self; 3], mut fun: T)
    where
        T: FnMut(Vaults),
    {
        for v3 in Self::ALL {
            if Self::should_prune([v0, v1, v2, v3]) {
                continue;
            }
            for v4 in Self::ALL {
                if Self::should_prune([v0, v1, v2, v3, v4]) {
                    continue;
                }
                for v5 in Self::ALL {
                    if Self::should_prune([v0, v1, v2, v3, v4, v5]) {
                        continue;
                    }
                    for v6 in Self::ALL {
                        if Self::should_prune([v0, v1, v2, v3, v4, v5, v6]) {
                            continue;
                        }
                        for v7 in Self::ALL {
                            if Self::should_prune([v0, v1, v2, v3, v4, v5, v6, v7]) {
                                continue;
                            }

                            for v8 in Self::ALL {
                                if Self::should_prune([v0, v1, v2, v3, v4, v5, v6, v7, v8]) {
                                    continue;
                                }

                                for s0 in 0..=PUZZLE.0[0] {
                                    for s1 in 0..=PUZZLE.0[1] {
                                        for s2 in 0..=PUZZLE.0[2] {
                                            for s3 in 0..=PUZZLE.0[3] {
                                                for s4 in 0..=PUZZLE.0[4] {
                                                    for s5 in 0..=PUZZLE.0[5] {
                                                        for s6 in 0..=PUZZLE.0[6] {
                                                            for s7 in 0..=PUZZLE.0[7] {
                                                                for s8 in 0..=PUZZLE.0[8] {
                                                                    fun([
                                                                        (v0, s0),
                                                                        (v1, s1),
                                                                        (v2, s2),
                                                                        (v3, s3),
                                                                        (v4, s4),
                                                                        (v5, s5),
                                                                        (v6, s6),
                                                                        (v7, s7),
                                                                        (v8, s8),
                                                                    ]);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
