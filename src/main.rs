mod vec2;
mod vec3;

use vec2::Vec2;

fn main() {
    let mut v = Vec2::new(0., 0.);
    let mut unit = Vec2::new(1., 1.);
    unit /= 2.;

    println!("v: {:?}", unit);
}
