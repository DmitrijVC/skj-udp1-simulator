use std::fmt::{Display, Formatter};
use num::integer::gcd;
use rand::{Rng, SeedableRng};
use rand::prelude::IteratorRandom;
use rand_pcg::Pcg64;
use crate::{NUM_MIN, NUM_MAX, TASKS_AMOUNT};

// ToDo: #repr[u8] instead of arbitrary numbers ???
#[derive(Clone, Debug)]
pub enum Task {
    GCD {
        numbers: Vec<u64>,
        result: u64,
    },
    SUM {
        numbers: [u64; 3],
        result: u64,
    },
    XK {
        x: u64,
        power: u8,
        result: u64,
    },
    StringDeletion {
        target: String,
        del: char,
        result: String,
    },
    StringConcat {
        target: String,
        result: String,
    },
}

impl Task {
    /// Generates a vector of tasks based on a given seed
    ///
    /// # Arguments
    /// * `seed` - A number used for pseudo random number generation
    ///
    pub fn gen_tasks(seed: u64) -> Vec<Self> {
        if TASKS_AMOUNT > 5 {
            panic!("There is only 5 tasks. Cannot generate more than that!");
        }

        let mut tasks_given = Vec::<u8>::new();
        let mut rng = Pcg64::seed_from_u64(seed);

        for _ in 0..TASKS_AMOUNT {
            // ToDo: replace the task assignment with something more optimal
            let mut id = rng.gen_range(0..5);
            while tasks_given.contains(&id) {
                id = rng.gen_range(0..5);
            }
            tasks_given.push(id);
        }

        let mut tasks_generated = Vec::<Self>::new();

        for task_id in tasks_given {
            match task_id {
                0 => {
                    let amount: u8 = rng.gen_range(2..6);
                    let mut numbers = Vec::new();
                    let mut result = 0;

                    for _ in 0..amount {
                        let number = rng.gen_range(NUM_MIN..NUM_MAX);
                        result = gcd(result, number);
                        numbers.push(number);
                    }

                    tasks_generated.push(Self::GCD {
                        numbers,
                        result,
                    });
                },
                1 => {
                    let num1 = rng.gen_range(NUM_MIN..NUM_MAX);
                    let num2 = rng.gen_range(NUM_MIN..NUM_MAX);
                    let num3 = rng.gen_range(NUM_MIN..NUM_MAX);
                    
                    tasks_generated.push(Self::SUM {
                        numbers: [num1, num2, num3],
                        result: num1+num2+num3,
                    });
                },
                2 => {
                    let x = rng.gen_range(NUM_MIN..NUM_MAX);
                    let power = rng.gen_range(1..6);
                    let mut result = 1u64;

                    while (result+1).pow(power as u32) <= x {
                        result+=1;
                    }

                    tasks_generated.push(Self::XK {
                        x,
                        power,
                        result,
                    });
                },
                3 => {
                    let target = format!("{}", rng.gen_range(NUM_MIN..NUM_MAX));
                    let del = target.chars().choose(&mut rng).unwrap();
                    let result = target.replace(del, "");

                    tasks_generated.push(Self::StringDeletion {
                        target,
                        del,
                        result,
                    });
                },
                4 => {
                    let target = format!("{}", rng.gen_range(NUM_MIN..NUM_MAX));
                    let result = format!("{}{}", target, target);

                    tasks_generated.push(Self::StringConcat {
                        target,
                        result,
                    });
                },
                _ => {}
            }
        }

        tasks_generated
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Task::GCD { numbers, .. } => {
                    format!("[GCD] W {0} kolejnych liniach odbierz {0} liczb naturalnych. Policz ich najwi??kszy wsp??lny dzielnik i wynik ode??lij.", numbers.len())
                }
                Task::SUM { numbers, .. } => {
                    format!("[SUM] W {0} kolejnych liniach odbierz {0} liczby naturalnych. Policz sum?? tych liczb i ode??lij.", numbers.len())
                }
                Task::XK { power, .. } => {
                    format!("[XK] Odbierz liczb?? naturaln?? x, Oblicz najwi??ksz?? liczb?? naturaln?? k, tak??, ??e k podniesione do pot??gi {} jest nie wi??ksze ni?? warto???? x. Ode??lij warto???? k.", power)
                }
                Task::StringDeletion { del, .. } => {
                    format!("[SD] Odbierz napis. Usu?? z niego wszystkie wyst??pienia {} i ode??lij wynik.", del)
                }
                Task::StringConcat { .. } => {
                    format!("[SC] Odbierz jedn?? linijk?? tekstu. Skonkatenuj tekst 2 razy z samym sob?? i ode??lij wynik.")
                }
            }
        )
    }
}
