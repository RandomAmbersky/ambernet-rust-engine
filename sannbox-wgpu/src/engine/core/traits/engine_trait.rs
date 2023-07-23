pub trait AsnEngine {
	fn init(&mut self);
	fn run(self);
}
