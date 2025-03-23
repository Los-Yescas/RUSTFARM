use godot::classes::InputEvent;
use godot::classes::TileMapLayer;
use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::classes::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Player {
    speed: f32,
    angular_speed: f64,
    input : Gd<Input>,
    is_moving : bool,
    target_position : Vector2,

    base: Base<Node2D>
}
use godot::classes::INode2D;

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        
        Self {
            speed: 500.0,
            angular_speed: std::f64::consts::PI,
            base,
            input : Input::singleton(),
            is_moving: false,
            target_position: Vector2::ZERO
        }
    }
    
    fn process(&mut self, delta: f64,) {
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
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32

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
            self.target_position = map.map_to_local(target_tile)
        }
    }
}
