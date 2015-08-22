use item::Item;

pub struct Inventory<'a> {
	items: Vec<&'a Item>,
}

impl<'a> Inventory<'a> {
	pub fn new() -> Inventory<'a> {
		Inventory {
			items: Vec::new(),
		}
	}

	pub fn add(&mut self, item: &'a Item) {
		self.items.push(item);
	}

	pub fn print_inventory(&self) {
		for i in &self.items {
			println!("{}", (*i).name());
		}
	}
}