use godot::{classes::{Button, CanvasLayer, GridContainer}, prelude::*};

use crate::{interfaces::utils::simple_slot_grid::SimpleGridSlot, item::item_resource::IItem, plant::items::fruit::FruitItemResource, player::Player, world_interactables::IWorldInteractable};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
struct SeedProcessor{
    base : Base<Node2D>,
    player : Option<Gd<Player>>
}

#[godot_api]
impl INode2D for SeedProcessor {
    fn ready(&mut self,) {
        self.player = self.base().try_get_node_as::<Player>("../Player");

        let mut close_button = self.base().get_node_as::<Button>("UI/Close");
        let show_menu_callable = self.base().callable("close_menu");
        close_button.connect("pressed", &show_menu_callable);
    }
}

#[godot_dyn]
impl IWorldInteractable for SeedProcessor{
    fn show_menu(&mut self, inventory : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>, _points : u16) {
        self.update_items(inventory);
        let mut ui = self.base().get_node_as::<CanvasLayer>("UI");
        ui.set_visible(true);
    }
}

#[godot_api]
impl SeedProcessor {
    #[func]
    fn close_menu(&mut self){
        let mut ui = self.base().get_node_as::<CanvasLayer>("UI");
        ui.set_visible(false);
        self.player.as_mut().unwrap().bind_mut().set_active(true);
    }

    fn update_items(&mut self,  inventario : Vec<Option<(DynGd<RefCounted, dyn IItem>, u16)>>){
        let mut grid = self.base().get_node_as::<GridContainer>("UI/Background/ItemGrid/GridContainer");

        for mut nodo in grid.get_children().iter_shared() {
            nodo.queue_free();
        }

        let produce_seeds_callable = self.base().callable("produce_seeds");
        for (i, inventory_slot) in inventario.iter().enumerate(){
            if let Some((item, stack)) = inventory_slot {
                if let Ok(_) = item.clone().into_gd().try_cast::<FruitItemResource>(){
                    let mut new_slot=SimpleGridSlot::from_item_resource(item, *stack, i);
                    grid.add_child(&new_slot);
                    new_slot.connect("item_selected", &produce_seeds_callable);
                } 
            }   
        }
    }
    fn update_information(&mut self){
        let player = self.player.as_ref().expect("Sin jugador");
        let inventory = player.bind().get_inventory();
        self.update_items(inventory);
    }

    #[func]
    fn produce_seeds(&mut self, index : u16){
        let mut sound = self.base().get_node_as::<AudioStreamPlayer>("FruitSound");

        let player = self.player.as_mut().expect("Sin jugador");
        let item = player.bind().get_inventory_item(index as usize).unwrap().0.clone();

        let fruta = item.into_gd().cast::<FruitItemResource>();
        let num_a_dar = fruta.bind().get_semillas_a_dar();
        let semilla = fruta.bind().give_seeds();

        let mut player = player.bind_mut();
        player.rest_item_to_inventory(index as usize, 1);
        
        for _i in 0..num_a_dar{
            match player.add_item_to_inventory(&semilla){
                Err(mess) => {
                    godot_print!("{mess}");
                    break;
                },
                Ok(_) => {
                    sound.play();
                }
            }
        }
        drop(player);
        self.update_information();
    }
}