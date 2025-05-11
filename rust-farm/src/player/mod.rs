use godot::classes::Area2D;
use godot::classes::InputEvent;
use godot::classes::Marker2D;
use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;

use crate::item::item_node::Item;
use crate::item::item_resource::IItem;
use crate::plant::plant_node::Planta;


pub mod player_interface;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    #[export]
    speed: f32,
    input : Gd<Input>,
    can_move : bool,
    is_moving : bool,
    target_position : Vector2,
    inventory : Vec<(DynGd<RefCounted, dyn IItem>, u16)>,
    item_actual : usize,
    #[export]
    inventario_maximo : u16,
    #[export]
    puntos : u16,
    direction : Vector2i,
    base: Base<Node2D>
}
#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        
        Self {
            speed: 500.0,
            base,
            input : Input::singleton(),
            is_moving: false,
            can_move : true,
            target_position: Vector2::ZERO,
            inventory : Vec::new(),
            item_actual : 0,
            inventario_maximo : 8,
            direction : Vector2i::RIGHT,
            puntos : 0,
        }
    }
    
    fn process(&mut self, _delta: f64,) {        
        if self.is_moving{
            return;
        }
        self.interaction_system();
    }
    fn physics_process(&mut self, delta: f64) {
        if self.is_moving && self.can_move{
            self.player_moving(delta);
        }
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
        self.interaction_system_inputs(&event);
        self.player_movement_input();
    }
}

#[godot_api]
impl Player {
    #[func]
    fn player_movement_input(&mut self){
        let mut direction = Vector2i::ZERO;
        direction.x = self.input.get_axis("left", "right").round() as i32;
        direction.y = self.input.get_axis("up", "down").round() as i32;

        if direction == Vector2i::ZERO || self.is_moving || !self.can_move{
            return;
        }
        if direction.x != 0 {
            direction.y = 0;
        }
        if direction == self.direction {
            self.move_to(direction);
        }else{
            self.face_to(direction);
            let mut timer = self.base().get_tree().unwrap().create_timer(0.15).unwrap();
            timer.connect("timeout", &self.base().callable("end_facing"));
        }
    }

    fn face_to(&mut self, direction : Vector2i ){
        self.can_move = false;
        self.base_mut()
            .get_node_as::<Node2D>("./InteractZone")
            .set_rotation(direction.cast_float().angle());
        self.direction = direction;
    }
    #[func]
    fn end_facing(&mut self){
        self.can_move = true;
        self.player_movement_input();
    }
    #[func]
    fn move_to(&mut self, direction : Vector2i){
        let map = self.base().get_node_as::<TileMapLayer>("../Suelo");
        let current_tile = map.local_to_map(self.base().get_global_position());
        let target_tile = Vector2i{
            x: current_tile.x + direction.x,
            y: current_tile.y + direction.y
        };

        let tile_data = map.get_cell_tile_data(target_tile);
        let walkable : bool = match tile_data {
            None => return,
            Some(tile ) => tile.get_custom_data("walkable").to::<bool>()
        };

        if walkable {
            self.is_moving = true;
            self.target_position = map.map_to_local(target_tile);
        }
    }
    #[func]
    fn player_moving(&mut self, delta : f64){
        let global_position = self.base().get_global_position();
        if  global_position == self.target_position {
            self.is_moving = false;
            self.player_movement_input();
            return;
        }
        let new_position = global_position.move_toward(self.target_position, self.speed * delta as f32);
        self.base_mut().set_global_position(new_position);
    }
    
