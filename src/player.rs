use item::Item;
use equippable::Equippable;
use inventory::Inventory;
use std::collections::HashMap;

pub struct Player<'a> {
	pub inventory: Inventory<'a>,
	pub equipped: HashMap<isize, &'a Equippable>,
	pub coins: u64,
}

#[derive(Copy, Clone)]
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

impl<'a> Player<'a> {
	pub fn new() -> Player<'a> {
		Player {
			inventory: Inventory::new(),
			equipped: HashMap::new(),
			coins: 0,
		}
	}

	pub fn equip(&mut self, equipment: &'a Equippable) -> Option<&Equippable> {
		let result = self.equipped.remove(&(equipment.slot() as isize));
		self.equipped.insert(equipment.slot() as isize, equipment);
		return result;
	}

	fn get_equipment(&self, slot: EquipSlotName) -> Option<&Equippable> {
		return match self.equipped.get(&(slot as isize)) {
			Some(&e) => Some(e),
			None => None,
		}
	}

	fn print_equip_slot(equipment: &Option<&Equippable>, name: &str) {
		match *equipment {
			Some(x) => println!("{}: {}", name.to_string(), (*x).name()),
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

	pub fn pickup(&mut self, item: &'a Item) {
		self.inventory.add(item);
	}
}
