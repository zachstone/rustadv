use item::Item;
use equip_slots::EquipSlots;
use equippable::Equippable;
use inventory::Inventory;

pub struct Player<'a> {
	pub inventory: Inventory<'a>,
	pub equipment: EquipSlots<'a>,
	pub coins: u64,
}

impl<'a> Player<'a> {
	pub fn new() -> Player<'a> {
		Player {
			inventory: Inventory::new(),
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
