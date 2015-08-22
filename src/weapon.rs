use equip_slots::EquipSlotName;
use equippable::Equippable;
use item::Item;

pub struct Weapon {
	pub name: String,
	pub damage: u32,
	pub speed: f32,
	pub slot: EquipSlotName,
	pub text: String,
	pub value: u32,
}

impl Equippable for Weapon {
	fn slot(&self) -> EquipSlotName {
		self.slot
	}
}

impl Item for Weapon {
	fn name(&self) -> String {
		self.name.to_string()
	}

	fn examine(&self) -> String {
		self.text.to_string()
	}
}
