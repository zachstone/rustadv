use equip_slots;
use equippable;
use item;

pub struct Weapon {
	pub name: String,
	pub damage: u32,
	pub speed: f32,
	pub slot: equip_slots::EquipSlotName,
	pub text: String,
	pub value: u32,
}

impl equippable::Equippable for Weapon {
	fn slot(&self) -> equip_slots::EquipSlotName {
		self.slot
	}
}

impl item::Item for Weapon {
	fn name(&self) -> String {
		self.name.to_string()
	}

	fn examine(&self) -> String {
		self.text.to_string()
	}
}