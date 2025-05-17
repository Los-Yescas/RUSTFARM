use godot::{classes::{CanvasLayer, GridContainer, ICanvasLayer, Label}, prelude::*};

use crate::{interfaces::utils::simple_slot_grid::SimpleGridSlot, player::Player};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
struct PlayerInterface{
    player : Option<Gd<Player>>,
    base : Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for PlayerInterface {
    fn ready(&mut self,) {
        self.player = Some(self.base().get_parent().expect("Sin Jugador").cast::<Player>());

        let update_inventory_callable = self.base().callable("update_inventory");
        let player = self.player.as_mut().unwrap();

        player.add_user_signal("inventory_updated");
        player.connect("inventory_updated", &update_inventory_callable);
        self.update_inventory();
    }
    fn process(&mut self, _delta: f64,) {
        let mut etiqueta_puntos = self.base().get_node_as::<Label>("./Puntos");
        let player = self.player.as_ref().expect("Jugador no encontrado").bind();
        etiqueta_puntos.set_text(&format!("{}", player.get_puntos()));
    }
}

#[godot_api]
impl PlayerInterface{
    #[func]
    fn update_inventory(&mut self){

        let mut grid = self.base().get_node_as::<GridContainer>("InventoryGrid");

        for mut node in grid.get_children().iter_shared(){
            node.queue_free();
        }

        let player = self.player.as_ref().unwrap();
        let max_size = player.bind().get_inventario_maximo();
        let current_item_index = player.bind().get_index_current_item();
        let inventory = player.bind().get_inventory();

        let select_item_callable = &self.base().callable("selected_item");

        for index in 0..max_size as usize {
            let mut slot : Gd<SimpleGridSlot>;
            if let Some(Some((item, stack))) = &inventory.get(index) {
                slot = SimpleGridSlot::from_item_resource(&item, *stack , index);
                slot.add_user_signal("selected_item");
                grid.add_child(&slot);
            }else {
                slot = SimpleGridSlot::new(index);
                slot.add_user_signal("selected_item");
                grid.add_child(&slot);
            }
            if index == current_item_index {
                slot.bind().disable();
            }else {
                slot.connect("selected_item", select_item_callable);
            }
        }
    }

    #[func]
    fn selected_item(&mut self, index : u16){
        let player = self.player.as_mut().unwrap();
        player.bind_mut().select_item(index as usize);
        self.update_inventory();
    }
}