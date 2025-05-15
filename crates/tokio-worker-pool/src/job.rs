pub type Job = Box<dyn FnOnce(usize) + Send + 'static>;
