use nannou::math::{
    cgmath::{vec2, Vector2},
};

pub mod fabrik;
pub mod grad_desc;
pub mod links;

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn direct_kinematics(links: &Vec<links::Link>) -> Vec<Vector2<f32>> {
    // returns the coordinates of the end-effector (the end of the final link)
    let mut angle_sum = 0.;
    let mut next_joint = vec2(0., 0.);
    let mut positions = vec![next_joint];

    for link in links {
        angle_sum += link.angle;
        let dx = link.length * angle_sum.cos();
        let dy = link.length * angle_sum.sin();
        next_joint = vec2(next_joint.x + dx, next_joint.y + dy);
        positions.push(next_joint);
    }
    positions
}
