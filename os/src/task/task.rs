//! Types related to task management

use crate::{config::CUR_SYSCALL_NUM, timer};

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task start time in ms
    pub task_start: usize,
    /// The task syscall times
    pub task_syscall_times: [u32; CUR_SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

impl TaskControlBlock {
    /// set the task start time
    pub fn set_start(&mut self) {
        match self.task_start {
            0 => self.task_start = timer::get_time_ms(),
            _ => {}
        }
    }
}
