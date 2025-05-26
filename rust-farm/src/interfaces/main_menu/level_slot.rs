use godot::{classes::{Button, IButton}, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct LevelSlot{
    base : Base<Button>,
    #[var]
    level_to_load : GString
}

#[godot_api]
impl IButton for LevelSlot {
    fn ready(&mut self,) {
        
    }
}

#[godot_api]
impl LevelSlot{
    pub fn new_level_slot(level_to_load : GString, index : u16) -> Gd<LevelSlot>{
        let level_slot = load::<PackedScene>("res://Interfaces/LevelSelection/level_slot.tscn");
        let mut new_slot = level_slot.instantiate_as::<LevelSlot>();
        new_slot.set_text(&format!("{index}"));
        new_slot.bind_mut().set_level_to_load(level_to_load);

        let load_level_callable = new_slot.callable("load_level");
        
        new_slot.connect("pressed", &load_level_callable);

        new_slot
    }

    #[func]
    fn load_level(&mut self){
        self.base().get_tree().unwrap().change_scene_to_file(&self.level_to_load);
    }
}