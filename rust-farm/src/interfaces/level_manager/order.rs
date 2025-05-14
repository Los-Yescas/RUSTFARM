use godot::{classes::{Control, GridContainer, IControl}, prelude::*};

use crate::{interfaces::utils::simple_slot_grid::SimpleGridSlot, item::item_resource::IItem};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Order{
    base : Base<Control>,
    order : Vec<(DynGd<RefCounted, dyn IItem>, u16)>
}

#[godot_api]
impl IControl for Order {
   fn ready(&mut self,) {
       for (index, order) in self.order.iter().enumerate() {
            let mut grid = self.base().get_node_as::<GridContainer>("Fondo/GridContainer");
            let (item, asked_for) = &order;
            let slot = SimpleGridSlot::from_item_resource_mini(item, *asked_for, index);

            grid.add_child(&slot);
       }
   } 
}  

#[godot_api]
impl Order {
    fn set_order(&mut self, order : Vec<(DynGd<RefCounted, dyn IItem>, u16)>){
        self.order = order;
    }

    pub fn from_order(order : &Vec<(DynGd<RefCounted, dyn IItem>, u16)>) -> Gd<Order> {
        let order_scene = load::<PackedScene>("res://Interfaces/Ordenes/Orden.tscn");
        
        let mut new_order = order_scene.instantiate_as::<Order>();
        new_order.bind_mut().set_order(order.clone());
        new_order
    }
}