use item::Item;
use item::ItemType::Equippable;
use std::collections::HashMap;

pub struct Player<'a> {
	pub inventory: Vec<Item<'a>>,
	pub equipped: HashMap<EquipSlotName, Item<'a>>,
	pub coins: u64,
	pub stats: Stats,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum EquipSlotName {
	RightHand,
	LeftHand,
	Head,
	Chest,
	Shoulders,
	Hands,
	Legs,
	Feet,
	Neck,
}

pub struct Stats {
	pub hp: u64,
	pub dmg: u64,
	pub apm: f32,
	pub lib: u64,
	pub hgt: u64,
	pub car: u64,
}

impl Stats {
	pub fn none() -> Stats {
		Stats {
			hp: 0,
			dmg: 0,
			apm: 0.0,
			lib: 0,
			hgt: 0,
			car: 0,
		}
	}
}

impl<'a> Player<'a> {
	pub fn new(stats: Stats) -> Player<'a> {
		Player {
			inventory: Vec::new(),
			equipped: HashMap::new(),
			coins: 0,
			stats: stats,
		}
	}

	pub fn equip(&mut self, equipment: Item<'a>) -> Option<Item> {
		match *equipment.properties() {
			Equippable(slot, _) => self.equipped.insert(slot, equipment),
			_ => None,
		}
	}

	pub fn unequip(&mut self, slot: EquipSlotName) -> Option<Item> {
		self.equipped.remove(&slot)
	}

	fn get_equipment(&self, slot: EquipSlotName) -> Option<&Item> {
		self.equipped.get(&slot)
	}

	fn print_equip_slot(equipment: &Option<&Item>, name: &str) {
		match *equipment {
			Some(x) => println!("{}: {}", name.to_string(), x.name()),
			None => println!("{}: Nothing", name.to_string()),
		}
	}

	pub fn print_equipment(&self) {
		use player::EquipSlotName::*;
		Player::print_equip_slot(&self.get_equipment(RightHand), "Right Hand");
		Player::print_equip_slot(&self.get_equipment(LeftHand), "Left Hand");
		Player::print_equip_slot(&self.get_equipment(Head), "Head");
		Player::print_equip_slot(&self.get_equipment(Chest), "Chest");
		Player::print_equip_slot(&self.get_equipment(Shoulders), "Shoulders");
		Player::print_equip_slot(&self.get_equipment(Hands), "Hands");
		Player::print_equip_slot(&self.get_equipment(Legs), "Legs");
		Player::print_equip_slot(&self.get_equipment(Feet), "Feet");
		Player::print_equip_slot(&self.get_equipment(Neck), "Neck");
	}

	pub fn print_inventory(&self) {
		for item in &self.inventory {
			println!("{}", item.name());
		}
	}

	/*
	fn pickup_if_exists(&self, item: Option<Box<Equippable>>) {
		match item {
			Some(x) => self.pickup(x),
			None => (),
		}
	}

	fn unequip(&self, equip_slot: equip_slots::EquipSlotName) {
		use equip_slots::EquipSlotName::*;

		match equip_slot {
			RightHand => self.inventory.push(self.equipment.right_hand),
			LeftHand => self.inventory.push(self.equipment.left_hand),
			Head => self.inventory.push(self.equipment.head),
			Chest => self.inventory.push(self.equipment.chest),
			Shoulders => self.inventory.push(self.equipment.shoulders),
			Hands => self.inventory.push(self.equipment.hands),
			Legs => self.inventory.push(self.equipment.legs),
			Feet => self.inventory.push(self.equipment.feet),
			Neck => self.inventory.push(self.equipment.neck),
		}
	}
	*/

	pub fn pickup(&mut self, item: Item<'a>) {
		self.inventory.push(item);
	}
}
