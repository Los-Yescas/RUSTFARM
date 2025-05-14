use godot::{classes::{CanvasLayer, GridContainer, ICanvasLayer}, prelude::*};

use crate::{item::item_resource::IItem, level_manager::level_manager_node::LevelManager};

use super::order::Order;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct LevelManagerInterface{
    base : Base<CanvasLayer>,
    level_manager : Option<Gd<LevelManager>>
}

#[godot_api]
impl ICanvasLayer for LevelManagerInterface {
    fn ready(&mut self,) {
        self.level_manager = Some(self.base().get_parent().expect("Sin padre").cast());
    }
}

impl LevelManagerInterface {
    pub fn update_info(&mut self , orders : &Vec<Vec<(DynGd<RefCounted, dyn IItem>, u16)>>){
        let mut orders_grid = self.base().get_node_as::<GridContainer>("Orders/GridContainer");
        for mut child in orders_grid.get_children().iter_shared() {
            child.queue_free();
        }
        
        for order in orders{
            let order = Order::from_order(order);
            orders_grid.add_child(&order);
        }
    }
}