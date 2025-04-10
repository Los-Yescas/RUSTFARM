use godot::prelude::*;

pub trait IItem {
    fn get_name(&self) -> GString {
        "Item Name should go here".into()
    }
    fn get_description(&self) -> GString {
        "Item Description should go here".into()
    }
    fn get_max_stack(&self) -> u32 {
        16
    }
    fn interact(&self, _world : Gd<Node2D>, _postion : Vector2) {
        godot_print!("Your interaction should go here")
    }
}

pub mod plant_items;
