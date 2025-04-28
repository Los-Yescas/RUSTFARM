use godot::{
    classes::{SpriteFrames, Texture2D},
    prelude::*,
};

use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct PlantResource {
    base: Base<Resource>,
    #[export]
    nombre: GString,
    #[export]
    crecimiento_minimo: u32,
    #[export]
    crecimiento_maximo: u32,
    #[export]
    puntos_para_crecer: u32,
    #[export]
    #[init(val = None)]
    sprite: Option<Gd<SpriteFrames>>,
    // #[export]
    // #[init(val = None)]
    // inventory_icon: Option<Gd<Texture2D>>,
}

// #[godot_api]
// impl IItem for PlantResource {
//     fn get_name(&self) -> GString {
//         self.nombre.clone()
//     }

//     fn get_description(&self) -> GString {
//         GString::from("Una planta en crecimiento.")
//     }

//     fn get_max_stack(&self) -> u16 {
//         1 // Aquí defines cuántos puedes apilar en el inventario
//     }

//     fn get_sprite(&self) -> Gd<Texture2D> {
//         self.inventory_icon.clone().unwrap_or_default()
//     }

//     fn get_price(&self) -> u16 {
//         10 // Puedes ponerle cualquier precio que quieras
//     }

//     fn interact(&self, _world: Gd<Node2D>, _position: Vector2) {
//         // Qué pasa cuando el jugador usa esta planta en el mundo
//         godot_print!("¡Interacción con la planta en el mundo!");
//     }
// }
