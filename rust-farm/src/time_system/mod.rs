use godot::prelude::*;

use crate::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=Node)]
struct TimeSystem{
    base : Base<Node>,
    #[var]
    game_active : bool,
    time_passed : f64,
    #[export]
    time_for_tick : f64,
    game_manager :  Gd<GameManager>
}


#[godot_api]
impl INode for TimeSystem {
    fn init(base: Base<Node>) -> Self {
        let manager   = godot::classes::Engine
        ::singleton()
        .get_singleton("GameManager")
        .expect("Game Manager no existe")
        .cast::<GameManager>();
        Self { base, game_active: true, time_passed: 0.0, time_for_tick: 0.0, game_manager: manager}
    }
    fn ready(&mut self,) {
        self.game_manager.bind_mut().base_mut().add_user_signal("tick");
    }
    fn process(&mut self, delta: f64,) {
        self.time_passed += delta;
        if self.time_passed >= self.time_for_tick {
            self.game_manager.bind_mut().tick();
            self.time_passed = 0.0;
        }
    }
}
