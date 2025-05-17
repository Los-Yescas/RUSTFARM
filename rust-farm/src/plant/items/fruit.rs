use godot::{classes::Texture2D, obj::NewGd, prelude::*};

use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct FruitItemResource{
    base : Base<Resource>,
    #[export]
    nombre : GString,
    #[export]
    descripcion : GString,
    #[export]
    max_stack : u16,
    #[export]
    textura : Option<Gd<Texture2D>>,
    #[export]
    precio : u16,
    #[export]
    ruta_de_semilla_a_dar : GString,
    #[export]
    semillas_a_dar : u16
}


impl FruitItemResource {
    pub fn give_seeds(&self) -> DynGd<RefCounted, dyn IItem>{
        let semilla = load::<Resource>(&self.ruta_de_semilla_a_dar);
        semilla.to_variant().to()
    }
}

#[godot_dyn]
pub impl IItem for FruitItemResource {
    fn get_name(&self) -> GString {
        self.nombre.clone()
    }
    fn get_description(&self) -> GString{
        self.descripcion.clone()
    }
    fn get_sprite(&self) -> Gd<Texture2D>{
        self.textura.clone().unwrap_or(Texture2D::new_gd())
    }
    fn get_max_stack(&self) -> u16 {
        self.max_stack
    }
    fn interact(&self, mut _world : Gd<Node2D>, _postion : Vector2) -> bool {
        false
    }
    fn get_precio(&self) -> u16{
        self.precio
    }
}