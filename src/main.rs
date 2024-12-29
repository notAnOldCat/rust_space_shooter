// * main.rs : fichier source de l'application

// Rend Clippy plus méticuleux
#![warn(clippy::all, clippy::pedantic)]

// Modules
// Utilise tout ce qui se trouve dans le module 'prelude' du crate 'macroquad'
use macroquad::prelude::*;

// Cet attribut sert à demander à Macroquad d'appeler la fonction 'main' lorsque l'application démarre.
#[macroquad::main(set_window_conf)]
/// Exécute la boucle de jeu
async fn main() {
    // Si la fonction n'est pas appelée explicitement, l'écran est tout de même rempli avec la couleur 'BLACK' au début
    // de chaque frame.
    clear_background(BLACK);

    next_frame().await;
}

/// Retourne sous forme de structure la configuration pour la fenêtre de jeu.
fn set_window_conf() -> Conf {
    Conf {
        window_title: String::from("Tutorial Space Shooter"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
