use pipe_trait::*;
use reqwest;
use serde::Deserialize;

// Definition taken from https://github.com/amazingmarvin/MarvinAPI/wiki/Marvin-Data-Types#tasks then converted to rust. The definitions are in a consistent format and there's absolutley a way to automate this.
// Commented out all the fields that errored after doing a regex find and replace because I don't actually need any of them right now hehe
#[derive(Deserialize, Debug)]
pub struct Task {
    pub _id: String, // The task's unique ID.  For tasks generated from recurring tasks, this is of the form `${date}-${id}` where date is YYYY-MM-DD and id is the recurring task's id.
                     // createdAt: Number, // Date.now() when the task was created.
                     // updatedAt: Number, // Date.now() when the task was last updated.
                     // workedOnAt: Number, // Date.now() when the task was last worked on (e.g. a subtask was completed).
                     // title: String, // The task's title, like "Go to market".
                     // parentId: String, // ID of parent project or category, or "unassigned".
                     // dueDate: String, // When this project should be completed, formatted as "YYYY-MM-DD". Or null if no dueDate.
                     // startDate: String, // When this task can be started, formatted as "YYYY-MM-DD". Or null if no start date.
                     // endDate: String, // When this task should be completed (soft deadline), formatted as "YYYY-MM-DD". Or null if no end date.
                     // day: String, // Which day the task is scheduled, in the format "YYYY-MM-DD", or "unassigned".  Guaranteed to be "YYYY-MM-DD" if the task is done.
                     // firstScheduled: String, // Which day the task was first assigned to, formatted as "YYYY-MM-DD" or "unassigned" if it was never scheduled yet. Used to calculate how many !!! in procrastination strategy.
                     // plannedWeek: String, // Which week the task is planned for. Date of the Monday of the ISO week (Mon-Sun) formatted as "YYYY-MM-DD"
                     // plannedMonth: String, // Which month the task is planned for. "YYYY-MM"
                     // sprintId: String, // The task's sprint, null if nothing
                     // rank: Number, // The task's sort order in the day view.
                     // masterRank: Number, // The task's sort order within the master list.  Having two ranks is necessary since scheduled tasks are shown in the master list, just grayed out.  And completed tasks within a project are shown crossed out.
                     // done: Boolean, // True if this task has been completed.
                     // // completedAt: Number|null, // Date.now() when this task was completed.
                     // duration: Number, // How long the user worked on this task (in ms). As of 1.18.0, only set when the task is done.
                     // // times: Number[], // Array of Date.now() when time tracking started (odd indexes) and stopped (even indexes).  Updated when time tracking stops and when the task is marked done (or when manually edited by user).
                     // firstTracked: Number, // When this task was first tracked (Date.now()).
                     // doneAt: Number, // When this task was completed (Date.now()).
                     // isReward: Boolean, // True if this task is a reward task.
                     // isStarred: Boolean, // Used for priorities strategy. 3=red, 2=orange, 1=yellow (or true from old version).
                     // isFrogged: Boolean, // True if this task has been frogged for eatThatFrog. 1=normal, 2=baby, 3=monster.
                     // isPinned: Boolean, // Whether this task has been pinned to the master list.  In other words, scheduling it will just schedule a copy, and this task will remain in the master list.
                     // pinId: String, // The pinned task that this task was copied from.
                     // recurring: Boolean, // True if this task was generated via a recurring task.
                     // recurringTaskId: String, // The recurring task that generated this task.
                     // echo: Boolean, // True if this is an "echo" task (from RecurringTask type="echo").
                     // echoId: String, // ID of task used to create this task (from RecurringTask type="echo").
                     // link: String, // System-created tasks can have links, i.e. to "/braindump".  This is in the link target.
                     // // subtasks: Object.<String,Subtask>, // ID => Subtask.
                     // colorBar: String, // One of null, "red", "yellow", "green" or "blue". No longer used.
                     // // labelIds: String[], // The IDs of labels assigned to the task.  Any labelId that doesn't correspond to an existing label in strategySettings.labels should be ignored.
                     // timeEstimate: Number, // How long the user thinks the task will take, in ms.
                     // note: String, // Task note for "notes" strategy.
                     // email: String, // Email HTML used to create note via "email" strategy.
                     // dailySection: String, // Section used in dailyStructure, one of "Morning", "Afternoon", or "Evening".
                     // bonusSection: String, // Section used in bonusStructure, either "Bonus" or "Essential".
                     // customSection: String, // Section used in customStructure (ID of a custom section stored in strategySettings.customStructure).
                     // timeBlockSection: String, // Section used in plannerStructure (ID of a time block).
                     // // dependsOn: Object.<String,Boolean>, // ID => true. Task and project IDs of items that have to be completed before this item can be worked on.
                     // backburner: Boolean, // Tasks created in the backburner are given this property if they don't get the backburner status from their parent. Tasks can also be in the backburner due to a label, dependency, start date, or inheritance without having backburner=true.
                     // reviewDate: String, // Date when user wants to review a task, formatted as "YYYY-MM-DD".
                     // itemSnoozeTime: Number, // Date.now() until when task is snoozed.  While snoozed the task is hidden everywhere except the master list.
                     // permaSnoozeTime: String, // Time (HH:mm) until when task is snoozed (NOT cleared on reschedule).
                     // calId: String, // ID (from Marvin) of calendar this task has been created from / assigned to.
                     // calURL: String, // Unique URL of this task in the calendar.
                     // etag: String, // Calendar etag to determine when an update is needed.
                     // calData: String, // Calendar data.  This string is modified and sent to server to update when tasks in Marvin change.
                     // generatedAt: Number, // Date.now() when this task was created as a recurring task instance.
                     // echoedAt: Number, // Date.now() when this task was created by the completion/deletion of a "repeat after X days" recurring task instance.
                     // deletedAt: Number, // Date.now() when this task was added to the trash.
                     // restoredAt: Number, // Date.now() when this task was restored from the trash.
                     // onboard: Boolean, // Set to true if this task is an onboarding task (i.e. one that Marvin created when the account was created).
                     // imported: Boolean, // Set to true if this task was imported in the Importer.

