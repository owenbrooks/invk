use crate::clamp;
use crate::direct_kinematics;
use crate::links;
use nannou::math::{
    cgmath::Vector2,
    MetricSpace,
};

pub fn inverse_kinematics(links: &Vec<links::Link>, end_pos: Vector2<f32>) -> Vec<links::Link> {
    // perform gradient descent over all the joint angles

    let max_change = 0.001;

    let delta = 0.01;
    let learn_rate = 0.0001;
    let epsilon = 0.1; // acceptable threshold distance to goal

    let mut links = links.clone();

    let iterations = 10000;
    for _ in 0..iterations {
        if distance_from_goal(&links, end_pos) < epsilon {
            break;
        }

        let mut new_links = links.clone();
        for i in (0..links.len()).rev() {
            new_links[i].angle += delta;
            let error =
                distance_from_goal(&new_links, end_pos) - distance_from_goal(&links, end_pos);
            let gradient = error / delta;
            let angle_diff = learn_rate * gradient;
            let angle_diff = clamp(angle_diff, -max_change, max_change);
            links[i].angle -= angle_diff;
        }
    }

    links
}

pub fn distance_from_goal(links: &Vec<links::Link>, goal_pos: Vector2<f32>) -> f32 {
    let positions = direct_kinematics(links);
    let end_pos = positions.last().unwrap();
    end_pos.distance(goal_pos)
}
