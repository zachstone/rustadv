use equip_slots::EquipSlotName;
use item::Item;
use equippable::Equippable;

pub struct Armor {
	pub name: String,
	pub armor: u32,
	pub slot: EquipSlotName,
	pub text: String,
	pub value: u32,
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
