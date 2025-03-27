use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct GameManager{
    base : Base<Object>,
    #[var]
    game_active : bool,
}
#[godot_api]
impl IObject for GameManager{
    fn init(base: Base <Object>) -> Self {
        Self { base, game_active: true }
    }
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn tick(&mut self){
        self.base_mut().emit_signal("tick", &[]);
    }
}