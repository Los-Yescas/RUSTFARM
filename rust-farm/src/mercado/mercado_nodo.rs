use godot::{classes::{CanvasLayer, GridContainer, InputEvent, Label}, prelude::*};

use crate::{interfaces::utils::slot_grid::GridSlot, item::item_resource::IItem, player::Player};


#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct Mercado {
    base : Base<Node2D>,
    #[export]
    items_a_la_venta_rutas : Array<GString>,
    //Quisiera un array de tuplas de el item y el stock disponible
    //Pero por limitaciones de godot-rust se separon en 2 arrays
    items_a_la_venta : Array<DynGd<RefCounted, dyn IItem>>,
    #[export]
    stock_de_items_a_la_venta : Array<u16>,
    player : Option<Gd<Player>>
}

#[godot_api]
impl INode2D for Mercado{
    fn ready(&mut self,) {
        for ruta in self.items_a_la_venta_rutas.iter_shared() {
            let resource : Gd<Resource> = load(&ruta);
            let item : DynGd<RefCounted, dyn IItem> = resource.to_variant().to();
            self.items_a_la_venta.push(&item);
        }

        self.player = self.base().try_get_node_as::<Player>("../Player");

        if self.items_a_la_venta.len() != self.stock_de_items_a_la_venta.len() {
            godot_error!("Stock y items no son del mismo tamano");
            return;
        }

        self.update_information();
    }
    fn input(&mut self, event: Gd<InputEvent>,) {
        if event.is_action_pressed("market") {
            let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
            let is_visible = interfaz.is_visible();
            interfaz.set_visible(!is_visible);
        }
    }
}

#[godot_api]
impl Mercado {
    #[func]
    fn buy_something(&mut self, item : DynGd<RefCounted, dyn IItem>){
        let player = self.player.as_mut().expect("Sin jugador");
        let full_inventory = player.bind().is_inventory_full();
        let precio = item.dyn_bind().get_precio();

        if !full_inventory && player.bind().get_puntos() >= precio{
            let result = player.bind_mut().add_item_to_inventory(&item);
            match result {
                Ok(_message) => {
                    player.bind_mut().rest_points(precio);
                    self.rest_item(item);
                    self.update_information();
                },
                Err(message) => godot_print!("{message}")
            }
        }
    }   
    fn rest_item(&mut self, item : DynGd<RefCounted, dyn IItem>){
        let item_index = self.items_a_la_venta
            .iter_shared().position(|el| el == item)
            .expect("Item clickeado sin existir");
        let mut stock = self.stock_de_items_a_la_venta.at(item_index);
        stock -= 1;

        if stock <= 0 {
            self.stock_de_items_a_la_venta.remove(item_index);
            self.items_a_la_venta.remove(item_index);
        }else{
            self.stock_de_items_a_la_venta.set(item_index, stock);
        }
    }
    fn update_information(&mut self){
        self.update_buy_menu();

        let player = self.player.as_ref().expect("Sin jugador");
        let mut points_label = self.base().get_node_as::<Label>("./MarketUI/Points");
        let points = player.bind().get_puntos();
        points_label.set_text(&format!("{points}$"));

    }
    fn update_buy_menu(&mut self) {
        let mut buy_grid_container = self.base().get_node_as::<GridContainer>("./MarketUI/BuyMenu/MarketUI/GridContainer");

        for mut nodo in buy_grid_container.get_children().iter_shared() {
            nodo.queue_free();
        }

        for (i, item) in self.items_a_la_venta.iter_shared().enumerate(){
            let grid_slot: Gd<PackedScene> = load("res://Interfaces/Slot.tscn");
            let new_node = grid_slot.instantiate().unwrap();
            let mut new_slot = new_node.cast::<GridSlot>();
            new_slot.bind_mut().from_item_resource(item, self.stock_de_items_a_la_venta.at(i));

            buy_grid_container.add_child(&new_slot);

            new_slot.add_user_signal("selected_item");
            let buy_callable = self.base().callable("buy_something");
            new_slot.connect("selected_item", &buy_callable);
        }
    }
}