                     // // Gamification
                     // marvinPoints: Number, // How many kudos you got for this task.
                     // // mpNotes: String[], // Notes on how Marvin awarded you kudos when you completed the task.
                     // rewardPoints: Number, // How many reward points this task awards.
                     // rewardId: Number, // ID of attached reward. Earned on completion.

                     // // Goals
                     // g_in_GOALID: Boolean, // If true, then a task is in a goal. It can also be in a goal from inheritance (i.e. if parent project is in goal).
                     // g_sec_GOALID: String, // The ID of the section/phase of the goal this task lives in. If not given, then the task goes in the first section.
                     // g_rank_GOALID: Number, // The rank of the task within its goal section.

                     // // NEW REMINDER FORMAT
                     // taskTime: String, // "HH:mm" time extracted from title
                     // reminderOffset: Number, // Reminder offset, either manually set or taken from default at reminder creation.
                     // reminderTime: String, // The unix timestamp (seconds) of the first reminder (i.e. before any snoozes), computed with taskTime and reminderOffset (or defaultOffset)
                     // snooze: Number, // Snooze duration, either manually set or taken from default at reminder creation.
                     // autoSnooze: Number, // Whether to autoSnooze, either manually set or taken from default at reminder creation.

                     // // OLD REMINDER FORMAT
                     // remindAt: String, // Time when user should be reminded, "YYYY-MM-DD HH:mm".
                     // reminder: Object, // How remindAt was chosen, so that if the task is renamed, the reminder can be updated.
                     // reminder.time: String, // Event time that was used to create the event.
                     // reminder.diff: Number, // Number of ms before reminder.time when remindAt is scheduled.
}

use serde::de::DeserializeOwned;

use crate::marvin_error::MarvinError;
use crate::secrets;
use reqwest::blocking::Response;

type PotentialResponse = Result<Response, MarvinError>;

fn request(endpoint: &str) -> PotentialResponse {
    let client = reqwest::blocking::Client::new();
    return client
        .post(format!("https://serv.amazingmarvin.com/api/{endpoint}"))
        .header("Content-Length", 0)
        .header("X-Full-Access-Token", secrets::FULL_ACCESS_TOKEN)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .or_else(|error| Err(MarvinError::RequestSend(error)))?
        .error_for_status()
        .or_else(|error| Err(MarvinError::BadRequest(error)))?
        .pipe(Ok);
}

fn springtrap<T: DeserializeOwned>(response: Response) -> Result<T, MarvinError> {
    response
        .json::<T>()
        .or_else(|error| Err(MarvinError::RequestSend(error)))?
        .pipe(Ok)
}

/// Obtain something and crush it into the desired shape in one swift motion.
/// [Visualization](https://youtube.com/clip/Ugkx6_yCmtd_YFHRkAGk9xv5SccrW9Tjq-rC?si=-xF9pB4civAFyWGc)
fn nuzzner_fubs<T: DeserializeOwned>(endpoint: &str) -> Result<T, MarvinError> {
    endpoint.pipe(request)?.pipe(springtrap::<T>)?.pipe(Ok)
}

pub fn today_items() -> Result<Vec<Task>, MarvinError> {
    nuzzner_fubs("todayItems")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        request("test")
            .expect("Looks like the test request failed, maybe your credentials are wrong?");

        assert!(matches!(
            request("invalid url")
                .expect_err("Apparently that's a valid URL??? What?? How did you even get here??"),
            MarvinError::BadRequest(_)
        ));
    }
}
