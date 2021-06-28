use invk::{fabrik, inverse_kinematics, links::Link};
use nannou::prelude::*;
struct Model {
    links: Vec<Link>,
    mouse_pos: Vector2<f32>,
}
fn main() {
    nannou::app(model).update(update).run();
}
fn model(app: &App) -> Model {
    app.new_window()
        .title("Inverse Kinematics")
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let lengths = vec![200., 100., 80., 50., 30.];
    let links: Vec<Link> = lengths.iter().map(|length| Link {
        length: *length,
        angle: 0.0,
    }).collect();
    Model {
        links: links.clone(),
        mouse_pos: pt2(0.0, 0.0),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mouse_pos = nannou::math::cgmath::Vector2 {
        x: model.mouse_pos.x,
        y: model.mouse_pos.y,
    };
    // model.links = inverse_kinematics(&model.links, mouse_pos);
    model.links = fabrik::inverse_kinematics(&model.links, mouse_pos);
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(pos) => {
            model.mouse_pos = pos;
        }
        _other => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(CORAL);

    // Draw links
    let width = 30.;
    let extra = 5.;
    let mut next_joint = pt2(0., 0.);
    let mut angle_sum = 0.;
    for link in &model.links {
        angle_sum += link.angle;
        let centre_x = (link.length / 2.) * angle_sum.cos();
        let centre_y = (link.length / 2.) * angle_sum.sin();
        draw.ellipse()
            .w_h(link.length + 2. * extra, width)
            .color(GREENYELLOW)
            .stroke(DARKBLUE)
            .stroke_weight(1.)
            .z_radians(angle_sum)
            .x_y(next_joint.x + centre_x, next_joint.y + centre_y);
        draw.ellipse().w_h(3., 3.).xy(next_joint).color(DARKBLUE);
        next_joint = pt2(next_joint.x + 2. * centre_x, next_joint.y + 2. * centre_y);
    }

    draw.to_frame(app, &frame).unwrap();
}
