use godot::prelude::*;

use crate::{item::item_resource::IItem, plant::{plant_resource::PlantResource, Planta}};

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct SeedItemResource{
    base : Base<Resource>,
    #[export]
    nombre : GString,
    #[export]
    descripcion : GString,
    #[export]
    ruta_de_planta_a_plantar : GString
}

#[godot_api]
pub impl IItem for SeedItemResource {
    fn get_name(&self) -> GString {
        self.nombre.clone()
    }
    fn get_description(&self) -> GString{
        self.descripcion.clone()
    }
    fn interact(&self, mut world : Gd<Node2D>, postion : Vector2) {
        let recurso_planta : Gd<PlantResource> = load(&self.ruta_de_planta_a_plantar);

        let mut new_planta = Planta::from_resource(recurso_planta);
        new_planta.set_position(postion);
        world.add_child(&new_planta);
    }
}