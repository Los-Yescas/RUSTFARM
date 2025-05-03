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

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    #[export]
    speed: f32,
    input : Gd<Input>,
    is_moving : bool,
    target_position : Vector2,
    inventory : Vec<(DynGd<RefCounted, dyn IItem>, u16)>,
    item_actual : usize,
    #[export]
    inventario_maximo : u16,
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
            target_position: Vector2::ZERO,
            inventory : Vec::new(),
            item_actual : 0,
            inventario_maximo : 8
        }
    }
    
    fn process(&mut self, _delta: f64,) {        
        if self.is_moving{
            return;
        }
        self.interaction_system();

        self.player_movement();
    }
    fn physics_process(&mut self, delta: f64) {
        if self.is_moving {
            self.player_moving(delta);
        }
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
        self.interaction_system_inputs(event);
    }
}

#[godot_api]
impl Player {
    #[func]
    fn player_movement(&mut self){
        if self.input.is_action_pressed("up"){
            self.set_movement_target(Vector2i::UP);
        }else if self.input.is_action_pressed("down") {
            self.set_movement_target(Vector2i::DOWN)
        }else if self.input.is_action_pressed("right") {
            self.set_movement_target(Vector2i::RIGHT)
        }else if self.input.is_action_pressed("left") {
            self.set_movement_target(Vector2i::LEFT)
        }
    }
    #[func]
    fn set_movement_target(&mut self, direction : Vector2i){
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
            self.base_mut().get_node_as::<Node2D>("./InteractZone").set_rotation(direction.cast_float().angle());
        }
    }
    #[func]
    fn player_moving(&mut self, delta : f64){
        let global_position = self.base().get_global_position();
        if  global_position == self.target_position {
            self.is_moving = false;
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
                    let res = self.add_item_to_inventory(resource);
                    match res {
                        Err(error) => godot_print!("{error}"),
                        Ok(_exito) => item.queue_free(),
                    }
                }else if let Ok(mut planta) = object.clone().try_cast::<Planta>(){
                    if let Some(resource) = planta.clone().bind_mut().harvest(){
                        let res = self.add_item_to_inventory(resource);
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
    fn interaction_system_inputs(&mut self, event: Gd<InputEvent>){
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
    fn add_item_to_inventory(&mut self, item : DynGd<RefCounted, dyn IItem>) -> Result<GString, GString>{
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
}

