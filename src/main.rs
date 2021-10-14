use std::f64::consts::PI;

use piston_window::*;

const HEIGHT:f64 = 600.0;
const WIDTH:f64 = 600.0;
const X_START:f64 = WIDTH / 2.0;
const Y_START:f64 = HEIGHT / 2.0;
const NUM_OF_RODS:i32 = 1500;

struct Rod {
    x1:f64,
    x2:f64,
    y1:f64,
    y2:f64,
    r1:f64,
    r2:f64,
    m1:f64,
    m2:f64,
    a1:f64,
    a2:f64,
    a1_v:f64,
    a2_v:f64,
    a1_a:f64,
    a2_a:f64,
    g:f64,
    red:f64,
    green:f64,
    blue:f64
}

impl Rod {
    pub fn new(x1:f64, x2:f64, y1:f64, y2:f64, r1:f64, r2:f64, m1:f64, m2:f64, a1:f64, a2:f64, a1_v:f64, a2_v:f64, a1_a:f64, a2_a:f64, g:f64, red:f64, green:f64, blue:f64) -> Rod{
        let r:Rod = Rod {
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            r1: r1,
            r2: r2,
            m1: m1,
            m2:m2,
            a1: a1,
            a2: a2,
            a1_v: a1_v,
            a2_v: a2_v,
            a1_a: a1_a,
            a2_a: a2_a,
            g: g,
            red: red,
            green: green,
            blue: blue
        };
        r
    }
}

