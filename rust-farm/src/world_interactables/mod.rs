use godot::{classes::RefCounted, obj::DynGd};

use crate::item::item_resource::IItem;

pub trait IWorldInteractable {
    fn show_menu(&mut self, inventory : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>, points : u16);
}

pub trait IWorldPickable {
    fn pick(&self) -> Option<&DynGd<RefCounted, dyn IItem>>;
    fn has_been_picked(&mut self);
}