use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
    pub done_by: Option<String>,
    pub done_at: Option<String>,
}

pub fn create_task(text: String) -> Task {
    Task {
        id: Uuid::new_v4(),
        text,
        done: false,
        done_at: None,
        done_by: None,
    }
}

pub fn create_task_list(tasks: Vec<Task>) -> TaskList {
    return TaskList { tasks };
}

#[derive(Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

pub trait ListCrud {
    fn add(&mut self, text: String);
    fn delete(&mut self, id: Uuid);
}

impl ListCrud for TaskList {
    fn add(&mut self, text: String) {
        let _ = &self.tasks.push(create_task(text));
    }
    fn delete(&mut self, id: Uuid) {
        match &self.tasks.iter().position(|x| x.id == id) {
            None => {}
            Some(index) => {
                let _ = &self.tasks.remove(index.to_owned());
            }
        }
    }
}
