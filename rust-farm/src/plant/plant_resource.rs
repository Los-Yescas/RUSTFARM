use godot::{classes::SpriteFrames, prelude::*};

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct PlantResource{
    base: Base<Resource>,
    #[export]
    nombre: GString,
    #[export]
    crecimiento_minimo : i32,
    #[export]
    puntos_para_crecer : i32,
    #[export]
    #[init(val = None)]
    sprite : Option<Gd<SpriteFrames>>
}

#[godot_api]
impl PlantResource {
    
}