    fn interaction_system(&mut self){
        if self.input.is_action_just_pressed("pick"){
            if let Some(object) = self.check_for_item() {
                if let Ok(mut item) = object.clone().try_cast::<Item>(){
                    let resource: DynGd<RefCounted, dyn IItem> = item.bind().get_item_resource().to_variant().to();
                    let res = self.add_item_to_inventory(&resource);
                    match res {
                        Err(error) => godot_print!("{error}"),
                        Ok(_exito) => item.queue_free(),
                    }
                }else if let Ok(mut planta) = object.clone().try_cast::<Planta>(){
                    if let Some(resource) = planta.clone().bind_mut().harvest(){
                        let res = self.add_item_to_inventory(&resource);
                        match res {
                            Err(error) => godot_print!("{error}"),
                            Ok(_exito) => planta.queue_free(),
                        }
                    }
                }
            }
        }else if self.input.is_action_just_pressed("interact") {
            self.interact();
        }
    }
    fn interaction_system_inputs(&mut self, event: &Gd<InputEvent>){
        if event.is_action_pressed("inventory+"){
            let inventario_maximo : usize = self.inventario_maximo.into();
            self.item_actual = (self.item_actual  + 1)%inventario_maximo;
        }else if event.is_action_pressed("inventory-") {
            let inventario_maximo : usize = self.inventario_maximo.into();
            if self.item_actual == 0 {
                self.item_actual = inventario_maximo;
            }else {
                self.item_actual = (self.item_actual  - 1)%inventario_maximo;
            }
        }else if event.is_action_pressed("inventory"){
            godot_print!("{:#?}", self.inventory);
        }
    }
    fn check_for_item(&self) -> Option<Gd<Node2D>>{
        let collider: Gd<Area2D> = self.base().get_node_as("./InteractZone/Area2D");
        let objects_in_area = collider.get_overlapping_areas();
        let object = objects_in_area.get(0);
        match object {
            None => None,
            Some(area2d) => Some(area2d.get_parent().expect("Sin padre").cast())
        }
    }
    pub fn add_item_to_inventory(&mut self, item : &DynGd<RefCounted, dyn IItem>) -> Result<GString, GString>{
        let item = item.clone();
        let index = self.inventory.iter().position(|(nodo,_)| *nodo == item);
        match index {
            None => {
                if self.inventory.len() < self.inventario_maximo.into() {
                    self.inventory.push((item, 1));
                    return Ok("Item añadido".into());
                }  else {
                    return Err("Inventory size not enough".into());
                }
            },
            Some(index) => {
                let tupla = &mut self.inventory[index];
                if tupla.1 < item.dyn_bind().get_max_stack(){
                    tupla.1 += 1;
                    return Ok("Item añadido".into());
                } else {
                    return Err("Max stack reached!!".into());
                }
            } 
        }
    }
    pub fn rest_item_to_inventory(&mut self, item : &DynGd<RefCounted, dyn IItem>) {
        let item = item.clone();
        if let Some(index) = self.inventory.iter().position(|(nodo,_)| *nodo == item){
            let tupla = &mut self.inventory[index];
            tupla.1 -= 1;
            if tupla.1 <= 0 {
                self.inventory.remove(index);
            }
        }
    }
    pub fn is_inventory_full(&self) -> bool {
        self.inventory.len() >= self.inventario_maximo.into()
    }
    pub fn sum_points(&mut self, points : u16){
        self.puntos += points;
    }
    pub fn rest_points(&mut self, points : u16){
        self.puntos -= points;
    }

    fn interact(&mut self){
        if let  Some(mut tupla_inventario) = self.inventory.get(self.item_actual){
            let  (item, stack) = &mut tupla_inventario;

            if self.check_for_item() == None{
                let world = self.base().get_parent().unwrap().cast();
                let position = self.base().get_node_as::<Marker2D>("./InteractZone/SpawnerPos").get_global_position();
                item.dyn_bind().interact(world, position);

                if *stack == 1{
                    self.inventory.remove(self.item_actual);
                }else{
                    self.inventory[self.item_actual].1 -= 1;
                }
            }
        }
    }

    pub fn get_equiped_item(&self) -> Option<&(DynGd<RefCounted, dyn IItem>, u16)>{
        self.inventory.get(self.item_actual)
    }
    pub fn get_inventory(&self) -> Vec<(DynGd<RefCounted, dyn IItem>, u16)> {
        self.inventory.clone()
    }
}

