use equip_slots;
use item;
use equippable;

pub struct Armor {
	pub name: String,
	pub armor: u32,
	pub slot: equip_slots::EquipSlotName,
	pub text: String,
	pub value: u32,
}

impl equippable::Equippable for Armor {
	fn slot(&self) -> equip_slots::EquipSlotName {
		self.slot
	}
}

impl item::Item for Armor {
	fn name(&self) -> String {
		self.name.to_string()
	}

	fn examine(&self) -> String {
		self.text.to_string()
	}
}