fn get_rods() -> Vec<Rod>{
    let mut rods=Vec::new();

    let mut red:f64 = 255.0;
    let mut green:f64 = 5.0;
    let mut blue:f64 = 5.0;

    // Epsilon
    let e = 0.000001;

    // Double pendulum parameters
    for n in 0..NUM_OF_RODS {
        let r1:f64 = 125.0; // Radius axe n°1
        let r2:f64 = 125.0; // Radius axe n°2
        let m1:f64 = 10.0; // Masse object n°1
        let m2:f64 = 10.0; // Masse object n°2
        let a1:f64 = PI / 1.1 + e * (2.0 * (n as f64) - 1000.0 + 1.0); // Angle n°1
        let a2:f64 = PI / 1.1; // Angle n°2
        let a1_v:f64 = 0.0; // Velocity n°1
        let a2_v:f64 = 0.0; // Velocity n°2
        let g:f64 = 0.1; // Gravitational acceleration
    
        // Motion equations
        // Solving angular velocity of rod n°1
        let mut num1 = -g * (2.0 * m1 + m2) * a1.sin();
        let mut num2 = -m2 * g * (a1 - 2.0 * a2).sin();
        let mut num3 = -2.0 * (a1 - a2).sin() * m2;
        let mut num4 = a2_v * a2_v * r2 + a1_v * a1_v * r1 * (a1 - a2).cos();
        let mut den = r1 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos());
        let a1_a = (num1 + num2 + num3 * num4) / den; // Acceleration n°1
        
        // Solving angular velocity of rod n°1
        num1 = 2.0 * (a1 - a2).sin();
        num2 = a1_v * a1_v * r1 * (m1 + m2);
        num3 = g * (m1 + m2) * a1.cos();
        num4 = a2_v * a2_v * r2 * m2 * (a1 - a2).cos();
        den = r2 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos());
        let a2_a = (num1 * (num2 + num3 + num4)) / den; // Acceleration n°2
        
        // Calculate cartesian coordinates
        let x1 = r1 * a1.sin();
        let y1 = r1 * a1.cos();
        let x2 = x1 + r2 * a2.sin();
        let y2 = y1 + r2 * a2.cos();

        // Increments the red, green and blue variables
        if red == 255.0 && green < 255.0 && blue == 5.0 {
            // red -> yellow
            green += 1.0;
        } else if red <= 255.0 && green == 255.0 && blue == 5.0 && red != 5.0{
            // yellow -> green
            red -= 1.0;
        } else if red == 5.0 && green == 255.0 && blue < 255.0 {
            // green -> cyan
            blue += 1.0;
        } else if red == 5.0 && green <= 255.0 && blue == 255.0 && green != 5.0 {
            // cyan -> blue
            green -= 1.0;
        } else if red < 255.0 && green == 5.0 && blue == 255.0 {
            // blue -> magenta
            red += 1.0;
        } else if red == 255.0 && green == 5.0 && blue <= 255.0 && blue != 5.0 {
            // magenta -> red
            blue -= 1.0;
        }

        rods.push(Rod::new(x1, x2, y1, y2, r1, r2, m1, m2, a1, a2, a1_v, a2_v, a1_a, a2_a, g, red , green , blue));
    }
    rods
}
fn main() {
    let bg = [1.0, 1.0, 1.0, 1.0];
    let mut rods:Vec<Rod> = get_rods();
    let mut window:PistonWindow = WindowSettings::new("Pendulum",[WIDTH,HEIGHT]).exit_on_esc(true).build().unwrap();
    let mut events = window.events;

    while let Some(e) = events.next(&mut window){
        if let Some(_) = e.render_args(){
            let rs = &rods;
            window.draw_2d(&e,|c, g, _|{
                clear(bg, g);
                for r in rs {
                    line_from_to([(r.red as f32) / 255.0, (r.green as f32) / 255.0, (r.blue as f32) / 255.0, 1.0],1.0,[X_START, Y_START], [X_START + r.x1, Y_START + r.y1], c.transform, g);
                    line_from_to([(r.red as f32) / 255.0, (r.green as f32) / 255.0, (r.blue as f32) / 255.0, 1.0],1.0,[X_START + r.x1, Y_START + r.y1], [X_START + r.x2, Y_START + r.y2], c.transform, g);
                }
            });
        }

        if let Some(_) = e.update_args(){
            let rs = &mut rods;
            for r in rs{

                // Motion equations
                // Solving angular velocity of rod n°1
                let mut num1 = -r.g * (2.0 * r.m1 + r.m2) * r.a1.sin();
                let mut num2 = -r.m2 * r.g * (r.a1 - 2.0 * r.a2).sin();
                let mut num3 = -2.0 * (r.a1 - r.a2).sin() * r.m2;
                let mut num4 = r.a2_v * r.a2_v * r.r2 + r.a1_v * r.a1_v * r.r1 * (r.a1 - r.a2).cos();
                let mut den = r.r1 * (2.0 * r.m1 + r.m2 - r.m2 * (2.0 * r.a1 - 2.0 * r.a2).cos());
                r.a1_a = (num1 + num2 + num3 * num4) / den; // Acceleration n°1

                // Solving angular velocity of rod n°2
                num1 = 2.0 * (r.a1 - r.a2).sin();
                num2 = r.a1_v * r.a1_v * r.r1 * (r.m1 + r.m2);
                num3 = r.g * (r.m1 + r.m2) * r.a1.cos();
                num4 = r.a2_v * r.a2_v * r.r2 * r.m2 * (r.a1 - r.a2).cos();
                den = r.r2 * (2.0 * r.m1 + r.m2 - r.m2 * (2.0 * r.a1 - 2.0 * r.a2).cos());
                r.a2_a = (num1 * (num2 + num3 + num4)) / den; // Acceleration n°1

                // Calculate cartesian coordinates
                r.x1 = r.r1 * r.a1.sin();
                r.y1 = r.r1 * r.a1.cos();
                r.x2 = r.x1 + r.r2 * r.a2.sin();
                r.y2 = r.y1 + r.r2 * r.a2.cos();

                // Updating Velocity and Angle
                r.a1_v += r.a1_a;
                r.a2_v += r.a2_a;
                r.a1 += r.a1_v;
                r.a2 += r.a2_v;

                // Damping factor
                r.a1_v *= 0.999;
                r.a2_v *= 0.999;
            }
        }
    }
}
