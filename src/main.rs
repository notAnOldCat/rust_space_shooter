// * main.rs : fichier source de l'application

/*
    Palette utilisée pour le jeu:
        - https://lospec.com/palette-list/citrink (Joueur & Background)
        - https://lospec.com/palette-list/spooky6 (Ennemi)
*/

// Rend Clippy plus méticuleux
#![warn(clippy::all, clippy::pedantic)]

// * Modules
// -- Sources
// -- Macroquad: https://docs.rs/macroquad/latest/macroquad/index.html
// -- Miniquad: https://docs.rs/miniquad/latest/miniquad/index.html

// Utilise tout ce qui se trouve dans le module 'prelude' du crate 'macroquad'
use macroquad::prelude::*;
// use rand::ChooseRandom;

// * Structures & implémentations
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Shape {
    // Crée une nouvelle instance de type `Shape`
    fn new(size: f32, speed: f32, x: f32, y: f32, color: Color) -> Self {
        Self {
            size,
            speed,
            x,
            y,
            color,
        }
    }
}
// Cet attribut sert à demander à Macroquad d'appeler la fonction 'main' lorsque l'application démarre.
#[macroquad::main(set_window_conf)]
/// Exécute la boucle de jeu
async fn main() {
    const MOVEMENT_SPEED: f32 = 300.0;
    const CIRCLE_RADIUS: f32 = 32.0;

    // Dès lors qu'un littéral est suffisamment long, il est recommandé d'utiliser le symbole '_' comme séparateur.
    let color_background: Color = Color::from_hex(0x254_d70);
    // let enemy_colors = vec![Color::from_hex(0x35014b), Color::from_hex(0x7c0a72), Color::from_hex(0xbe1d47)];

    let mut squares: Vec<Shape> = vec![];

    // Représente le joueur
    let mut circle: Shape = Shape::new(
        CIRCLE_RADIUS * 2.0,
        MOVEMENT_SPEED,
        screen_width() / 2.0,
        screen_height() / 2.0,
        Color::from_hex(0xfcf_660),
    );

    // Génère une seed aléatoire utilisée pour les fonctions simulant le hasard.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    rand::srand(miniquad::date::now() as u64);

    loop {
        // Si la fonction n'est pas appelée explicitement, l'écran est tout de même rempli avec la couleur 'BLACK' au début
        // de chaque frame.
        clear_background(color_background);

        // Utiliser le temps écoulé depuis la dernière frame permet de convertir le mouvement par frame en mouvement par seconde.
        let delta_time = get_frame_time();

        // Chaque frame, un carré a un certain pourcentage de chances d'être généré
        if rand::gen_range(0, 99) >= 95 {
            let size: f32 = rand::gen_range(16.0, 64.0);
            // let color = enemy_colors.choose();
            squares.push(Shape::new(
                size,
                rand::gen_range(100.0, 300.0),
                rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                -size + size / 2.0,
                Color::from_hex(0xbe1_d47),
            ));
        }

        // Ajuste la vélocité des ennemis toutes les secondes
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        // Reçoit les inputs du joueur et ajuste sa vélocité en fonction.
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }

        // Brider le déplacement du joueur de sorte qu'il ne puisse franchir les bords de la fenêtre de jeu.
        circle.x = clamp(
            circle.x,
            0.0 + circle.size / 2.0,
            screen_width() - circle.size / 2.0,
        );
        circle.y = clamp(
            circle.y,
            0.0 + circle.size / 2.0,
            screen_height() - circle.size / 2.0,
        );

        // Supprime un ennemi dès lors qu'il franchit le bord inférieur de la fenêtre de jeu.
        squares.retain(|square| square.y < screen_height() + square.size);

        // Effectue le rendu des ennemis à l'écran.
        for square in &squares {
            draw_rectangle_ex(
                square.x,
                square.y,
                square.size,
                square.size,
                DrawRectangleParams {
                    offset: Vec2::new(0.5, 0.5),
                    rotation: 0.0,
                    color: square.color,
                },
            );
        }

        // Effectue le rendu du joueur à l'écran.
        draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);

        next_frame().await;
    }
}

/// Retourne sous forme de structure la configuration pour la fenêtre de jeu.
fn set_window_conf() -> Conf {
    Conf {
        window_title: String::from("Rusty Space Shooter"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}
