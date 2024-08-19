use crate::marvin::Task;

/// Additional variables stored in the notes of a task, in the form of "➕" followed by some JSON.
struct ImposedValues {}

// Add baseline imposed values to a task.
fn indoctrinate(task: Task) {
    todo!()
}

// Loads(or initializes) imposed values from a task, lets you view and change them, then saves.
fn manipulate(task: Task, callable: Box<dyn Fn(Task, ImposedValues)>) {
    todo!()
}
