use equippable::*;

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

pub struct EquipSlots<'a> {
	pub right_hand: Option<&'a Equippable>,
	pub left_hand: Option<&'a Equippable>,
	pub head: Option<&'a Equippable>,
	pub chest: Option<&'a Equippable>,
	pub shoulders: Option<&'a Equippable>,
	pub hands: Option<&'a Equippable>,
	pub legs: Option<&'a Equippable>,
	pub feet: Option<&'a Equippable>,
	pub neck: Option<&'a Equippable>,
}

impl<'a> EquipSlots<'a> {
	pub fn new() -> EquipSlots<'a> {
		EquipSlots {
			right_hand: None,
			left_hand: None,
			head: None,
			chest: None,
			shoulders: None,
			hands: None,
			legs: None,
			feet: None,
			neck: None,
		}
	}

	fn print_equip_slot(slot: &Option<&Equippable>, name: &str) {
		match *slot {
			Some(ref x) => println!("{}: {}", name.to_string(), (*x).name()),
			None => println!("{}: Nothing", name.to_string()),
		}
	}

	pub fn print_equipment(&self) {
		EquipSlots::print_equip_slot(&self.right_hand, "Right Hand");
		EquipSlots::print_equip_slot(&self.left_hand, "Left Hand");
		EquipSlots::print_equip_slot(&self.head, "Head");
		EquipSlots::print_equip_slot(&self.chest, "Chest");
		EquipSlots::print_equip_slot(&self.shoulders, "Shoulders");
		EquipSlots::print_equip_slot(&self.hands, "Hands");
		EquipSlots::print_equip_slot(&self.legs, "Legs");
		EquipSlots::print_equip_slot(&self.feet, "Feet");
		EquipSlots::print_equip_slot(&self.neck, "Neck");

	}
}
