use godot::{classes::{AudioStream, Texture2D, TileMapLayer}, obj::NewGd, prelude::*};

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
    fn pick(&self) -> DynGd<RefCounted, dyn IItem>{
        self.to_gd().to_variant().to()
    }
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
    fn interact(&mut self, mut world : Gd<Node2D>, position : Vector2, objeto : Option<Gd<Node2D>>) -> Result<bool, GString> {
        if objeto.is_some(){
            return Err("Algo en frente".into());
        }

        let tiles = world.get_node_as::<TileMapLayer>("Suelo");
        let tile_position = tiles.local_to_map(position);
        let tile_data = tiles.get_cell_tile_data(tile_position);

        let can_grow : bool = tile_data.unwrap().get_custom_data("plantable").to();

        if !can_grow{
            godot_print!("No puede plantar aca");
            return Err("No puede plantar aca".into());
        }

        let recurso_planta : Gd<PlantResource> = load(&self.ruta_de_planta_a_plantar);

        let mut new_planta = Planta::from_resource(recurso_planta);
        new_planta.set_position(position);
        world.add_child(&new_planta);
        Ok(true)
    }

    fn get_precio(&self) -> u16 {
        self.precio
    }

    fn get_interact_sound(&self) -> Option<Gd<AudioStream>> {
        Some(load::<AudioStream>("res://Sounds/Sound/items/plant.mp3"))
    } 
}