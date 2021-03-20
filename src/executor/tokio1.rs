use crate::join_handle::{InnerJoinHandle, JoinHandle};
use crate::{AgnostikExecutor, LocalAgnostikExecutor};
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio1_crate as tokio;

/// A wrapper around the `tokio` (version 1.*) crate which implements `AgnostikExecutor` and
/// `LocalAgnostikExecutor`.
pub struct Tokio1Executor(Mutex<Arc<tokio::runtime::Runtime>>);

impl Tokio1Executor {
    /// Create a new `Tokio1Executor`.
    pub fn new() -> Self {
        Self::with_runtime(tokio::runtime::Runtime::new().expect("failed to create runtime"))
    }

    /// Create a new `TokioExecutor` with a custom runtime.
    pub fn with_runtime(runtime: tokio::runtime::Runtime) -> Self {
        Tokio1Executor(Mutex::new(Arc::new(runtime)))
    }

    pub(crate) fn set_runtime(&self, runtime: tokio::runtime::Runtime) {
        let mut inner = self.0.lock().unwrap();
        *inner = Arc::new(runtime);
    }
}

impl AgnostikExecutor for Tokio1Executor {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let rt = self.0.lock().unwrap().clone();
        let handle = rt.spawn(future);
        JoinHandle(InnerJoinHandle::Tokio1(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let rt = self.0.lock().unwrap().clone();
        let handle = rt.spawn_blocking(task);
        JoinHandle(InnerJoinHandle::Tokio1(handle))
    }

    fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let rt = self.0.lock().unwrap().clone();
        rt.block_on(future)
    }
}

impl LocalAgnostikExecutor for Tokio1Executor {
    fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        // TODO: use our `Runtime` somehow?
        let handle = tokio::task::spawn_local(future);
        JoinHandle(InnerJoinHandle::Tokio1(handle))
    }
}
