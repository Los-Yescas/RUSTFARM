use godot::{classes::{RandomNumberGenerator, Timer}, prelude::*};

use crate::{interfaces::level_manager::level_manager_interface::LevelManagerInterface, item::item_resource::IItem};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct LevelManager{
    base : Base<Node2D>,
    pedidos : Vec<Vec<(DynGd<RefCounted, dyn IItem>, u16)>>,
    #[export]
    items_a_pedir : Array<GString>,
    items_list : Vec<DynGd<RefCounted, dyn IItem>>,
    #[export]
    #[init (val = 8)]
    pedidos_maximos_actuales : u8,
    #[export]
    #[init(val=10.0)]
    tiempo_minimo : f32,
    #[export]
    #[init(val=20.0)]
    tiempo_maximo : f32,
    #[export]
    #[init(val=1)]
    minimo_a_pedir : u8,
    #[export]
    #[init(val=64)]
    maximo_a_pedir : u8,
    rng : Gd<RandomNumberGenerator>
}

#[godot_api]
impl INode2D for LevelManager {
    fn ready(&mut self,) {
        for ruta in self.items_a_pedir.iter_shared() {
            if let Ok(recurso) = try_load::<Resource>(&ruta) {
                if let Ok(item) = recurso.to_variant().try_to() {
                    self.items_list.push(item);
                }else{
                    godot_error!("No se le paso un item a LevelManager");
                    return;
                }
            }else {
                godot_error!("No se le paso un Recurso a LevelManager");
                return;
            }
        }
        self.rng = RandomNumberGenerator::new_gd();
        self.rng.randomize();

        let mut timer = self.base().get_node_as::<Timer>("Timer");
        timer.connect("timeout", &self.base().callable("generate_new_order"));
        self.reset_timer();
    }
}

#[godot_api]
pub impl LevelManager {
    fn reset_timer(&mut self){
        let mut timer = self.base().get_node_as::<Timer>("Timer");
        let tiempo_pedido = self.rng.randf_range(self.tiempo_minimo, self.tiempo_maximo) as f64;
        timer.set_wait_time(tiempo_pedido);
        timer.start();
    }

    #[func]
    fn generate_new_order(&mut self){
        if self.pedidos.len() >= self.pedidos_maximos_actuales.into() {
            self.reset_timer();
            return;
        }
        let num_a_pedir = self.rng.randi_range(1, 3);
        let mut pedido = Vec::new();
        for _num in 0..num_a_pedir {
            let index_of_item = self.rng.randi_range(0, (self.items_list.len()-1) as i32) as usize;
            let item = &self.items_list[index_of_item];
            let asked_for = self.rng.randi_range(self.minimo_a_pedir.into(), self.maximo_a_pedir.into());
            pedido.push((item.clone(), asked_for as u16));
        }
        self.pedidos.push(pedido);
        self.reset_timer();
        
        self.update_interface();
    }

    fn update_interface(&mut self){
        let mut interface = self.base().get_node_as::<LevelManagerInterface>("UI");
        interface.bind_mut().update_info(&self.pedidos);
    }

    pub fn get_orders(&self) -> &Vec<Vec<(DynGd<RefCounted, dyn IItem>, u16)>>{
        &self.pedidos
    }
}