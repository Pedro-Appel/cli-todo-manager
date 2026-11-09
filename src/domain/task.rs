#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskId(u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    title: String,
    done: bool,
}

impl Task {
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Expected to be used in task 1.4")
    )]
    pub fn new(id: TaskId, title: String) -> Result<Self, String> {
        if title.is_empty() || title.trim().is_empty() {
            return Err("Task title cannot be empty".into());
        }
        Ok(Task {
            id,
            title,
            done: false,
        })
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Expected to be used in task 1.4")
    )]
    pub fn id(&self) -> TaskId {
        self.id
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Expected to be used in task 1.4")
    )]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Expected to be used in task 1.4")
    )]
    pub fn is_done(&self) -> &bool {
        &self.done
    }

    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "Expected to be used in task 1.4")
    )]
    pub fn complete(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_creates_a_new_task_with_valid_title() {
        let result = Task::new(TaskId(1), "Test Task".to_string()).unwrap();
        assert_eq!("Test Task", result.title());
        assert_eq!(TaskId(1), result.id());
        assert!(!result.is_done());
    }

    #[test]
    fn task_transition_state() {
        let mut result = Task::new(TaskId(1), "Test Task".to_string()).unwrap();
        assert!(!result.is_done());
        result.complete();
        assert!(result.is_done());
    }

    #[test]
    fn invalid_task_title_should_have_error() {
        let result = Task::new(TaskId(1), "".to_string());
        assert!(result.is_err());

        let result2 = Task::new(TaskId(1), "        ".to_string());
        assert!(result2.is_err());
    }
}
