use godot::prelude::*;

use crate::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Planta{
    base : Base<Node2D>,
    #[export]
    name : GString,
    #[export]
    points_for_growing : i32,
    grow_points : i32,
}

#[godot_api]
impl INode2D for Planta {
    fn init(base: Base<Node2D>) -> Self {
        Self { base, name: "".into(), points_for_growing: 0, grow_points: 0 }
    }

    fn ready(&mut self,) {
        let grow_callable = self.base_mut().callable("grow_tick");
        godot::classes::Engine
        ::singleton()
        .get_singleton("GameManager")
        .expect("Game Manager no existe")
        .cast::<GameManager>().bind_mut().base_mut().connect("tick", &grow_callable);
    }
}

#[godot_api]
impl Planta {
    #[func]
    pub fn grow_tick(&mut self){
        self.grow_points += 100;
        if self.grow_points > self.points_for_growing{
            godot_print!("siguiente fase, invecil");
            self.grow_points = 0;
        }
        let name = self.base().get_name();
        godot_print!("Crezco, IMbecil {}", name);
    }
}