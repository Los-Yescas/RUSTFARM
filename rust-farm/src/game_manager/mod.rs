use godot::{classes::RandomNumberGenerator, prelude::*};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct GameManager{
    base : Base<Object>,
    #[var]
    game_active : bool,
    rng : Gd<RandomNumberGenerator>
}
#[godot_api]
impl IObject for GameManager{
    fn init(base: Base <Object>) -> Self {
        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();
        Self { base, game_active: true, rng }
    }
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn tick(&mut self){
        let randon_number = self.rng.randi();
        self.base_mut().emit_signal("tick", &[randon_number.to_variant()]);
    }
}