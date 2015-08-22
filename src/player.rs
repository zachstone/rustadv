use item;
use equip_slots::*;
use equippable::*;

pub struct Player<'a> {
	pub inventory: Vec<Box<item::Item>>,
	pub equipment: EquipSlots<'a>,
	pub coins: u32,
}

impl<'a> Player<'a> {
	pub fn new() -> Player<'a> {
		Player {
			inventory: Vec::new(),
			equipment: EquipSlots::new(),
			coins: 0,
		}
	}

	pub fn equip(&mut self, equipment: &'a Equippable) {
		use equip_slots::EquipSlotName::*;

		match (*equipment).slot() {
			RightHand => self.equipment.right_hand = Some(equipment),
			LeftHand => self.equipment.left_hand = Some(equipment),
			Head => self.equipment.head = Some(equipment),
			Chest => self.equipment.chest = Some(equipment),
			Shoulders => self.equipment.shoulders = Some(equipment),
			Hands => self.equipment.hands = Some(equipment),
			Legs => self.equipment.legs = Some(equipment),
			Feet => self.equipment.feet = Some(equipment),
			Neck => self.equipment.neck = Some(equipment),
		}
	}

	/*
	fn pickup_if_exists(&self, item: Option<Box<Equippable>>) {
		match item {
			Some(x) => self.pickup(x),
			None => (),
		}
	}

	fn unequip(&self, equip_slot: EquipSlotName) {
		use EquipSlotName::*;

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

	fn pickup(&mut self, item: Box<item::Item>) {
		self.inventory.push(item);
	}
}
