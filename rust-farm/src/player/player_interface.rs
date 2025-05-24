use godot::{classes::{CanvasLayer, GridContainer, ICanvasLayer, Label}, prelude::*};

use crate::{interfaces::utils::simple_slot_grid::SimpleGridSlot, item::item_resource::IItem, player::Player};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct PlayerInterface{
    player : Option<Gd<Player>>,
    base : Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for PlayerInterface {
    fn ready(&mut self,) {
        self.player = Some(self.base().get_parent().expect("Sin Jugador").cast::<Player>());
    }
    fn process(&mut self, _delta: f64,) {
        let mut etiqueta_puntos = self.base().get_node_as::<Label>("./Puntos");
        let player = self.player.as_ref().expect("Jugador no encontrado").bind();
        etiqueta_puntos.set_text(&format!("{}", player.get_puntos()));
    }
}

#[godot_api]
impl PlayerInterface{
    pub fn update_inventory(&mut self, max_size: u16, current_item_index: usize, inventory: &Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>){

        let mut grid = self.base().get_node_as::<GridContainer>("InventoryGrid");

        for mut node in grid.get_children().iter_shared(){
            node.queue_free();
        }

        let select_item_callable = &self.base().callable("item_selected");

        for index in 0..max_size as usize {
            let mut slot : Gd<SimpleGridSlot>;
            if let Some(Some((item, stack))) = &inventory.get(index) {
                slot = SimpleGridSlot::from_item_resource(&item, *stack , index);
                grid.add_child(&slot);
            }else {
                slot = SimpleGridSlot::new(index);
                grid.add_child(&slot);
            }
            if index == current_item_index {
                slot.bind().disable();
            }else {
                slot.connect("item_selected", select_item_callable);
            }
        }
    }

    #[func]
    fn item_selected(&mut self, index : u16){
        let player = self.player.as_mut().unwrap();
        player.bind_mut().select_item(index as usize);

        let max_size = player.bind().get_inventario_maximo();
        let current_item_index = player.bind().get_index_current_item();
        let inventory = player.bind().get_inventory();

        self.update_inventory(max_size, current_item_index, &inventory);
    }
}