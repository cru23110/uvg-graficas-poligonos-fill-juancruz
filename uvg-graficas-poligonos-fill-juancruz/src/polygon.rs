// use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_polygon_outline(fb: &mut Framebuffer, pts: &[(i32, i32)]) {
    if pts.len() < 2 { return; }
    for i in 0..pts.len() {
        let (x0, y0) = pts[i];
        let (x1, y1) = pts[(i + 1) % pts.len()];
        line(fb, x0, y0, x1, y1);
    }
}

pub fn fill_polygons_even_odd(fb: &mut Framebuffer, contours: &[Vec<(i32, i32)>]) {
    if contours.is_empty() { return; }

    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    for c in contours {
        for &(_x, y) in c {
            ymin = ymin.min(y);
            ymax = ymax.max(y);
        }
    }
    ymin = ymin.max(0);
    ymax = ymax.min(fb.height as i32 - 1);

    let mut edges: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for c in contours {
        if c.len() < 2 { continue; }
        for i in 0..c.len() {
            let p0 = c[i];
            let p1 = c[(i + 1) % c.len()];
            edges.push((p0, p1));
        }
    }

    for y in ymin..=ymax {
        let mut x_intersections: Vec<i32> = Vec::new();

        for &((x0, y0), (x1, y1)) in &edges {
            if y0 == y1 { continue; }

            let (ymin_e, ymax_e, x_at_ymin, x_at_ymax) = if y0 < y1 {
                (y0, y1, x0, x1)
            } else {
                (y1, y0, x1, x0)
            };

            if y >= ymin_e && y < ymax_e {
                let t_num = (y - ymin_e) as f32;
                let t_den = (ymax_e - ymin_e) as f32;
                let t = if t_den != 0.0 { t_num / t_den } else { 0.0 };
                let x = x_at_ymin as f32 + t * ((x_at_ymax - x_at_ymin) as f32);
                x_intersections.push(x.floor() as i32);
            }
        }

        if x_intersections.is_empty() { continue; }

        x_intersections.sort_unstable();

        let mut it = x_intersections.iter();
        let mut parity = false; 
        let mut prev_x = 0;

        while let Some(&xi) = it.next() {
            parity = !parity;
            if parity {
                prev_x = xi;
            } else {
                let xstart = prev_x;
                let xend = xi;
                if xend >= 0 && xstart < fb.width as i32 {
                    let xs = xstart.max(0);
                    let xe = xend.min(fb.width as i32 - 1);
                    for x in xs..=xe {
                        fb.set_pixel(x, y);
                    }
                }
            }
        }
    }
}