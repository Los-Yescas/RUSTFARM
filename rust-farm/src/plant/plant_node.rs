use godot::{classes::{AnimatedSprite2D, TileMapLayer}, prelude::*};
use super::plant_resource::PlantResource;

use crate::{game_manager::GameManager, item::item_resource::IItem, world_interactables::IWorldPickable};

#[derive(PartialEq)]
pub enum FasesPlantas {
    Bebe, 
    Infancia,
    Adolescencia,
    Juventud,
    Madura
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Planta{
    base : Base<Node2D>,
    #[init(val = 0)]
    grow_points : u32,
    #[export]
    plant_data_path : GString,
    plant_data : Gd<PlantResource>,
    fruit_data : Option<DynGd<RefCounted, dyn IItem>>,
    #[init(val = FasesPlantas::Bebe)]
    fase_actual : FasesPlantas,
    fertilidad : f32
}

#[godot_api]
impl INode2D for Planta {

    fn ready(&mut self,) {
        let grow_callable = self.base_mut().callable("grow_tick");
        godot::classes::Engine
        ::singleton()
        .get_singleton("GameManager")
        .expect("Game Manager no existe")
        .cast::<GameManager>().bind_mut().base_mut().connect("tick", &grow_callable);

        self.plant_data = load(&self.plant_data_path);
        self.fruit_data = Some(load::<Resource>(&self.plant_data.bind().get_plant_fruit_data_path()).to_variant().to());

        let sprite = self.plant_data.bind().get_sprite().expect("No hay animacion");

        let mut animated_sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        animated_sprite.set_sprite_frames(&sprite);

        let tiles = self.base().get_node_as::<TileMapLayer>("../Suelo");
        let tile_position = tiles.local_to_map(self.base().get_global_position());
        let tile_data = tiles.get_cell_tile_data(tile_position);

        self.fertilidad = tile_data.unwrap().get_custom_data("fertility").to();
    }
}

#[godot_api]
impl Planta {
    #[func]
    pub fn grow_tick(&mut self, random_number : u32 ){
        let plant_data = self.plant_data.bind();
        let min_cre = plant_data.get_crecimiento_minimo();
        let max_cre = plant_data.get_crecimiento_maximo();
        let punt_pa_cre = plant_data.get_puntos_para_crecer();

        self.grow_points += ((min_cre + random_number%(max_cre-min_cre)) as f32 * self.fertilidad).ceil() as u32;
        drop(plant_data);

        if self.grow_points >= punt_pa_cre {
            let mut planta_sprite = self.base_mut().get_node_as::<AnimatedSprite2D>("./AnimatedSprite2D");
            self.fase_actual = match self.fase_actual {
                FasesPlantas::Bebe => {
                    planta_sprite.set_frame(1);
                    FasesPlantas::Infancia
                },
                FasesPlantas::Infancia => {
                    planta_sprite.set_frame(2);
                    FasesPlantas::Adolescencia
                },
                FasesPlantas::Adolescencia => {
                    planta_sprite.set_frame(3);
                    FasesPlantas::Juventud
                },
                _ => {
                    planta_sprite.set_frame(4);
                    FasesPlantas::Madura
                }
            };
            self.grow_points = 0;
        }
    }
    #[func]
    pub fn from_resource(plant_resource : Gd<PlantResource>) -> Gd<Planta>{
        let plant_scene = load::<PackedScene>("res://Plantas/PlantNode.tscn");
        let mut new_plant = plant_scene.instantiate_as::<Planta>();
        let mut plant = new_plant.bind_mut();
        plant.set_plant_data_path(plant_resource.get_path());
        drop(plant);

        new_plant
    }
    #[func]
    pub fn harvest(&mut self) -> Option<DynGd<RefCounted, dyn IItem>>{
        if self.fase_actual == FasesPlantas::Madura {
            return Some(self.fruit_data.to_variant().to());
        }
        None
    }
}

#[godot_dyn]
impl IWorldPickable for Planta {
    fn pick(&self) -> Option<&DynGd<RefCounted, dyn IItem>> {
        if self.fase_actual == FasesPlantas::Madura {
            return self.fruit_data.as_ref();
        }
        return None;
    }
    fn has_been_picked(&mut self) {
        self.base_mut().queue_free();
    }
}