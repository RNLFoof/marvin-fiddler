use num_traits::Float;
use std::ops::Div;
use std::time::{Duration, Instant};

/* In another language, all of these functions would be overloads. In this language, if specialization was stable, at least two of those functions would be Trait methods. */

/// Calculates the imparativeness of a task being completed, in the form of an abstract number comparable to other instances of the same.
///
/// Ex. A task that should have been done in a week but it has since been two weeks, vs a task that should have been done in a day and it has since been half a day,
/// would have imperativenesses of 2 and 0.5 respectively. The former task is of a higher imparativeness (meaning it should be prioritized in comparison)
///
/// * [`time_appraisal`] : How long the task *should* be done in. That is, this much time later, it should be done, *not* how long it would take of continious effort to actually do.
///   [This can be any unit of time, but it needs to be the same unit of time as `time_reality`. Obviously.](https://www.jpl.nasa.gov/missions/mars-climate-orbiter)
/// * [`time_reality`] : How long its been since the time appraisal was made. [This can be any unit of time, but it needs to be the same unit of time as `time_appraisal`. Obviously.](https://www.jpl.nasa.gov/missions/mars-climate-orbiter)
/* It could be `Num + Div` instead if the output was forced to always be a float. I don't feel like implementing this.
The issue with letting it be an integer is that low numbers would produce actively inaccurate results, and low numbers are likely (ex. a number of days) */
fn abstract_imperativeness<I: Float + Div>(
    time_appraisal: I,
    time_reality: I,
) -> <I as Div>::Output {
    if time_appraisal.is_zero() {
        num_traits::identities::zero()
    } else {
        time_reality / time_appraisal
    }
}

/// Calculates the imparativeness of a task being completed, in the form of an abstract number comparable to other instances of the same.
///
/// Ex. A task that should have been done in a week but it has since been two weeks, vs a task that should have been done in a day and it has since been half a day,
/// would have imperativenesses of 2 and 0.5 respectively. The former task is of a higher imparativeness (meaning it should be prioritized in comparison)
///
/// * [`time_appraisal`] : How long the task *should* be done in. That is, this much time later, it should be done, *not* how long it would take of continious effort to actually do.
/// * [`time_reality`] : How long its been since the time appraisal was made.
fn imperativeness(time_appraisal: Duration, time_reality: Duration) -> f32 {
    abstract_imperativeness(
        time_appraisal.as_millis() as f32,
        time_reality.as_millis() as f32,
    )
}

/// Calculates the imparativeness of a task being completed, in the form of an abstract number comparable to other instances of the same.
///
/// Ex. A task that should have been done in a week but it has since been two weeks, vs a task that should have been done in a day and it has since been half a day,
/// would have imperativenesses of 2 and 0.5 respectively. The former task is of a higher imparativeness (meaning it should be prioritized in comparison)
///
/// * [`time_appraised_at`] : When the `time_appraisal` was made (or when it was intended to be relevant to, ex. a form that should be filled in within a week of recieving it, but you're only going to recieve it at the end of the month)
/// * [`time_appraisal`] : How long the task *should* be done in. That is, this much time later, it should be done, *not* how long it would take of continious effort to actually do.
fn imperativeness_from_timestamps(time_appraised_at: Instant, time_appraisal: Duration) -> f32 {
    imperativeness(time_appraisal, time_appraised_at.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abstract_imperativeness() {
        assert_eq!(super::abstract_imperativeness(0.0, 2.0), 0.0);
        assert_eq!(super::abstract_imperativeness(1.0, 2.0), 2.0);
        assert_eq!(super::abstract_imperativeness(2.0, 1.0), 0.5);
        assert_eq!(super::abstract_imperativeness(1 as f32, 2 as f32), 2.0);
    }

    #[test]
    fn imperativeness() {
        assert_eq!(
            super::imperativeness(Duration::from_secs(1), Duration::from_secs(2)),
            2.0
        );
    }

    #[test]
    fn imperativeness_from_timestamps() {
        let one_minute = Duration::from_secs(60);
        let one_hour = one_minute * 60;
        let one_day = one_hour * 24;
        assert_eq!(
            format!(
                "{:.1}",
                super::imperativeness_from_timestamps(Instant::now() - one_day, one_day * 2)
            ),
            "0.5"
        );
    }
}
