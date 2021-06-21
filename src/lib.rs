use nannou::math::cgmath::Vector2;

pub mod links;

pub fn inverse_kinematics(links: &Vec<links::Link>, end_pos: Vector2<f32>) -> Vec<links::Link> {
    let mut links = links.clone();
    links[0].angle = end_pos.y.atan2(end_pos.x);
    links.to_vec()
}

// pub fn direct_kinematics(links: &Vec<links::Link>) -> Vector2<f32> {

// }

// pub fn distance_from_goal(links: &Vec<links::Link>, end_pos: Vector2<f32>) -> f32 {

// }

// want to perform gradient descent over all the joint angles