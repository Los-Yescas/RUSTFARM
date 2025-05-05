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
    #[export]
    #[init(default = 1)] // Plantas maduras usualmente no se apilan (1 por slot)
    stack_size: u16,
     #[export]
     precio: u16,
    // #[init(val = None)]
    // inventory_icon: Option<Gd<Texture2D>>,
}
#[godot_dyn]
impl IItem for PlantResource {
    fn get_name(&self) -> GString {
        self.nombre.clone()
    }

    fn get_description(&self) -> GString {
        "Planta cosechada".into() // O usa un campo descripción si lo tienes
    }

    fn get_max_stack(&self) -> u16 {
        self.stack_size
    }

    fn get_sprite(&self) -> Gd<Texture2D> {
        // Implementa según tus necesidades
        Texture2D::new_gd()
    }

    fn get_price(&self) -> u16 {
        self.precio
    }

    fn interact(&self, _world: Gd<Node2D>, _position: Vector2) {
        // No necesaria para plantas cosechadas
    }
}
