use godot::{classes::{CanvasLayer, GridContainer, InputEvent}, prelude::*};

use crate::{interfaces::utils::slot_grid::GridSlot, item::item_resource::IItem};

use super::mercado_interfaz::MarketUI;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Mercado {
    base : Base<Node2D>,
    #[export]
    items_a_la_venta_rutas : Array<GString>,
    items_a_la_venta : Array<DynGd<RefCounted, dyn IItem>>
}

#[godot_api]
impl INode2D for Mercado{
    fn ready(&mut self,) {
        for ruta in self.items_a_la_venta_rutas.iter_shared() {
            let resource : Gd<Resource> = load(&ruta);
            let item : DynGd<RefCounted, dyn IItem> = resource.to_variant().to();
            self.items_a_la_venta.push(&item);
        }
        let mut ui = self.base().get_node_as::<GridContainer>("./MarketUI/Background/MarketUI/GridContainer");
        for item in self.items_a_la_venta.iter_shared(){
            let grid_slot: Gd<PackedScene> = load("res://Interfaces/Slot.tscn");
            let new_node = grid_slot.instantiate().unwrap();
            let mut new_slot = new_node.cast::<GridSlot>();
            new_slot.bind_mut().from_item_resource(item);

            ui.add_child(&new_slot);
        }
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
        if event.is_action_pressed("market") {
            let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
            let is_visible = interfaz.is_visible();
            interfaz.set_visible(!is_visible);
        }
    }
}