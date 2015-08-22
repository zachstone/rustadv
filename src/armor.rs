use player::EquipSlotName;
use player::Stats;
use item::Item;
use item::ItemType::Equippable;

pub fn new<'a>(name: &'a str, description: &'a str, slot: EquipSlotName, armor: u64, cost: u64) -> Item<'a> {
	Item::new(name, description, cost,
		Equippable (
			slot,
			Stats {
				car: armor,
				.. Stats::none()
			}
		)
	)
}
