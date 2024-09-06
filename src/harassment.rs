use crate::marvin::Task;

fn contemplate_harassment() {
    todo!()
}

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
    use std::{sync::LazyLock, time::Duration};

    pub struct Cultivar {
        harvest_interval: Duration,
        harvest_behavior: Box<dyn Fn() -> Vec<String>>,
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
