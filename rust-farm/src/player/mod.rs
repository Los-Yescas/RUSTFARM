use godot::classes::Area2D;
use godot::classes::InputEvent;
use godot::classes::Marker2D;
use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;

use crate::item::item_resource::IItem;
use crate::level_manager::level_manager_node::Pedido;
use crate::world_interactables::IWorldInteractable;
use crate::world_interactables::IWorldPickable;


pub mod player_interface;

#[derive(GodotClass)]
#[class(base=Node2D, init)]
pub struct Player {
    base: Base<Node2D>,
    #[export]
    #[init(val = 500.0)]
    speed: f32,
    #[init(val = true)]
    can_move : bool,
    #[init(val = false)]
    is_moving : bool,
    #[init(val = true)]
    #[var]
    active : bool,
    #[init(val = Vector2::ZERO)]
    target_position : Vector2,
    inventory : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>,
    item_actual : usize,
    #[export]
    #[init(val = 8)]
    inventario_maximo : u16,
    #[export]
    #[init(val = 0)]
    puntos : u16,
    #[init(val = Vector2i::RIGHT)]
    direction : Vector2i,
    #[var]
    points_made : u16,
    #[var]
    orders_made : u16
}
#[godot_api]
impl INode2D for Player {

    fn ready(&mut self,) {
        self.inventory = vec![None; self.inventario_maximo as usize];
    }
    fn physics_process(&mut self, delta: f64) {
        if !self.active{
            return;
        }
        if self.is_moving && self.can_move{
            self.player_moving(delta);
        }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>,) {
        if !self.active{
            return;
        }
        self.player_movement_input();
        self.interaction_system_inputs(&event);
    }
}


#[godot_api]
impl Player {
    fn interaction_system_inputs(&mut self, event: &Gd<InputEvent>){

        if event.is_action_pressed("inventory+"){
            self.select_item(self.item_actual + 1);
        }else if event.is_action_pressed("inventory-") {
            self.select_item((self.item_actual + self.inventario_maximo as usize) - 1);
        }else if event.is_action_pressed("inventory"){
            godot_print!("{:#?}", self.inventory);
        }else if event.is_action_pressed("pick"){
            self.pick_item();
        }

        if event.is_action_pressed("interact") && !self.is_moving{
            self.interact();   
        }
    }
    pub fn select_item(&mut self, index : usize){
        self.item_actual = (index as u16 % self.inventario_maximo) as usize;
        self.base_mut().emit_signal("inventory_updated", &[]);
    }
    fn pick_item(&mut self){
        if let Some(object) = self.check_for_item() {
            let variant = object.to_variant();
            if let Ok(mut pickable) = variant.try_to::<DynGd<Node2D, dyn IWorldPickable>>(){
                let mut pickable = pickable.dyn_bind_mut();
                if let Some(item) = pickable.pick(){
                    let item = item.dyn_bind().pick();
                    match self.add_item_to_inventory(&item) {
                        Err(error) => godot_print!("{error}"),
                        Ok(_exito) => {
                            self.base_mut().emit_signal("inventory_updated", &[]);
                            pickable.has_been_picked()
                        }
                    }
                }
            } else if let Ok(mut interactable) = variant.try_to::<DynGd<Node2D, dyn IWorldInteractable>>(){
                interactable.dyn_bind_mut().show_menu(self.inventory.clone(), self.puntos);
            }
        }
    }
    #[func]
    fn player_movement_input(&mut self){
        let mut direction = Vector2i::ZERO;
        let input = Input::singleton();
        direction.x = input.get_axis("left", "right").round() as i32;
        direction.y = input.get_axis("up", "down").round() as i32;

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

        let indexes = self.available_slots_with_item(&item);

        if let Some(index) = indexes.get(0) {
            self.add_one_item_to_slot(*index);
            self.base_mut().emit_signal("inventory_updated", &[]);
            return Ok("Item añadido".into());
        }else {
            if let Some(empty_slot_index) = self.empty_slot_index() {
                self.inventory[empty_slot_index] = Some((item, 1));
                self.base_mut().emit_signal("inventory_updated", &[]);
                return Ok("Item añadido".into());
            }
        }
        Err("Can't add to inventory".into())
    }
    fn available_slots_with_item(&mut self, item : &DynGd<RefCounted, dyn IItem>) ->  Vec<usize>{
        self.inventory.iter().enumerate()
            .filter(|&(_, slot)| {
                if *slot == None{
                    return false;
                }

                let slot = slot.as_ref().unwrap();
                if slot.0 == *item {
                    if slot.1 < item.dyn_bind().get_max_stack() {
                        return true;
                    }
                }
                return false;
            }).map(|(index, _)| index)
            .collect()
    }
    fn empty_slot_index(&self) -> Option<usize>{
        self.inventory.iter().position(|slot| *slot==None)
    }
    fn add_one_item_to_slot(&mut self, index : usize){
        self.base_mut().emit_signal("inventory_updated", &[]);
        self.inventory[index].as_mut().unwrap().1 += 1;
    }
    pub fn rest_item_to_inventory(&mut self, index : usize, number : u16) {
        if let Some(slot) = &mut self.inventory[index]{
            if number > slot.1 {
                godot_error!("No puede restar mas de lo que se tiene");
                return;
            }
            slot.1 -= number;
            if slot.1 <= 0 {
                self.inventory[index] = None;
            }
            self.base_mut().emit_signal("inventory_updated", &[]);
        }else{
            godot_error!("Tratando de restar a un slot vacio");
        }
    }

