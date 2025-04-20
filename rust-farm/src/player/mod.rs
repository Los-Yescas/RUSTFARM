use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::INode2D;

use crate::item::item_resource::plant_items::seed_item::SeedItemResource;
use crate::item::item_resource::IItem;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    speed: f32,
    input : Gd<Input>,
    is_moving : bool,
    target_position : Vector2,
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
            target_position: Vector2::ZERO
        }
    }
    
    fn process(&mut self, _delta: f64,) {
        

        if self.is_moving{
            return;
        }

        if self.input.is_action_pressed("up"){
            self.move_to(Vector2i::UP);
        }else if self.input.is_action_pressed("down") {
            self.move_to(Vector2i::DOWN)
        }else if self.input.is_action_pressed("right") {
            self.move_to(Vector2i::RIGHT)
        }else if self.input.is_action_pressed("left") {
            self.move_to(Vector2i::LEFT)
        }

        if self.input.is_action_just_pressed("interact"){
            self.interact();
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

    fn interact(&self){
        let semilla : Gd<SeedItemResource> = load("res://Plantas/Items Semillas/planta_fea_semilla.tres");
        semilla.bind().interact(self.base().get_node_as::<Node2D>("../../Node2D"), self.base().get_position());
    }
}
