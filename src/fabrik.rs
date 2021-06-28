use crate::{direct_kinematics, links};
use nannou::math::{
    cgmath::{vec2, Vector2},
    MetricSpace,
};

pub fn inverse_kinematics(links: &Vec<links::Link>, end_pos: Vector2<f32>) -> Vec<links::Link> {
    let total_length = links.iter().map(|link| link.length).sum();
    let dist = end_pos.distance(vec2(0.0, 0.0));
    if dist > total_length {
        // goal is unreachable, just straighten all links
        let angle_to_goal = end_pos.y.atan2(end_pos.x);
        let mut result_links = links.clone();
        for link in &mut result_links {
            link.angle = 0.0;
        }
        result_links[0].angle = angle_to_goal;
        result_links
    } else {
        // goal is reachable, perform fabrik algorithm
        let n = links.len() + 1;
        let root = vec2(0.0, 0.0); // initial position of first joint, assumed 0 (b)
        let tolerance = 0.01;

        let mut positions = direct_kinematics(links);
        let end_effector = positions[n - 1];
        let mut diff = end_effector.distance(end_pos); // (difA)

        let mut iterations = 0;
        while diff > tolerance && iterations < 100 {
            // Stage 1: Forward reaching
            positions[n - 1] = end_pos;

            for i in (0..n - 1).rev() {
                let dist = positions[i + 1].distance(positions[i]);
                let lambda = links[i].length / dist;
                positions[i] = (1.0 - lambda) * positions[i + 1] + lambda * positions[i];
            }

            // Stage 2: Backward reaching
            positions[0] = root;

            for i in 0..n - 1 {
                let dist = positions[i + 1].distance(positions[i]);
                let lambda = links[i].length / dist;
                positions[i + 1] = (1.0 - lambda) * positions[i] + lambda * positions[i + 1];
            }
            diff = positions[n - 1].distance(end_pos);
            iterations += 1;
        }
        // println!("Iters: {}", iterations);

        links_from_positions(links, &positions)
    }
}

fn links_from_positions(
    orig_links: &Vec<links::Link>,
    positions: &Vec<Vector2<f32>>,
) -> Vec<links::Link> {
    let mut links = orig_links.clone();
    let mut angle_sum = 0.0;
    for i in 0..positions.len() - 1 {
        let diff = positions[i + 1] - positions[i];
        let total_angle = diff.y.atan2(diff.x);
        let joint_angle = total_angle - angle_sum;

        links[i].angle = joint_angle;
        angle_sum += joint_angle;
    }
    links
}

#[cfg(test)]
mod tests {
    use crate::fabrik::links_from_positions;
    use crate::links::Link;
    use nannou::math::cgmath::vec2;

    #[test]
    fn test_links_from_positions() {
        let l1 = Link {
            length: 200.,
            angle: 0.,
        };
        let l2 = Link {
            length: 100.,
            angle: 0.,
        };
        let l3 = Link {
            length: 80.,
            angle: 0.,
        };
        let links = vec![l1, l2, l3];
        let positions = vec![
            vec2(0.0, 0.0),
            vec2(
                200.0 * std::f32::consts::FRAC_PI_6.cos(),
                200.0 * std::f32::consts::FRAC_PI_6.sin(),
            ),
            vec2(
                200.0 * std::f32::consts::FRAC_PI_6.cos()
                    + 100. * std::f32::consts::FRAC_PI_4.cos(),
                200.0 * std::f32::consts::FRAC_PI_6.sin()
                    + 100. * std::f32::consts::FRAC_PI_4.sin(),
            ),
        ];
        let links = links_from_positions(&links, &positions);
        assert_eq!(links[1].angle, std::f32::consts::FRAC_PI_4);
    }

    #[test]
    fn test_quad2() {
        let l1 = Link {
            length: 200.,
            angle: 0.,
        };
        let l2 = Link {
            length: 100.,
            angle: 0.,
        };
        let links = vec![l1, l2];
        let positions = vec![
            vec2(0.0, 0.0),
            vec2(
                200.0 * (std::f32::consts::FRAC_PI_6 + std::f32::consts::FRAC_PI_2).cos(),
                200.0 * (std::f32::consts::FRAC_PI_6 + std::f32::consts::FRAC_PI_2).sin(),
            ),
            vec2(
                200.0 * (std::f32::consts::FRAC_PI_6 + std::f32::consts::FRAC_PI_2).cos()
                    + 100. * (std::f32::consts::FRAC_PI_4).cos(),
                200.0 * (std::f32::consts::FRAC_PI_6 + std::f32::consts::FRAC_PI_2).sin()
                    + 100. * (std::f32::consts::FRAC_PI_4).sin(),
            ),
        ];
        let links = links_from_positions(&links, &positions);
        assert_eq!(links[1].angle, std::f32::consts::FRAC_PI_4);
    }
}
