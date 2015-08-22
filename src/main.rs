#![allow(dead_code)]

mod player;
mod equip_slots;
mod item;
mod equippable;
mod armor;
mod weapon;

use player::Player;
use armor::Armor;
use equip_slots::EquipSlotName::*;
use weapon::Weapon;

fn main() {
	let axe = Weapon {
		name: "Battleaxe".to_string(),
		damage: 12,
		speed: 0.71,
		slot: RightHand,
		text: "An Axe.".to_string(),
		value: 1200,
	};

	let helm = Armor {
		name: "Dragon Helm".to_string(),
		armor: 36,
		slot: Head,
		text: "A helm created from dragon bones.".to_string(),
		value: 23000,
	};

	let boots = Armor {
		name: "Boots of Blinding Speed".to_string(),
		armor: 9001,
		slot: Feet,
		text: "A blatant Morrowind reference.".to_string(),
		value: 500,
	};

	//let sword: Weapon = Item::new();
	let mut player = Player::new();
	player.equip(&axe);
	player.equip(&helm);
	player.equip(&boots);

	//player.pickup(Box::new(axe));
	//let x = player.equipment.right_hand;

	player.equipment.print_equipment();
}
