use godot::{classes::Texture2D, obj::NewGd, prelude::*};

use crate::{item::item_resource::IItem, plant::{plant_resource::PlantResource, plant_node::Planta}};

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct SeedItemResource{
    base : Base<Resource>,
    #[export]
    nombre : GString,
    #[export]
    descripcion : GString,
    #[export]
    max_stack : u16,
    #[export]
    ruta_de_planta_a_plantar : GString,
    #[export]
    textura : Option<Gd<Texture2D>>,
    #[export]
    precio : u16
}

#[godot_dyn]
pub impl IItem for SeedItemResource {
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
    fn interact(&self, mut world : Gd<Node2D>, postion : Vector2) {
        let recurso_planta : Gd<PlantResource> = load(&self.ruta_de_planta_a_plantar);

        let mut new_planta = Planta::from_resource(recurso_planta);
        new_planta.set_position(postion);
        world.add_child(&new_planta);
    }

    fn get_precio(&self) -> u16 {
        self.precio
    }
}