use equip_slots;
use item;

pub trait Equippable : item::Item {
	fn slot(&self) -> equip_slots::EquipSlotName;
	//fn equip(&self, Player);
	//fn price
}