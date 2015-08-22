use equippable;

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

pub struct EquipSlots {
	pub right_hand: Option<Box<equippable::Equippable>>,
	pub left_hand: Option<Box<equippable::Equippable>>,
	pub head: Option<Box<equippable::Equippable>>,
	pub chest: Option<Box<equippable::Equippable>>,
	pub shoulders: Option<Box<equippable::Equippable>>,
	pub hands: Option<Box<equippable::Equippable>>,
	pub legs: Option<Box<equippable::Equippable>>,
	pub feet: Option<Box<equippable::Equippable>>,
	pub neck: Option<Box<equippable::Equippable>>,
}

impl EquipSlots {
	pub fn new() -> EquipSlots {
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
	
	fn print_equip_slot(slot: &Option<Box<equippable::Equippable>>, name: &str) {
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