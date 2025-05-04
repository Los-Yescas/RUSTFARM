use godot::{classes::{CanvasLayer, InputEvent}, prelude::*};

use crate::item::item_resource::IItem;

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
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
        if event.is_action_pressed("market") {
            let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
            let is_visible = interfaz.is_visible();
            interfaz.set_visible(!is_visible);
        }
    }
}