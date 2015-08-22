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
	coins: u32,
}

impl Player {
	fn new() -> Player {
		Player {
			inventory: Vec::new(),
			equipment: EquipSlots::new(),
			coins: 0,
		}
	}

	fn equip(&mut self, equipment: Box<Equippable>) {
		use EquipSlotName::*;

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

	fn pickup(&mut self, item: Box<Item>) {
		self.inventory.push(item);
	}
}

trait Item {
	fn name(&self) -> String;
	fn examine(&self) -> String;
}

trait Equippable : Item {
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
	fn slot(&self) -> EquipSlotName {
		self.slot
	}
}

impl Item for Weapon {
	fn name(&self) -> String {
		self.name.to_string()
	}

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
	fn slot(&self) -> EquipSlotName {
		self.slot
	}
}

impl Item for Armor {
	fn name(&self) -> String {
		self.name.to_string()
	}

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
	Player::equip(&mut player, Box::new(helm));

	//player.pickup(Box::new(axe));
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