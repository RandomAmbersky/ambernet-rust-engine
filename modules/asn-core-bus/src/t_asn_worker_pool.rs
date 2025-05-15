pub trait AsnWorkerPool<F>
where
    F: FnOnce(),
{
    fn run_main_thread(&self, func: F) -> Result<(), String>;
    fn run_thread(&self, func: F) -> Result<(), String>;
}
