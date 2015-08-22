use player::EquipSlotName;
use item::Item;

pub trait Equippable : Item {
	fn slot(&self) -> EquipSlotName;
	//fn equip(&self, Player);
	//fn price
}
