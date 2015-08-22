use player::EquipSlotName;
use player::Stats;

pub struct Item<'a> {
	name: &'a str,
	description: &'a str,
	cost: u64,
	properties: ItemType,
}

impl<'a> Item<'a> {
	pub fn new(name: &'a str, description: &'a str, cost: u64, properties: ItemType) -> Item<'a> {
		Item{name: name, description: description, cost: cost, properties: properties}
	}
	pub fn name(&self) -> &str {
		self.name
	}

	pub fn examine(&self) -> &str {
		self.description
	}

	pub fn cost(&self) -> u64 {
		self.cost
	}

	pub fn properties(&self) -> &ItemType {
		&self.properties
	}
}

pub enum ItemType {
	Equippable(EquipSlotName, Stats),
	Consumable{effect: Stats, duration: u64},
	None
}
