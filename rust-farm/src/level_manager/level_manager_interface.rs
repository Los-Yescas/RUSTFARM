use godot::{classes::{CanvasLayer, GridContainer, ICanvasLayer}, prelude::*};

use crate:: level_manager::level_manager_node::LevelManager;

use super::{level_manager_node::Pedido, order::Order};

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

#[godot_api]
impl LevelManagerInterface {
    pub fn update_info(&mut self , orders : &Vec<Pedido>){
        let mut orders_grid = self.base().get_node_as::<GridContainer>("Orders/ScrollContainer/GridContainer");
        for mut child in orders_grid.get_children().iter_shared() {
            child.queue_free();
        }
        
        for (index, order) in orders.iter().enumerate(){
            let order = Order::from_order(order, index);

            orders_grid.add_child(&order);

            let items = order.get_node_as::<GridContainer>("GridContainer").get_children();
            let check_order_callable = &self.base().get_parent().unwrap().callable("check_order");
            for mut child in items.iter_shared(){
                child.connect("item_selected", check_order_callable);
            }
        }
    }
}