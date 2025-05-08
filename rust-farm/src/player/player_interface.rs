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
        self.player = Some(self.base().get_parent().expect("Sin Jugador").cast::<Player>());
    }
    fn process(&mut self, _delta: f64,) {
        let mut etiqueta_objeto = self.base().get_node_as::<Label>("./ActualObject");
        let mut etiqueta_puntos = self.base().get_node_as::<Label>("./Puntos");
        let player = self.player.as_ref().expect("Jugador no encontrado").bind();
        let objeto = player.get_equiped_item();
        let texto : GString = match objeto {
            None => "Sin objeto".into(),
            Some(tupla) => {
                let (nodo, stack) = tupla;
                format!("{:#?} {}", nodo.dyn_bind().get_name(), stack).into()
            }
        };
        etiqueta_objeto.set_text(&texto);
        etiqueta_puntos.set_text(&format!("{}", player.get_puntos()));
    }
}