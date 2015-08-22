#![allow(dead_code)]

mod player;
mod equip_slots;
mod item;
mod equippable;
mod armor;
mod weapon;

fn main() {
	let axe = weapon::Weapon {
		name: "Battleaxe".to_string(),
		damage: 12,
		speed: 0.71,
		slot: equip_slots::EquipSlotName::RightHand,
		text: "An Axe.".to_string(),
		value: 1200,
	};

	let helm = armor::Armor {
		name: "Dragon Helm".to_string(),
		armor: 36,
		slot: equip_slots::EquipSlotName::Head,
		text: "A helm created from dragon bones.".to_string(),
		value: 23000,
	};

	//let sword: Weapon = Item::new();
	let mut player = player::Player::new();
	player.equip(Box::new(axe));
	player.equip(Box::new(helm));

	//player.pickup(Box::new(axe));
	//let x = player.equipment.right_hand;

	player.equipment.print_equipment();
}