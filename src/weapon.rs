use player::EquipSlotName;
use player::Stats;
use item::ItemType::Equippable;
use item::Item;

pub fn new<'a>(name: &'a str, description: &'a str, slot: EquipSlotName, damage: u64, speed: f32, cost: u64) -> Item<'a> {
	Item::new(name, description, cost,
		Equippable (slot,
			Stats {
				dmg: damage,
				apm: speed,
				.. Stats::none()
			}
		)
	)
}
