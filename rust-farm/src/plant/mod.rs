use godot::prelude::*;
use plant_resource::PlantResource;

use crate::game_manager::GameManager;

pub mod plant_resource;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Planta{
    base : Base<Node2D>,
    #[init(val = 0)]
    grow_points : i32,
    #[export]
    plant_data_path : GString,
    plant_data : Gd<PlantResource>
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

        self.plant_data = load(&self.plant_data_path)
    }
}

#[godot_api]
impl Planta {
    #[func]
    pub fn grow_tick(&mut self){
        let plant_data = self.plant_data.bind();
        self.grow_points += 100;
        if self.grow_points > plant_data.get_puntos_para_crecer(){
            godot_print!("siguiente fase, invecil");
            self.grow_points = 0;
        }
        let name = plant_data.get_nombre();
        godot_print!("Crezco, IMbecil {}", name);
    }
}