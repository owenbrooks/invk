use nannou::prelude::*;
mod links;
use links::Link;

struct Model {
    links: Vec<Link>,
}
fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}
fn model(_app: &App) -> Model {
    let l1 = Link {
        length: 10.,
        angle: 0.,
    };
    let l2 = Link {
        length: 10.,
        angle: 0.,
    };
    Model {
        links: vec![l1, l2],
    }
}
fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::Update(update) => {
            println!("{}", update.since_last.as_nanos());
        }
        _other => (),
    }
}
fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to purple.
    draw.background().color(CORAL);

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();
    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Draw a blue ellipse with a radius of 10 at the (x,y) coordinates of (0.0, 0.0)
    draw.ellipse().color(GREENYELLOW).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
