struct TaskBuf {
    tasks: Vec<Task>,
}

impl TaskBuf {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn push(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn pop(&mut self) -> Option<Task> {
        self.tasks.pop()
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }
}

// Task implementation
struct Task {
    name: String,
    task: Box<dyn Fn() -> ()>,
}
