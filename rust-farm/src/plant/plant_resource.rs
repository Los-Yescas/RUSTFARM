use godot::{classes::SpriteFrames, prelude::*};

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct PlantResource{
    base: Base<Resource>,
    #[export]
    nombre: GString,
    #[export]
    crecimiento_minimo : u32,
    #[export]
    crecimiento_maximo : u32,
    #[export]
    puntos_para_crecer : u32,
    #[export]
    #[init(val = None)]
    sprite : Option<Gd<SpriteFrames>>,
    #[export]
    plant_fruit_data_path : GString,
}
