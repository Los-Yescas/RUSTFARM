use godot::{classes::{Button, CanvasLayer, ColorRect, GridContainer, Label}, prelude::*};

use crate::{interfaces::utils::slot_grid::GridSlot, item::item_resource::IItem, player::Player, world_interactables::IWorldInteractable};


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
    #[export]
    factor_de_venta : f32,
    player : Option<Gd<Player>>
}

#[godot_dyn]
impl IWorldInteractable for Mercado {
    fn show_menu(&mut self, inventory : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>, points : u16) {
        let mut points_label = self.base().get_node_as::<Label>("./MarketUI/Points");
        points_label.set_text(&format!("{points}$"));
        self.update_sell_menu(inventory);
        self.update_buy_menu();
        self.show_buy_menu();
        let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
        
        interfaz.set_visible(true);
    }
}

#[godot_api]
impl INode2D for Mercado{
    fn ready(&mut self,) {

        if !(self.factor_de_venta > 0.0 && self.factor_de_venta <= 1.0){
            self.factor_de_venta = 1.0;
            godot_error!("Factor de Precio no valido");
        }

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

        let show_buy_callable = self.base().callable("show_buy_menu");
        let show_sell_callable = self.base().callable("show_sell_menu");
        let mut buy_menu_button = self.base().get_node_as::<Button>("./MarketUI/Buy");
        let mut sell_menu_button = self.base().get_node_as::<Button>("./MarketUI/Sell");
        buy_menu_button.connect("pressed", &show_buy_callable);
        sell_menu_button.connect("pressed", &show_sell_callable);

        let mut close_button = self.base().get_node_as::<Button>("MarketUI/Close");
        let show_menu_callable = self.base().callable("close_market");
        close_button.connect("pressed", &show_menu_callable);

        self.update_information();
    }
}

#[godot_api]
impl Mercado {
    #[func]
    fn show_market(&mut self){
        self.show_buy_menu();
        self.update_information();
        
        let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
        
        interfaz.set_visible(true);
        
    }
    #[func]
    fn close_market(&mut self){
        let mut interfaz = self.base().get_node_as::<CanvasLayer>("MarketUI");
        
        interfaz.set_visible(false);

        self.player.as_mut().unwrap().bind_mut().set_active(true);
    }
    #[func]
    fn buy_something(&mut self, index : u16){
        let player = self.player.as_mut().expect("Sin jugador");
        let full_inventory = player.bind().is_inventory_full();
        let item = self.items_a_la_venta.at(index as usize).dyn_bind().pick();
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
    #[func]
    fn sell_item(&mut self, index : u16){
        let mut player = self.player.as_mut().expect("Sin jugador").bind_mut();
        let index = index as usize;
        let (item, _) = player.get_inventory_item(index).unwrap();
        let precio = (item.dyn_bind().get_precio() as f32 * self.factor_de_venta) as u16;

        player.rest_item_to_inventory(index, 1);
        player.sum_points(precio);
        drop(player);
        self.update_information();
    }
    fn rest_item(&mut self, item : DynGd<RefCounted, dyn IItem>){
        let item_index = self.items_a_la_venta
            .iter_shared().position(|el| el.dyn_bind().get_name() == item.dyn_bind().get_name())
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
        let player = self.player.as_ref().expect("Sin jugador");
        let mut points_label = self.base().get_node_as::<Label>("./MarketUI/Points");
        let points = player.bind().get_puntos();
        points_label.set_text(&format!("{points}$"));
        let inventory = player.bind().get_inventory();
        self.update_sell_menu(inventory);
        self.update_buy_menu();
    }

    fn update_buy_menu(&mut self) {
        let mut buy_grid_container = self.base().get_node_as::<GridContainer>("./MarketUI/BuyMenu/MarketUI/GridContainer");

        for mut nodo in buy_grid_container.get_children().iter_shared() {
            nodo.queue_free();
        }

        for (i, item) in self.items_a_la_venta.iter_shared().enumerate(){
            
            let mut new_slot=GridSlot::from_item_resource(item, self.stock_de_items_a_la_venta.at(i), 1.0, i as u16);

            buy_grid_container.add_child(&new_slot);

            let buy_callable = self.base().callable("buy_something");
            new_slot.connect("item_selected", &buy_callable);
        }
    }

    fn update_sell_menu(&mut self, inventario : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>){
        let mut sell_grid_container = self.base().get_node_as::<GridContainer>("./MarketUI/SellMenu/MarketUI/GridContainer");

        for mut nodo in sell_grid_container.get_children().iter_shared() {
            nodo.queue_free();
        }

        
        for (i, inventory_slot) in inventario.iter().enumerate(){
            if let Some((item, stack)) = inventory_slot {
                let mut new_slot=GridSlot::from_item_resource(item.clone(), *stack, self.factor_de_venta, i as u16);
                sell_grid_container.add_child(&new_slot);
                let sell_callable = self.base().callable("sell_item");
                new_slot.connect("item_selected", &sell_callable);
            }   
        }
    }

    #[func]
    fn show_buy_menu(&mut self){
        let mut buy_menu = self.base().get_node_as::<ColorRect>("./MarketUI/BuyMenu");
        let mut sell_menu = self.base().get_node_as::<ColorRect>("./MarketUI/SellMenu");
        buy_menu.set_visible(true);
        sell_menu.set_visible(false);
    }

    #[func]
    fn show_sell_menu(&mut self){
        let mut buy_menu = self.base().get_node_as::<ColorRect>("./MarketUI/BuyMenu");
        let mut sell_menu = self.base().get_node_as::<ColorRect>("./MarketUI/SellMenu");
        buy_menu.set_visible(false);
        sell_menu.set_visible(true);
    }
}