    pub fn is_inventory_full(&self) -> bool {
        self.inventory.iter().position(|slot| slot.is_none()).is_none()
    }
    pub fn sum_points(&mut self, points : u16){
        self.points_made += points;
        self.puntos += points;
    }
    pub fn rest_points(&mut self, points : u16){
        self.puntos -= points;
    }

    fn interact(&mut self){

        let item_in_front = self.check_for_item();
        let world = self.base().get_parent().unwrap().cast();
        let position = self.base().get_node_as::<Marker2D>("./InteractZone/SpawnerPos").get_global_position();

        if let  Some(tupla_inventario) = &mut self.inventory[self.item_actual]{
            let  (item, _) = tupla_inventario;
   
            let consume = item.dyn_bind_mut().interact(world, position, item_in_front);

            if consume {
                self.rest_item_to_inventory(self.item_actual, 1);
                self.base_mut().emit_signal("inventory_updated", &[]);
            }
        }
    }

    pub fn fullfill_order(&mut self, pedido : &Pedido) -> bool{
        let mut usos_inventario : Vec<u16> = vec![0; self.inventario_maximo as usize];
        let mut items_satisfechos : Vec<(usize, u16)> = pedido.requerimientos.iter().enumerate().map(
            |(index, _)| (index, 0)
        ).collect();
        for (inventory_index, inventory_slot) in self.inventory.iter().enumerate() {

            if inventory_slot.is_none(){
                continue;
            }
            let inventory_slot = inventory_slot.as_ref().unwrap();

            let indexes_for_asked : Vec<usize> = pedido.requerimientos.iter().enumerate().filter(
                |(index, requerimeinto)| {
                    let item = &requerimeinto.item;
                    let neccesity = requerimeinto.necesidad;

                    inventory_slot.0.dyn_bind().get_name() == item.dyn_bind().get_name()
                    &&
                    items_satisfechos[*index].1 < neccesity
                }
                ).map(|(index, _)|{
                    index
                }).collect();

            

            if indexes_for_asked.is_empty() {
                continue;
            }
            for index_asked in indexes_for_asked {
                let neccesity = pedido.requerimientos[index_asked].necesidad;

                let available_items = inventory_slot.1-usos_inventario[inventory_index];
                let needed_items = neccesity - items_satisfechos[index_asked].1;
                if needed_items <= available_items {
                    usos_inventario[inventory_index] += needed_items;
                    items_satisfechos[index_asked].1 += needed_items;
                }else{
                    usos_inventario[inventory_index] += available_items;
                    items_satisfechos[index_asked].1 += available_items;
                    break;
                }
            }
        }

        let satisfied_needs = items_satisfechos.iter().position(
            |(index_asked, fullfilled)| pedido.requerimientos[*index_asked].necesidad>*fullfilled
        ).is_none();
        if satisfied_needs {
            for (index, using) in usos_inventario.iter().enumerate() {
                if *using == 0 {
                    continue;
                }
                self.rest_item_to_inventory(index, *using);
            }
            self.orders_made += 1;
            return true;
        }
        false
    }

    pub fn get_equiped_item(&self) -> Option<&(DynGd<RefCounted, dyn IItem>, u16)>{
        self.inventory[self.item_actual].as_ref()
    }
    pub fn get_inventory_item(&self, index : usize) -> Option<&(DynGd<RefCounted, dyn IItem>, u16)>{
        self.inventory[index].as_ref()
    }
    pub fn get_index_current_item(&self) -> usize{
        self.item_actual
    }
    pub fn get_inventory(&self) -> Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>> {
        self.inventory.clone()
    }
}
