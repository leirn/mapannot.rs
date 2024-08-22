mod math;
mod rendering;

use slint::SharedString;

use rendering::{render_image, Color, Drawable, DrawableType, Point};

slint::include_modules!();

struct IdGenerator {
    id: i32,
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator { id: 0 }
    }

    fn get_id(&mut self) -> i32 {
        self.id += 1;
        log::debug!("New id: {}", self.id);
        self.id
    }
}

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();

    let current_color = Color { r: 255, g: 0, b: 0 };
    let mut id_generator = IdGenerator::new();
    let mut standing_point = Point { x: 0, y: 0 };
    let mut standing_drawable = None;
    let mut next_action = 0;

    let mut drawables: Vec<Drawable> = vec![];
    drawables.push(Drawable {
        id: id_generator.get_id(),
        point1: Point { x: 100, y: 0 },
        point2: Point { x: 100, y: 480 },
        color: current_color.clone(),
        object_type: DrawableType::Line,
        width: 2.5,
    });
    drawables.push(Drawable {
        id: id_generator.get_id(),
        point1: Point { x: 0, y: 100 },
        point2: Point { x: 640, y: 100 },
        color: current_color.clone(),
        object_type: DrawableType::Line,
        width: 2.5,
    });
    drawables.push(Drawable {
        id: id_generator.get_id(),
        point1: Point { x: 320, y: 240 },
        point2: Point { x: 320, y: 140 },
        color: current_color.clone(),
        object_type: DrawableType::Circle,
        width: 2.5,
    });
    drawables.push(Drawable {
        id: id_generator.get_id(),
        point1: Point { x: 320, y: 240 },
        point2: Point { x: 320, y: 140 },
        color: current_color.clone(),
        object_type: DrawableType::Point,
        width: 5.,
    });

    let d = drawables.clone();

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_request_increase_value({
        log::debug!("Entering on_request_increase_value");
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_decrease_value({
        log::debug!("Entering on_request_decrease_value");
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        }
    });

    ui.on_image_click({
        log::debug!("Entering on_image_click");
        let ui_handle = ui.as_weak();
        let ui = ui_handle.unwrap();

        move || {
            let x = ui.get_mouse_x();
            let y = ui.get_mouse_y();
            log::debug!("Mouse position = {x}, {y}");

            log::debug!("Current action: {}", ui.get_current_action());
            next_action = 0;

            let contextual_text = match ui.get_current_action() {
                // Get closest line
                0 => {
                    let closest_line = math::closest_line(Point { x: x, y: y }, drawables.clone());
                    match closest_line {
                        Some(line) => {
                            let output = format!("{:?}", line);
                            output
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Vertical line
                1 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: Point { x: x, y: 0 },
                        point2: Point { x: x, y: 480 },
                        color: current_color.clone(),
                        object_type: DrawableType::Line,
                        width: 2.5,
                    });
                    "Vertical line added".to_string()
                }
                // Horizontal line
                2 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: Point { x: 0, y: y },
                        point2: Point { x: 640, y: y },
                        color: current_color.clone(),
                        object_type: DrawableType::Line,
                        width: 2.5,
                    });
                    "Horizontal line added".to_string()
                }
                // Point
                3 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: Point { x: x, y: y },
                        point2: Point { x: 0, y: 0 },
                        color: current_color.clone(),
                        object_type: DrawableType::Point,
                        width: 2.5,
                    });
                    "Point added".to_string()
                }
                // First segment point
                4 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 5;
                    "Click on the second segment point".to_string()
                }
                // Second segment point
                5 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: standing_point,
                        point2: Point { x: x, y: y },
                        color: current_color.clone(),
                        object_type: DrawableType::Segment,
                        width: 2.5,
                    });
                    "Segment added".to_string()
                }
                // First angle computation line
                6 => {
                    let closest_line = math::closest_line(Point { x: x, y: y }, drawables.clone());
                    match closest_line {
                        Some(line) => {
                            standing_drawable = Some(line);
                            next_action = 7;
                            "Click on the second line".to_string()
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Second angle computation line
                7 => {
                    let closest_line = math::closest_line(Point { x: x, y: y }, drawables.clone());
                    match closest_line {
                        Some(line) => {
                            let line1 = standing_drawable.unwrap();
                            let angle = math::angle_between(line1, line);
                            let output: String = format!("The angle between {} and {} is {}° / {}°", line1.id, line.id, angle.to_degrees().abs(), 180. - angle.to_degrees().abs());
                            output
                        }
                        None => "No line found".to_string(),
                    }
                }
                // First line point
                8 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 5;
                    "Click on the second line point".to_string()
                }
                // Second line point
                9 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: standing_point,
                        point2: Point { x: x, y: y },
                        color: current_color.clone(),
                        object_type: DrawableType::Line,
                        width: 2.5,
                    });
                    "Line added".to_string()
                }
                // First half line point
                10 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 5;
                    "Click on the half broken line point".to_string()
                }
                // Second half line point
                11 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: standing_point,
                        point2: Point { x: x, y: y },
                        color: current_color.clone(),
                        object_type: DrawableType::DemiDroite,
                        width: 2.5,
                    });
                    "Half line added".to_string()
                }
                // Circle center
                12 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 13;
                    "Click on any point on the circle".to_string()
                }
                // Second circle point
                13 => {
                    drawables.push(Drawable {
                        id: id_generator.get_id(),
                        point1: standing_point,
                        point2: Point { x: x, y: y },
                        color: current_color.clone(),
                        object_type: DrawableType::Circle,
                        width: 2.5,
                    });
                    "Circle added".to_string()
                }
                // Delete object
                14 => {
                    // TODO : only works on lines and segments for now
                    let closest_line = math::closest_line(Point { x: x, y: y }, drawables.clone());
                    match closest_line {
                        Some(line) => {
                            // Remove line from drawables based on id
                            drawables.retain(|x| x.id != line.id);
                            "Object deleted".to_string()
                        }
                        None => "No object found".to_string(),
                    }
                }
                _ => String::new(),
            };
            let d = drawables.clone();

            ui.set_contextual_text(SharedString::from(contextual_text.as_str()));
            ui.set_map(render_image(&d));
            ui.set_current_action(next_action);
        }
    });

    let _thread = std::thread::spawn(move || {
        log::debug!("Entering thread");
        let handle_copy = ui_handle.clone();

        let _ = slint::invoke_from_event_loop(move || {
            log::debug!("Entering invoke_from_event_loop");
            let image = render_image(&d);
            let ui_local_handle = handle_copy.unwrap();

            ui_local_handle.set_map(image);
        });
    });

    ui.run()
}
