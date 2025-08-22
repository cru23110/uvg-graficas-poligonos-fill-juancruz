mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::{draw_polygon_outline, fill_polygons_even_odd};

fn main() {
    let width: u32 = 800;
    let height: u32 = 600;

    let mut fb = Framebuffer::new(width, height);
    fb.set_background_color(Color::new(240, 220, 200, 255));
    fb.clear();

    // Polígono 1
    let poly1: Vec<(i32, i32)> = vec![
        (165,380),(185,360),(180,330),(200,327),(207,345),
        (233,330),(230,360),(250,380),(220,385),(205,410),(193,383),
    ];

    // Polígono 2
    let poly2: Vec<(i32, i32)> = vec![
        (321,335),(288,286),(339,251),(374,302),
    ];

    // Polígono 3
    let poly3: Vec<(i32, i32)> = vec![
        (377,249),(411,197),(436,249),
    ];

    // Polígono 4 (contorno exterior)
    let poly4: Vec<(i32, i32)> = vec![
        (413,177),(448,159),(502,180),(582,85),(533,53),(535,36),(676,37),(660,52),
        (750,145),(761,167),(679,129),(659,129),(614,216),(415,214),(632,230),(580,230),
        (597,215),(552,214),(514,157),(446,180),
    ];

    // Polígono 5 (AGUJERO dentro de polígono 4 – NO debe pintarse)
    let hole4: Vec<(i32, i32)> = vec![
        (682,175),(708,120),(735,148),(739,170),
    ];

    // -------------------------
    // RELLENOS
    // -------------------------

    // P1
    fb.set_current_color(Color::new(200, 40, 40, 255)); // relleno
    fill_polygons_even_odd(&mut fb, &[poly1.clone()]);

    // P2
    fb.set_current_color(Color::new(40, 160, 60, 255));
    fill_polygons_even_odd(&mut fb, &[poly2.clone()]);

    // P3
    fb.set_current_color(Color::new(40, 90, 200, 255));
    fill_polygons_even_odd(&mut fb, &[poly3.clone()]);

    // P4 con agujero (even-odd: contorno exterior + agujero)
    fb.set_current_color(Color::new(230, 150, 40, 255));
    fill_polygons_even_odd(&mut fb, &[poly4.clone(), hole4.clone()]);

    // -------------------------
    // CONTORNOS (línea)
    // -------------------------
    fb.set_current_color(Color::BLACK);
    draw_polygon_outline(&mut fb, &poly1);
    draw_polygon_outline(&mut fb, &poly2);
    draw_polygon_outline(&mut fb, &poly3);
    draw_polygon_outline(&mut fb, &poly4);
    draw_polygon_outline(&mut fb, &hole4);

    // Exportar resultado
    let output_file = "out.bmp";
    fb.render_to_file(output_file);

    println!("Listo. Imagen generada en {}", output_file);
}