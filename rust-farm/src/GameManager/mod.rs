use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct GameManager{
    base : Base<Node>,
    #[init(val = true)]
    gameActive : bool,
    #[init(val = 0.0)]
    time_passed : f64,
    #[export]
    time_for_tick : i32
}

#[godot_api]
impl GameManager {
    fn pause_game(&mut self){
        self.gameActive = !self.gameActive
    }
    fn tick(&mut self){
        self.time_passed = 0.0;
        godot_print!("Paso un tick, malnacido")
    }
}
#[godot_api]
impl INode for GameManager{
    fn process(&mut self, delta: f64,) {
        self.time_passed += delta;
        if self.time_passed >= self.time_for_tick.into() {
            self.tick();
        }
    }
}