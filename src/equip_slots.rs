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
	/*
	fn print_equip_slot(slot: Option<Box<Equippable>>, name: &str) {
		match slot {
			Some(ref x) => format!("{}: {}", name, (*x).name()),
			None => format!("{}: Nothing", name),
		}
	}
	*/
	pub fn print_equipment(&self) {
		//EquipSlots::print_equip_slot(self.right_hand, "Right Hand");
		match self.right_hand {
			Some(ref x) => println!("Right hand: {}", (*x).name()),
			None => println!("Right hand: Nothing"),
		}
		match self.left_hand {
			Some(ref x) => println!("Left hand: {}", (*x).name()),
			None => println!("Left hand: Nothing"),
		}
		match self.head {
			Some(ref x) => println!("Head: {}", (*x).name()),
			None => println!("Head: Nothing"),
		}
	}
}