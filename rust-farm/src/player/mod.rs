use godot::classes::Area2D;
use godot::classes::InputEvent;
use godot::classes::Marker2D;
use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;

use crate::item::item_node::Item;
use crate::item::item_resource::plant_items::seed_item::SeedItemResource;
use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    #[export]
    speed: f32,
    input : Gd<Input>,
    is_moving : bool,
    target_position : Vector2,
    inventory : Vec<(Gd<RefCounted>, u16)>,
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
        
        if self.input.is_action_just_pressed("inventory"){
            godot_print!("{:#?}", self.inventory);
        }

        if self.is_moving{
            return;
        }
        self.interaction_system();

        if self.input.is_action_pressed("up"){
            self.move_to(Vector2i::UP);
        }else if self.input.is_action_pressed("down") {
            self.move_to(Vector2i::DOWN)
        }else if self.input.is_action_pressed("right") {
            self.move_to(Vector2i::RIGHT)
        }else if self.input.is_action_pressed("left") {
            self.move_to(Vector2i::LEFT)
        }
    }
    fn physics_process(&mut self, delta: f64) {
        if self.is_moving {
            let global_position = self.base().get_global_position();
            if  global_position == self.target_position {
                self.is_moving = false;
                return;
            }
            let new_position = global_position.move_toward(self.target_position, self.speed * delta as f32);
            self.base_mut().set_global_position(new_position);
        }
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
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
        }
    }
}

#[godot_api]
impl Player {
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
            self.base_mut().get_node_as::<Node2D>("./InteractZone").set_rotation(direction.cast_float().angle());
        }
    }

    fn interaction_system(&mut self){
        if self.input.is_action_just_pressed("pick"){
            if let Some(object) = self.check_for_item() {
                if let Ok(item) = object.try_cast::<Item>(){
                    self.add_item_to_inventory(item);
                }
            }
        }else if self.input.is_action_just_pressed("interact") {
            self.interact();
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
    fn add_item_to_inventory(&mut self, mut item : Gd<Item>){
        let item_resource = item.bind().get_item_resource().expect("Sin recurso");
        let index = self.inventory.iter().position(|(nodo,_)| *nodo == item_resource);
        match index {
            None => {
                if self.inventory.len() < self.inventario_maximo.into() {
                    self.inventory.push((item_resource, 1));
                    item.queue_free();
                }  else {
                    godot_print!("No puede cogerlo espacio maximo")
                }
            },
            Some(index) => {
                let tupla = &mut self.inventory[index];
                let resource : DynGd<RefCounted, dyn IItem> = item_resource.to_variant().to();
                if tupla.1 < resource.dyn_bind().get_max_stack(){
                    tupla.1 += 1;
                    item.queue_free();
                } else {
                    godot_print!("No puede cogerlo espacio maximo")
                }
            } 
        }
    }

    fn interact(&mut self){
        if let  Some(mut tupla_inventario) = self.inventory.get(self.item_actual){
            let  (item, stack) = &mut tupla_inventario;

            let item: DynGd<RefCounted, dyn IItem> = item.to_variant().try_to().expect("No es Item");
            if let Ok(planta) = item.try_cast::<SeedItemResource>(){
                if self.check_for_item() == None {
                    let world = self.base().get_parent().unwrap().cast();
                    let position = self.base().get_node_as::<Marker2D>("./InteractZone/SpawnerPos").get_global_position();
                    planta.bind().interact(world, position);

                    if *stack == 1{
                        self.inventory.remove(self.item_actual);
                    }else{
                        self.inventory[self.item_actual].1 -= 1;
                    }
                }
            }
        }
    }

    pub fn get_equiped_item(&self) -> Option<&(Gd<RefCounted>, u16)>{
        self.inventory.get(self.item_actual)
    }
}

