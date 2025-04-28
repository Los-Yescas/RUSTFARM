use godot::{classes::Texture2D, prelude::*};

pub trait IItem {
    fn get_name(&self) -> GString;
    fn get_description(&self) -> GString;
    fn get_max_stack(&self) -> u16;
    fn get_sprite(&self) -> Gd<Texture2D>;
    fn get_price(&self) -> u16;
    fn interact(&self, _world: Gd<Node2D>, _position: Vector2);
}

pub mod plant_items;
