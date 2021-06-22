use nannou::math::{
    cgmath::{vec2, Vector2},
    MetricSpace,
};

pub mod links;

pub fn inverse_kinematics(links: &Vec<links::Link>, end_pos: Vector2<f32>) -> Vec<links::Link> {
    // perform gradient descent over all the joint angles

    let min_angle = -4.*std::f32::consts::FRAC_PI_2;
    let max_angle = 4.*std::f32::consts::FRAC_PI_2;
    
    let delta = 0.01;
    let learn_rate = 0.0001;

    let mut links = links.clone();

    for _ in 0..10000 {
        let mut new_links = links.clone();
        for i in 0..links.len() {
            new_links[i].angle += delta;
            let error =
                distance_from_goal(&new_links, end_pos) - distance_from_goal(&links, end_pos);
            let gradient = error / delta;
            links[i].angle -= learn_rate * gradient;
            links[i].angle = clamp(links[i].angle, min_angle, max_angle);
        }
    }

    links
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn direct_kinematics(links: &Vec<links::Link>) -> Vector2<f32> {
    let mut angle_sum = 0.;
    let mut next_joint = vec2(0., 0.);
    for link in links {
        angle_sum += link.angle;
        let dx = link.length * angle_sum.cos();
        let dy = link.length * angle_sum.sin();
        next_joint = vec2(next_joint.x + dx, next_joint.y + dy);
    }
    next_joint
}

pub fn distance_from_goal(links: &Vec<links::Link>, goal_pos: Vector2<f32>) -> f32 {
    let end_pos = direct_kinematics(links);
    end_pos.distance(goal_pos)
}
