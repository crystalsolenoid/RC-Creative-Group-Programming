use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .simple_window(view)
//        .event(window_event)
        .run()
}

struct Model {
    invert_mouse_x: bool,
}

fn model(_app: &App) -> Model {
    Model {
        invert_mouse_x: false,
    }
}



fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(CORNFLOWERBLUE);

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();
    draw.tri()
        .points(win.bottom_left(), win.top_left(), win.top_right())
        .color(VIOLET);

    // Draw an ellipse to follow the mouse.
    let t = app.time * 0.001;
    draw.ellipse()
        .x_y(app.mouse.x * t.cos(), app.mouse.y)
        .radius(win.w() * 0.125 * t.sin())
        .color(RED);

    // Draw a line!
    draw.line()
        .weight(10.0 + (t.sin() * 0.5 + 0.5) * 90.0)
        .caps_round()
        .color(PALEGOLDENROD)
        .points(win.top_left() * t.sin(), win.bottom_right() * t.cos());

    // Draw a quad that follows the inverse of the ellipse.
    if model.invert_mouse_x {
        draw.quad()
            .x_y(-app.mouse.x, app.mouse.y)
            .color(DARKGREEN)
            .rotate(t);
    } else {
        draw.quad()
            .x_y(app.mouse.x, app.mouse.y)
            .color(DARKGREEN)
            .rotate(t);
    }


    // Draw a rect that follows a different inverse of the ellipse.
    draw.rect()
        .x_y(app.mouse.y, app.mouse.x)
        .w(app.mouse.x * 0.25)
        .hsv(t, 1.0, 1.0);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
