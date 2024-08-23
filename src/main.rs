mod math;
mod rendering;

use slint::{SharedString, VecModel};

use rendering::{Color, DrawableType, Point, Renderer};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();

    let mut renderer = Renderer::new();
    let mut standing_point = Point { x: 0, y: 0 };
    let mut standing_drawable = None;
    let mut next_action = 0;

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_image_click({
        log::debug!("Entering on_image_click");
        let ui_handle = ui.as_weak();
        let ui = ui_handle.unwrap();

        move || {
            let x = ui.get_mouse_x();
            let y = ui.get_mouse_y();
            let red = ui.get_stroke_red().round() as u8;
            let green = ui.get_stroke_green().round() as u8;
            let blue = ui.get_stroke_blue().round() as u8;
            let width = ui.get_stroke_width();
            log::debug!("Mouse position = {x}, {y}");

            log::debug!("Current action: {}", ui.get_current_action());

            let contextual_text = match ui.get_current_action() {
                // Get closest line
                0 => {
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
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
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        Point { x: x, y: 0 },
                        Point { x: x, y: 480 },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    "Vertical line added".to_string()
                }
                // Horizontal line
                2 => {
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        Point { x: 0, y: y },
                        Point { x: 640, y: y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    "Horizontal line added".to_string()
                }
                // Point
                3 => {
                    renderer.add_drawable_by_values(
                        DrawableType::Point,
                        Point { x: x, y: y },
                        Point { x: 0, y: 0 },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
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
                    renderer.add_drawable_by_values(
                        DrawableType::Segment,
                        standing_point,
                        Point { x: x, y: y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    next_action = 0;
                    "Segment added".to_string()
                }
                // First angle computation line
                6 => {
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
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
                    next_action = 0;
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_line {
                        Some(line) => {
                            let line1 = standing_drawable.unwrap();
                            let angle = math::angle_between(line1, line);
                            let output: String = format!(
                                "The angle between {} and {} is {}° / {}°",
                                line1.id,
                                line.id,
                                angle.to_degrees().abs(),
                                180. - angle.to_degrees().abs()
                            );
                            output
                        }
                        None => "No line found".to_string(),
                    }
                }
                // First line point
                8 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 9;
                    "Click on the second line point".to_string()
                }
                // Second line point
                9 => {
                    next_action = 0;
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        standing_point,
                        Point { x: x, y: y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    "Line added".to_string()
                }
                // First half line point
                10 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 11;
                    "Click on the half broken line point".to_string()
                }
                // Second half line point
                11 => {
                    next_action = 0;
                    renderer.add_drawable_by_values(
                        DrawableType::HalfLine,
                        standing_point,
                        Point { x: x, y: y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
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
                    next_action = 0;
                    renderer.add_drawable_by_values(
                        DrawableType::Circle,
                        standing_point,
                        Point { x: x, y: y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    "Circle added".to_string()
                }
                // Delete object
                14 => {
                    let closest_line =
                        math::closest_object(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_line {
                        Some(line) => {
                            // Remove line from drawables based on id
                            renderer.remove_drawable(line.id);
                            "Object deleted".to_string()
                        }
                        None => "No object found".to_string(),
                    }
                }
                // First point to measure distance
                15 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 16;
                    "Click on the second point".to_string()
                }
                // Second point to measure distance
                16 => {
                    next_action = 0;
                    let m_per_px = ui.get_m_per_px();
                    let distance = math::distance(standing_point, Point { x: x, y: y });
                    format!(
                        "Distance beetwen two points is {:.2} km",
                        distance * m_per_px / 1000.
                    )
                }
                // Point to measure distance to line
                17 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 18;
                    "Click on the second point".to_string()
                }
                // Line to measure distance
                18 => {
                    next_action = 0;
                    let m_per_px = ui.get_m_per_px();
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_line {
                        Some(line) => {
                            let distance = math::perpendicular_distance(standing_point, line);
                            format!(
                                "Distance beetwen two points is {:.2} km",
                                distance * m_per_px / 1000.
                            )
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Circle radius
                19 => {
                    next_action = 0;
                    let m_per_px = ui.get_m_per_px();
                    let closest_circle = math::closest_circle(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_circle {
                        Some(circle) => {
                            let distance = math::distance(circle.point1, circle.point2);
                            format!(
                                "The radius is {:.2} km",
                                distance * m_per_px / 1000.
                            )
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Point the parallel line will go through
                20 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 21;
                    "Click on a line".to_string()
                }
                // Line of reference for the parallel line
                21 => {
                    next_action = 0;
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_line {
                        Some(closest_line) => {
                            let mut drawable = math::parallel_line(standing_point, closest_line);
                            drawable.color = Color {
                                r: red,
                                g: green,
                                b: blue,
                            };
                            drawable.width = width;
                            renderer.add_drawable(drawable);
                            "Parallel line added".to_string()
                        }
                        None => "No line found".to_string(),
                    }
                }
                // First point for median
                22 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 23;
                    "Click on the second point".to_string()
                }
                // Second point for median
                23 => {
                    next_action = 0;
                    let mut drawable = math::median_line(standing_point, Point { x: x, y: y });
                    drawable.color = Color {
                        r: red,
                        g: green,
                        b: blue,
                    };
                    drawable.width = width;
                    renderer.add_drawable(drawable);
                    "Median line added".to_string()
                }
                // Point the perpendicular line will go through
                24 => {
                    standing_point = Point { x: x, y: y };
                    next_action = 25;
                    "Click on a line".to_string()
                }
                // Line of reference for the perpendicular line
                25 => {
                    next_action = 0;
                    let closest_line = math::closest_line(Point { x: x, y: y }, renderer.get_drawables());
                    match closest_line {
                        Some(closest_line) => {
                            let mut drawable = math::perpendicular_line(standing_point, closest_line);
                            drawable.color = Color {
                                r: red,
                                g: green,
                                b: blue,
                            };
                            drawable.width = width;
                            renderer.add_drawable(drawable);
                            "Perpendicular line added".to_string()
                        }
                        None => "No line found".to_string(),
                    }
                }
                _ => String::new(),
            };
            let d = renderer.get_drawables();

            ui.set_contextual_text(SharedString::from(contextual_text.as_str()));
            ui.set_map(renderer.render_image());
            ui.set_current_action(next_action);
            let mut my_vec = vec![];

            for dd in d {
                let s = format!("{} - {:?}", dd.id, dd.object_type);
                let s = slint::StandardListViewItem::from(slint::SharedString::from(s.as_str()));
                my_vec.push(s);
            }
            let model = slint::ModelRc::new(VecModel::from(my_vec));
            ui.set_item_list(model);
        }
    });

    let _thread = std::thread::spawn(move || {
        log::debug!("Entering thread");
        let handle_copy = ui_handle.clone();

        let _ = slint::invoke_from_event_loop(move || {
            log::debug!("Entering invoke_from_event_loop");
            
            let mut renderer = Renderer::new();
            let image = renderer.render_image();
            let ui_local_handle = handle_copy.unwrap();

            ui_local_handle.set_map(image);
        });
    });

    ui.run()
}
