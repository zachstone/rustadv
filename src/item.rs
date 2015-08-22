pub trait Item {
	fn name(&self) -> String;
	fn examine(&self) -> String;
}