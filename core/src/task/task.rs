// Task Management System used by Merge
// Copyright © 2023 Merge. All Rights Reserved.

pub trait Task {
    fn run(&mut self);
}

pub struct TaskManager {
    pub tasks: Vec<Box<dyn Task>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn pop(&mut self) {
        self.tasks.pop();
    }

    pub fn push(&mut self, task: Box<dyn Task>) {
        self.tasks.push(task);
    }

    pub fn run_all(&mut self) {
        for task in self.tasks.iter_mut() {
            task.run();
        }
    }
}
