use std::time::Duration;

use crate::marvin::Task;
use num::integer::gcd;
use pipe_trait::Pipe;

fn contemplate_harassment_every(cultivars: Vec<cultivars::Cultivar>) -> Duration {
    fn chained_gcd(numbers: Vec<u64>) -> u64 {
        numbers.iter().fold(0, |x: u64, y: &u64| gcd(x, *y))
    }

    cultivars
        .iter()
        .map(|x| x.harvest_interval.as_secs())
        .collect::<Vec<_>>()
        .pipe(chained_gcd)
        .pipe(Duration::from_secs)
}

pub fn prepetually_contemplate_harassment(cultivars: Vec<cultivars::Cultivar>) {}

fn harass() {
    todo!()
}

fn harvest(cultivars: Vec<cultivars::Cultivar>) -> String {
    todo!()
}

fn vocalize(this: String) {
    todo!()
}

struct Context {
    tracked_task: Task,
    todays_tasks: Vec<Task>,
}

mod cultivars {
    use std::{time::Duration};

    pub struct Cultivar {
        pub harvest_interval: Duration,
        pub harvest_behavior: Box<dyn Fn() -> Vec<String>>,
    }

    // Kinda hate that these are functions ionstead of constants,
    // but it ended up being a pain in the ass to make them constants.
    // idk enough about rust for that yet

    fn current_time() -> Cultivar {
        Cultivar {
            harvest_interval: Duration::from_secs(60 * 15),
            harvest_behavior: Box::new(|| todo!()),
        }
    }

    fn fraction_of_duration() -> Cultivar {
        Cultivar {
            harvest_interval: Duration::from_secs(60 * 5),
            harvest_behavior: Box::new(|| todo!()),
        }
    }

    fn duration_exceeded() -> Cultivar {
        Cultivar {
            harvest_interval: Duration::from_secs(60 * 5),
            harvest_behavior: Box::new(|| todo!()),
        }
    }

    fn prioritize() -> Cultivar {
        Cultivar {
            harvest_interval: Duration::from_secs(60 * 60 * 1),
            harvest_behavior: Box::new(|| todo!()),
        }
    }
}
