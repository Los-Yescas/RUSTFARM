use godot::{classes::Texture2D, prelude::*};

pub trait IItem {
    fn pick(&self) -> DynGd<RefCounted, dyn IItem>;
    fn get_name(&self) -> GString ;
    fn get_description(&self) -> GString ;
    fn get_max_stack(&self) -> u16;
    fn get_sprite(&self) -> Gd<Texture2D>;
    fn get_precio(&self) -> u16;
    fn interact(&mut self, _world : Gd<Node2D>, _position : Vector2, objeto : Option<Gd<Node2D>>) -> bool;
}
