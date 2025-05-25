use godot::{classes::{Button, CanvasLayer, InputEvent, Label, RandomNumberGenerator, Timer}, prelude::*};

use crate::{item::item_resource::IItem, player::Player};

use super::level_manager_interface::LevelManagerInterface;


#[derive(Clone)]
pub struct Pedido {
    pub requerimientos : Vec<Requerimiento>,
    pub recompensa : u16,
    pub time_for_order : f32,
    pub time_passed : f32
}   
#[derive(Clone)]
pub struct Requerimiento {
    pub item : DynGd<RefCounted, dyn IItem>,
    pub necesidad : u16
}


#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct LevelManager{
    base : Base<Node2D>,
    pedidos : Vec<Pedido>,
    #[export]
    items_a_pedir : Array<GString>,
    items_list : Vec<DynGd<RefCounted, dyn IItem>>,
    #[export]
    #[init (val = 8)]
    pedidos_maximos_actuales : u8,
    #[export]
    #[init(val=10.0)]
    tiempo_minimo_entre_nuevas_ordenes : f32,
    #[export]
    #[init(val=20.0)]
    tiempo_maximo_entre_nuevas_ordenes : f32,
    #[export]
    #[init(val=1)]
    minimo_a_pedir : u8,
    #[export]
    #[init(val=64)]
    maximo_a_pedir : u8,
    rng : Gd<RandomNumberGenerator>,
    #[export]
    next_level : Option<Gd<PackedScene>>,
    #[export]
    #[init(val = 20)]
    tiempo_de_nivel : u16,
    #[export]
    puntos_min_por_orden : u16,
    #[export]
    puntos_max_por_orden : u16,
    #[export]
    tiempo_minimo_de_orden : u16,
    #[export]
    tiempo_maximo_de_orden : u16
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

        let mut level_timer = self.base().get_node_as::<Timer>("LevelTimer");
        level_timer.connect("timeout", &self.base().callable("show_end_screen"));
        level_timer.set_wait_time(self.tiempo_de_nivel as f64);
        level_timer.start();

        let mut button = self.base().get_node_as::<Button>("WinScreen/NextLevel");
        button.connect("pressed", &self.base().callable("finish_level"));


        let mut main_menu_button = self.base().get_node_as::<Button>("WinScreen/MainMenu");
        let mut restart_button = self.base().get_node_as::<Button>("WinScreen/Restart");

        main_menu_button.connect("pressed", &self.base().callable("return_to_main_menu"));
        restart_button.connect("pressed", &self.base().callable("restart_level"));
    }

    fn process(&mut self, delta : f64){
        self.update_time_interface();

        self.update_time_of_orders(delta);
    }

    fn input(&mut self, event: Gd<InputEvent >,) {
        if event.is_action_pressed("exit"){
            self.show_end_screen();
        }
    }
}

#[godot_api]
pub impl LevelManager {
    fn reset_timer(&mut self){
        let mut timer = self.base().get_node_as::<Timer>("Timer");
        let tiempo_pedido = self.rng.randf_range(self.tiempo_minimo_entre_nuevas_ordenes, self.tiempo_maximo_entre_nuevas_ordenes) as f64;
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
        let recompensa = self.rng.randi_range(self.puntos_min_por_orden as i32, self.puntos_max_por_orden as i32) as u16;
        let tiempo = self.rng.randi_range(self.tiempo_minimo_de_orden as i32, self.tiempo_maximo_de_orden as i32) as f32;
        let mut pedido = Pedido { requerimientos: Vec::new(), recompensa, time_for_order : tiempo, time_passed : 0.0 };
        for _num in 0..num_a_pedir {
            let index_of_item = self.rng.randi_range(0, (self.items_list.len()-1) as i32) as usize;
            let item = &self.items_list[index_of_item];
            let asked_for = self.rng.randi_range(self.minimo_a_pedir.into(), self.maximo_a_pedir.into()) as u16;
            pedido.requerimientos.push(Requerimiento { item: item.clone(), necesidad: asked_for  });
        }
        self.pedidos.push(pedido);
        self.reset_timer();
        
        self.update_orders_interface();
    }

    fn update_orders_interface(&mut self){
        let mut interface = self.base().get_node_as::<LevelManagerInterface>("OrdersUI");
        interface.bind_mut().update_info(&self.pedidos);
    }
    fn update_time_interface(&mut self){
        let mut time_label = self.base().get_node_as::<Label>("TimeInterface/TimeLeft");
        let level_timer = self.base().get_node_as::<Timer>("LevelTimer");

        let time_left = level_timer.get_time_left().ceil();
        time_label.set_text(&format!("{time_left}"));
    }

    pub fn get_orders(&self) -> &Vec<Pedido>{
        &self.pedidos
    }

    #[func]
    fn check_order(&mut self, index : u16){
        let mut player = self.base().get_node_as::<Player>("../Player");
        let pedido = &self.pedidos[index as usize];
        if player.bind_mut().fullfill_order(pedido){
            player.bind_mut().sum_points(pedido.recompensa);
            self.remove_order(index as usize);
        }
    }


    pub fn remove_order(&mut self, index : usize){
        self.pedidos.remove(index);

        self.update_orders_interface();
    }

    #[func]
    fn show_end_screen(&mut self){
        let mut screen = self.base().get_node_as::<CanvasLayer>("WinScreen");
        screen.set_visible(true);

        let mut player = self.base().get_node_as::<Player>("../Player");
        let profits = player.bind().get_points_made();
        let orders = player.bind().get_orders_made();
        player.bind_mut().set_active(false);

        let mut orders_label = self.base().get_node_as::<Label>("WinScreen/Ordenes");
        let mut profits_label = self.base().get_node_as::<Label>("WinScreen/Ganancias");

        orders_label.set_text(&format!("{orders} ordenes"));
        profits_label.set_text(&format!("{profits} $ en ganancia"));
    }

    #[func]
    pub fn finish_level(&mut self){
        self.base().get_tree().unwrap().change_scene_to_file(&self.next_level.as_ref().expect("Sin siguiente nivel").get_path());
    }

    fn update_time_of_orders(&mut self, delta:f64){
        let mut index_for_removal : Vec<usize> = Vec::new();
        for (index, orden) in self.pedidos.iter_mut().enumerate(){
            orden.time_passed += delta as f32;
            if orden.time_passed >= orden.time_for_order {
                index_for_removal.push(index);
            }
        }
        for index in index_for_removal {
            self.remove_order(index);
            self.update_time_interface();
        }
    }

    #[func]
    fn return_to_main_menu(&mut self){
        self.base().get_tree().unwrap().change_scene_to_file("res://MenuPrincipal.tscn");
    }
    #[func]
    fn restart_level(&mut self){
        self.base().get_tree().unwrap().reload_current_scene();
    }
}