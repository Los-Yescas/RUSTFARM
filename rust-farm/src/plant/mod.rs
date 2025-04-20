use godot::{classes::{AnimatedSprite2D, Area2D, CollisionShape2D, RectangleShape2D}, prelude::*};
use plant_resource::PlantResource;

use crate::game_manager::GameManager;

pub mod plant_resource;


enum FasesPlantas {
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
    #[init(val = FasesPlantas::Bebe)]
    fase_actual : FasesPlantas
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

        let sprite = self.plant_data.bind().get_sprite().expect("No hay animacion");

        let mut animated_sprite = AnimatedSprite2D::new_alloc();
        animated_sprite.set_sprite_frames(&sprite);
        animated_sprite.set_name("AnimatedSprite2D");
        self.base_mut().add_child(&animated_sprite);

        let mut area_collision = Area2D::new_alloc();
        let mut collider = CollisionShape2D::new_alloc();
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2 { x: 100.0, y: 100.0 });
        collider.set_shape(&shape);
        area_collision.add_child(&collider);
        self.base_mut().add_child(&area_collision);
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

        self.grow_points += min_cre + random_number%(max_cre-min_cre);
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
    pub fn from_resource(plant_resource : Gd<PlantResource>) -> Gd<Self>{
        Gd::from_init_fn(|base| {
            Self {
                base,
                plant_data_path : plant_resource.get_path(),
                plant_data : plant_resource,
                fase_actual : FasesPlantas::Bebe,
                grow_points : 0,
            }
        })
    }
}