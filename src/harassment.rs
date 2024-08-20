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
    use std::time::Duration;

    pub struct Cultivar {
        harvest_interval: Duration,
        harvest_behavior: Box<dyn Fn() -> Vec<String>>,
    }

    fn current_time() -> String {
        todo!()
    }

    fn fraction_of_duration() -> String {
        todo!()
    }

    fn duration_exceeded() -> String {
        todo!()
    }

    fn prioritize() -> String {
        todo!()
    }
}
