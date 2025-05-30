use godot::{classes::{Control, GridContainer, IControl, Label, ProgressBar}, prelude::*};

use crate::interfaces::utils::simple_slot_grid::SimpleGridSlot;

use super::level_manager_node::Pedido;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Order{
    base : Base<Control>,
    #[init(val=Pedido{
        requerimientos:Vec::new(),
        recompensa : 0,
        time_for_order : 0.0,
        time_passed : 0.0
    })]
    order : Pedido,
    index : usize
}

#[godot_api]
impl IControl for Order {
   fn ready(&mut self,) {
       for requerimiento in &self.order.requerimientos {
            let mut grid = self.base().get_node_as::<GridContainer>("GridContainer");
            let item = &requerimiento.item;
            let asked_for = requerimiento.necesidad;
            let slot = SimpleGridSlot::from_item_resource_mini(item, asked_for, self.index);

            grid.add_child(&slot);
       }
       let mut reward_label = self.base().get_node_as::<Label>("Reward");
       reward_label.set_text(&format!("{}$", self.order.recompensa));

       let mut bar = self.base().get_node_as::<ProgressBar>("TimeLeft");
        bar.set_max((self.order.time_for_order*100.0) as f64);
   } 
   fn process(&mut self, delta: f64,) {
       self.update_timer(delta);
   }
}  

#[godot_api]
impl Order {
    pub fn set_order(&mut self, order : &Pedido){
        self.order = order.clone();
    }
    fn set_index(&mut self, index : usize) {
        self.index = index;
    }

    pub fn from_order(order : &Pedido, index : usize) -> Gd<Order> {
        let order_scene = load::<PackedScene>("res://Interfaces/Ordenes/Orden.tscn");
        
        let mut new_order = order_scene.instantiate_as::<Order>();
        new_order.bind_mut().set_order(&order);
        new_order.bind_mut().set_index(index);
        new_order
    }

    pub fn update_timer(&mut self, delta : f64){
        self.order.time_passed += delta as f32;

        let mut bar = self.base().get_node_as::<ProgressBar>("TimeLeft");
        bar.set_value((self.order.time_passed * 100.0) as f64);
    }
}