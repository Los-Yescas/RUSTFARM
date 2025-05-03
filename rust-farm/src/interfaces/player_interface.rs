use godot::{classes::{CanvasLayer, ICanvasLayer, Label}, prelude::*};

use crate::player::Player;

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
struct PlayerInterface{
    player : Option<Gd<Player>>,
    base : Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for PlayerInterface {
    fn ready(&mut self,) {
        self.player = self.base().try_get_node_as::<Player>("../Player")
    }
    fn process(&mut self, _delta: f64,) {
        let mut etiqueta = self.base().get_node_as::<Label>("./ActualObject");
        let player = self.player.as_ref().expect("Jugador no encontrado").bind();
        let objeto = player.get_equiped_item();
        let texto : GString = match objeto {
            None => "Sin objeto".into(),
            Some(tupla) => {
                let (nodo, stack) = tupla;
                format!("{:#?} {}", nodo.dyn_bind().get_name(), stack).into()
            }
        };
        etiqueta.set_text(&texto);
    }
}