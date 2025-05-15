use godot::{
    classes::{Area2D, Texture2D, TileMapLayer},
    obj::NewGd,
    prelude::*,
};

use crate::{
    item::item_resource::IItem,
    plant::{Planta, plant_resource::PlantResource},
};

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct SeedItemResource {
    base: Base<Resource>,
    #[export]
    nombre: GString,
    #[export]
    descripcion: GString,
    #[export]
    max_stack: u16,
    #[export]
    ruta_de_planta_a_plantar: GString,
    #[export]
    textura: Option<Gd<Texture2D>>,
    #[export]
    precio: u16,
}

#[godot_dyn]
pub impl IItem for SeedItemResource {
    fn get_name(&self) -> GString {
        self.nombre.clone()
    }
    fn get_description(&self) -> GString {
        self.descripcion.clone()
    }
    fn get_sprite(&self) -> Gd<Texture2D> {
        self.textura.clone().unwrap_or(Texture2D::new_gd())
    }
    fn get_max_stack(&self) -> u16 {
        self.max_stack
    }
    fn interact(&self, mut world: Gd<Node2D>, postion: Vector2) {
        let recurso_planta: Gd<PlantResource> = load(&self.ruta_de_planta_a_plantar);

        let mut new_planta = Planta::from_resource(recurso_planta);
        new_planta.set_position(postion);
        world.add_child(&new_planta);
    }
    fn get_price(&self) -> u16 {
        self.precio
    }
}
impl SeedItemResource {
    /// Verifica si se puede plantar en una posición específica
    pub fn can_plant(&self, world: &Gd<Node2D>, position: Vector2) -> bool {
        // Corrección 1: Usar el world pasado como parámetro en lugar de self.base()
        let tilemap = world.get_node_as::<TileMapLayer>("Suelo");
        let tile_pos = tilemap.local_to_map(position);

        // Corrección 2: Asegurarse de retornar explícitamente el bool
        tilemap
            .get_cell_tile_data(tile_pos)
            .map(|data| data.get_custom_data("plantable").to::<bool>())
            .unwrap_or(false)
    }

    /// Versión extendida de interact que incluye todas las validaciones
    pub fn try_plant(&self, world: Gd<Node2D>, position: Vector2) -> Result<(), GString> {
        if !self.can_plant(&world, position) {
            return Err("No se puede plantar en esta posición".into());
        }

        // Si pasa todas las validaciones, ejecutar la plantación
        self.interact(world, position);
        Ok(())
    }
}
