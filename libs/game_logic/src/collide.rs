use super::{Circle, Rect};

pub fn is_collide( c: &Circle, r: &Rect ) -> bool {
    let closest_x = if c.x < (r.x - r.w / 2.0) {
        r.x - r.w / 2.0
    } else if c.x > (r.x + r.w / 2.0) {
        r.x + r.w / 2.0
    } else {
        c.x
    };
    let closest_y = if c.y < (r.y - r.h / 2.0) {
        r.y - r.h / 2.0
    } else if c.y > (r.y + r.h / 2.0) {
        r.y + r.h / 2.0
    } else {
        c.y
    };
    return distance_squared( c.x, c.y, closest_x, closest_y ) < c.r * c.r;
}

fn distance_squared( x1: f32, y1: f32, x2: f32, y2: f32 ) -> f32 {
    let delta_x = x2 - x1;
    let delta_y = y2 - y1;
    return delta_x * delta_x + delta_y * delta_y;
}