use godot::{classes::{Button, CanvasLayer, Control, GridContainer, IControl}, prelude::*};
use level_slot::LevelSlot;

pub mod level_slot;

#[derive(GodotClass)]
#[class(init, base=Control)]
struct MainMenu{
    base : Base<Control>,
    #[export]
    level_list : Array<Option<Gd<PackedScene>>>
}

#[godot_api]
impl IControl for MainMenu {
    fn ready(&mut self,) {
        let mut start_button = self.base().get_node_as::<Button>("MenuPrincipal/Fondo/Iniciar");
        let mut level_selection_button = self.base().get_node_as::<Button>("MenuPrincipal/Fondo/SeleccionarNivel");
        let mut exit_button = self.base().get_node_as::<Button>("MenuPrincipal/Fondo/Salir");

        start_button.connect("pressed", &self.base().callable("start_game"));
        level_selection_button.connect("pressed", &self.base().callable("show_level_selection"));
        exit_button.connect("pressed", &self.base().callable("exit_game"));

        let mut back_button = self.base().get_node_as::<Button>("SeleccionDeNivel/Fondo/Back");
        back_button.connect("pressed", &self.base().callable("back_to_menu"));

        let mut grid = self.base().get_node_as::<GridContainer>("SeleccionDeNivel/Fondo/GridContainer");
        for (index, level) in self.level_list.iter_shared().enumerate() {
            let level = level.expect("Main Menu sin nivel");
            let new_slot = LevelSlot::new_level_slot(level.get_path(), index as u16 + 1);
            grid.add_child(&new_slot);
        }
    }
}

#[godot_api]
impl MainMenu {

    #[func]
    fn start_game(&mut self){
        let level = self.level_list.at(0).expect("Sin niveles").get_path();
        self.base().get_tree().unwrap().change_scene_to_file(&level);
    }
    #[func]
    fn show_level_selection(&mut self){
        self.base().get_node_as::<CanvasLayer>("SeleccionDeNivel").set_visible(true);
    }
    #[func]
    fn exit_game(&mut self){
        self.base().get_tree().unwrap().quit();
    }
    #[func]
    fn back_to_menu(&mut self){
        self.base().get_node_as::<CanvasLayer>("SeleccionDeNivel").set_visible(false);
    }
}