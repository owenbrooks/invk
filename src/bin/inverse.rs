use learn_nannou::links::Link;
use nannou::prelude::*;
struct Model {
    links: Vec<Link>,
    mouse_pos: Vector2<f32>,
}
fn main() {
    nannou::app(model).update(update).run();
}
fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();
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
    let l4 = Link {
        length: 50.,
        angle: 0.,
    };
    Model {
        links: vec![l1, l2, l3, l4],
        mouse_pos: pt2(0., 0.),
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let time = update.since_start.as_secs_f32();
    let scale_factor = 1.;
    let new_angle = time * scale_factor % std::f32::consts::TAU;
    for link in &mut model.links {
        link.angle = new_angle;
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::MouseMoved(pos) => {
            model.mouse_pos = pos;
        },
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
        let centre_x = (link.length / 2. - extra) * angle_sum.cos();
        let centre_y = (link.length / 2. - extra) * angle_sum.sin();
        draw.ellipse()
            .w_h(link.length + extra, width)
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
