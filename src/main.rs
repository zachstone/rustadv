#![allow(dead_code)]

mod player;
mod item;
mod armor;
mod weapon;

use player::EquipSlotName::*;
use player::Player;
use player::Stats;
use item::Item;
use item::ItemType;
use item::ItemType::Equippable;

fn main() {
	let axe = weapon::new("Battleaxe", "An Axe.", RightHand, 12, 0.71, 1200);
	let helm = armor::new("Dragon Helm", "A helm created from dragon bones.", Head, 36, 23000);
	let sandals = armor::new("Sandals", "Maybe Jesus once owned these sandals.", Feet, 1, 9999);
	let boots = armor::new("Boots of Blinding Speed", "A blatant Morrowind reference.", Feet, 9001, 500);
	let mcguffin = Item::new("McGuffin of Lesser Plot", "The minor priestess Riella requires this post-haste!", 0, ItemType::None);
	let mcguffin2 = Item::new("McGuffin of Greater Plot", "The major priestess Riellaja requires this post-haste!", 0, ItemType::None);

	let mut player = Player::new(Stats {
		hp: 12,
		dmg: 0,
		apm: 0.0,
		lib: 5,
		hgt: 72,
		car: 0,
	});

	player.equip(axe);
	player.equip(helm);
	player.equip(sandals);

	player.pickup(mcguffin);
	player.pickup(boots);
	player.pickup(mcguffin2);

	let mut i = 0;
	while i < player.inventory.len() {
		if !try_equip(&mut player, i) {
			i += 1;
		}
	}
	player.print_equipment();
	player.print_inventory();
}

fn try_equip<'a>(player: &mut Player<'a>, index: usize) -> bool {
	println!("Equipping {} from slot {}", player.inventory[index].name(), index);
	match *(player.inventory[index]).properties() {
		Equippable(_,_) => {
			let item = player.inventory.swap_remove(index);
			if let Some(dropped) = (*player).equip(item) {
				println!("Dropping {} on the ground!", dropped.name());
			}
			true
		},
		_ => {
			println!("Can't equip {}!", player.inventory[index].name());
			false
		}
	}
}
