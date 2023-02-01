/**************************************************************************************************
* Name : 									 task/mod.rs
* Author : 										Avery
* Date : 									  2/01/2023
* Purpose : 					       Async/Await using Tasks
* Version : 									 0.1
* Comment :     Goodbye, blog_os. You've helped us so much with setting up this project.
**************************************************************************************************/

pub mod executor;
pub mod simple_executor;
pub mod keyboard;

use core::{task::{Context, Poll}, future::Future, pin::Pin, sync::atomic::{AtomicU64, Ordering}};
use alloc::boxed::Box;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}