#![allow(dead_code)]

#[derive(Copy, Clone)]
enum EquipSlotName {
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

struct EquipSlots {
	right_hand: Option<Box<Equippable>>,
	left_hand: Option<Box<Equippable>>,
	head: Option<Box<Equippable>>,
	chest: Option<Box<Equippable>>,
	shoulders: Option<Box<Equippable>>,
	hands: Option<Box<Equippable>>,
	legs: Option<Box<Equippable>>,
	feet: Option<Box<Equippable>>,
	neck: Option<Box<Equippable>>,
}

impl EquipSlots {
	fn new() -> EquipSlots {
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
	fn print_equipment(&self) {
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

struct Player {
	inventory: Vec<Box<Item>>,
	equipment: EquipSlots,
}

impl Player {
	fn new() -> Player {
		Player {
			inventory: Vec::new(),
			equipment: EquipSlots::new(),
		}
	}

	fn equip(&mut self, equipment: Box<Equippable>) {
		match (*equipment).slot() {
			EquipSlotName::RightHand => self.equipment.right_hand = Some(equipment),
			EquipSlotName::LeftHand => self.equipment.left_hand = Some(equipment),
			EquipSlotName::Head => self.equipment.head = Some(equipment),
			EquipSlotName::Chest => self.equipment.chest = Some(equipment),
			EquipSlotName::Shoulders => self.equipment.shoulders = Some(equipment),
			EquipSlotName::Hands => self.equipment.hands = Some(equipment),
			EquipSlotName::Legs => self.equipment.legs = Some(equipment),
			EquipSlotName::Feet => self.equipment.feet = Some(equipment),
			EquipSlotName::Neck => self.equipment.neck = Some(equipment),
		}
	}

	fn pickup(&mut self, item: Box<Item>) {
		self.inventory.push(item);
	}
}

trait Item {
	fn examine(&self) -> String;
}

trait Equippable {
	fn name(&self) -> String;
	fn slot(&self) -> EquipSlotName;
	//fn equip(&self, Player);
	//fn price
}

//---

struct Weapon {
	name: String,
	damage: u32,
	speed: f32,
	slot: EquipSlotName,
	text: String,
	value: u32,
}

impl Equippable for Weapon {
	fn name(&self) -> String {
		self.name.to_string()
	}

	fn slot(&self) -> EquipSlotName {
		self.slot
	}
}

impl Item for Weapon {
	fn examine(&self) -> String {
		self.text.to_string()
	}
}

//---

struct Armor {
	name: String,
	armor: u32,
	slot: EquipSlotName,
	text: String,
	value: u32,
}

impl Equippable for Armor {
	fn name(&self) -> String {
		self.name.to_string()
	}

	fn slot(&self) -> EquipSlotName {
		self.slot
	}
}

impl Item for Armor {
	fn examine(&self) -> String {
		self.text.to_string()
	}
}

fn main() {
	let axe: Weapon = Weapon {
		name: "Battleaxe".to_string(),
		damage: 12,
		speed: 0.71,
		slot: EquipSlotName::RightHand,
		text: "An Axe.".to_string(),
		value: 1200,
	};

	let helm: Armor = Armor {
		name: "Dragon Helm".to_string(),
		armor: 36,
		slot: EquipSlotName::Head,
		text: "A helm created from dragon bones.".to_string(),
		value: 23000,
	};

	//let sword: Weapon = Item::new();
	let mut player = Player::new();
	player.equip(Box::new(axe));
	player.equip(Box::new(helm));
	//player.pickup(Item);
	//let x = player.equipment.right_hand;

	player.equipment.print_equipment();
	//format("{:?}", player.inventory);

	/*
	match player.equipment.right_hand {
		Some(x) => println!("{}", (*x).examine()),
		None => println!("Nothing Equipped"),
	}
	*/
	
}