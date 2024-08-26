mod math;
mod rendering;

use log::debug;
use slint::{SharedString, VecModel};

use rendering::{Color, DrawableType, Point, Renderer};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    env_logger::builder().format_timestamp_millis().init();

    let mut renderer = Renderer::new("data/chouette/989.jpg");
    let mut renderer_temp = renderer.clone();
    renderer.add_layer("data/chouette/500.png", 1000, 3000, 0.8);
    let mut standing_point = Point { x: 0, y: 0 };
    let mut standing_point_2 = Point { x: 0, y: 0 };
    let mut standing_drawable = None;
    let mut next_action = NextAction::None;

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut selected_listview_item = None;

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

            log::debug!("Current action: {:?}", ui.get_current_action());

            let contextual_text = match ui.get_current_action() {
                // Get closest object
                NextAction::None => {
                    let closest_object = math::closest_object(Point { x, y }, renderer.get_drawables());
                    match closest_object {
                        Some(object) => {
                            selected_listview_item = Some(object.id);
                            let output = format!("{:?}", object);
                            output
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Vertical line
                NextAction::Vertical => {
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        Point { x, y: 0 },
                        Point { x, y: 480 },
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
                NextAction::Horizontal => {
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        Point { x: 0, y },
                        Point { x: 640, y },
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
                NextAction::Point => {
                    renderer.add_drawable_by_values(
                        DrawableType::Point,
                        Point { x, y },
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
                NextAction::Segment => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Segment2;
                    "Click on the second segment point".to_string()
                }
                // Second segment point
                NextAction::Segment2 => {
                    renderer.add_drawable_by_values(
                        DrawableType::Segment,
                        standing_point,
                        Point { x, y },
                        Color {
                            r: red,
                            g: green,
                            b: blue,
                        },
                        width,
                    );
                    next_action = NextAction::None;
                    "Segment added".to_string()
                }
                // First angle computation line
                NextAction::MeasureAngle => {
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
                    match closest_line {
                        Some(line) => {
                            standing_drawable = Some(line);
                            next_action = NextAction::MeasureAngle2;
                            "Click on the second line".to_string()
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Second angle computation line
                NextAction::MeasureAngle2 => {
                    next_action = NextAction::None;
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
                    match closest_line {
                        Some(line) => {
                            let line1 = standing_drawable.unwrap();
                            let angle = math::angle_between(line1, line);
                            let output: String = format!(
                                "The angle between {} and {} is {:.2}° / {:.2}°",
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
                NextAction::Line => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Line2;
                    "Click on the second line point".to_string()
                }
                // Second line point
                NextAction::Line2 => {
                    next_action = NextAction::None;
                    renderer.add_drawable_by_values(
                        DrawableType::Line,
                        standing_point,
                        Point { x, y },
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
                NextAction::HalfLine => {
                    standing_point = Point { x, y };
                    next_action = NextAction::HalfLine2;
                    "Click on the half broken line point".to_string()
                }
                // Second half line point
                NextAction::HalfLine2 => {
                    next_action = NextAction::None;
                    renderer.add_drawable_by_values(
                        DrawableType::HalfLine,
                        standing_point,
                        Point { x, y },
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
                NextAction::CenterAndEdge => {
                    standing_point = Point { x, y };
                    next_action = NextAction::CenterAndEdge2;
                    "Click on any point on the circle".to_string()
                }
                // Second circle point
                NextAction::CenterAndEdge2 => {
                    next_action = NextAction::None;
                    renderer.add_drawable_by_values(
                        DrawableType::Circle,
                        standing_point,
                        Point { x, y },
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
                NextAction::Delete => {
                    let closest_line =
                        math::closest_object(Point { x, y }, renderer.get_drawables());
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
                NextAction::MeasureTwoPoints => {
                    standing_point = Point { x, y };
                    next_action = NextAction::MeasureTwoPoints2;
                    "Click on the second point".to_string()
                }
                // Second point to measure distance
                NextAction::MeasureTwoPoints2 => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let distance = math::distance(standing_point, Point { x, y });
                    debug!("Distance: {} px", distance);
                    format!(
                        "Distance beetwen two points is {:.2} km or {:.1} px",
                        distance * m_per_px / 1000., distance
                    )
                }
                // Point to measure distance to line
                NextAction::MeasurePointToLine => {
                    standing_point = Point { x, y };
                    next_action = NextAction::MeasurePointToLine2;
                    "Click on the second point".to_string()
                }
                // Line to measure distance
                NextAction::MeasurePointToLine2 => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
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
                NextAction::MeasureRadius => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let closest_circle =
                        math::closest_circle(Point { x, y }, renderer.get_drawables());
                    match closest_circle {
                        Some(circle) => {
                            let distance = math::distance(circle.point1, circle.point2);
                            format!("The radius is {:.2} km", distance * m_per_px / 1000.)
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Point the parallel line will go through
                NextAction::Parallel => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Parallel2;
                    "Click on a line".to_string()
                }
                // Line of reference for the parallel line
                NextAction::Parallel2 => {
                    next_action = NextAction::None;
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
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
                NextAction::TwoPointsMedian => {
                    standing_point = Point { x, y };
                    next_action = NextAction::TwoPointsMedian2;
                    "Click on the second point".to_string()
                }
                // Second point for median
                NextAction::TwoPointsMedian2 => {
                    next_action = NextAction::None;
                    let mut drawable = math::median_line(standing_point, Point { x, y });
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
                NextAction::Perpendicular => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Perpendicular2;
                    "Click on a line".to_string()
                }
                // Line of reference for the perpendicular line
                NextAction::Perpendicular2 => {
                    next_action = NextAction::None;
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
                    match closest_line {
                        Some(closest_line) => {
                            let mut drawable =
                                math::perpendicular_line(standing_point, closest_line);
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
                // Point the new lines will go through, computed from given angle
                NextAction::FromAngle => {
                    standing_point = Point { x, y };
                    next_action = NextAction::FromAngle2;
                    "Click on a reference line".to_string()
                }
                // Line of reference for the new lines, computed from given angle
                NextAction::FromAngle2 => {
                    next_action = NextAction::None;
                    let closest_line = math::closest_line(Point { x, y }, renderer.get_drawables());
                    match closest_line {
                        Some(closest_line) => {
                            let drawable =
                                math::parallel_line(standing_point, closest_line);

                            let (mut line1, mut line2) = math::get_lines_from_angles(drawable, standing_point, ui.get_angle());

                            let color = Color {
                                r: red,
                                g: green,
                                b: blue,
                            };

                            line1.color = color;
                            line1.width = width;
                            renderer.add_drawable(line1);
                            line2.color = color;
                            line2.width = width;
                            renderer.add_drawable(line2);
                            "Lines added".to_string()
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Point the tangent linse will go through
                NextAction::Tangent => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Tangent2;
                    "Click on a line".to_string()
                }
                // Circle of reference for the tangent lines
                NextAction::Tangent2 => {
                    next_action = NextAction::None;
                    let closest_circle = math::closest_circle(Point { x, y }, renderer.get_drawables());
                    match closest_circle {
                        Some(closest_circle) => {
                            if let Some((mut tangent1, mut tangent2)) =
                                math::tangent_lines_to_circle(standing_point, closest_circle) {                            
                                    let color = Color {
                                        r: red,
                                        g: green,
                                        b: blue,
                                    };
        
                                    tangent1.color = color;
                                    tangent1.width = width;
                                    renderer.add_drawable(tangent1);
                                    tangent2.color = color;
                                    tangent2.width = width;
                                    renderer.add_drawable(tangent2);
                                    "Tangent lines added".to_string()
                                }
                                else {
                                    "Selected point is inside the circle".to_string()
                                }
                            
                        }
                        None => "No line found".to_string(),
                    }
                }
                // Circle with center and radius in km
                NextAction::CircleRadiusLength => {
                    next_action = NextAction::None;
                    let radius = (ui.get_radius() / ui.get_m_per_px() * 1_000.) as i32;
                    renderer.add_drawable_by_values(DrawableType::Circle, Point { x, y }, Point { x: x + radius, y }, Color {
                        r: red,
                        g: green,
                        b: blue,
                    }, width);
                    "Circle added".to_string()
                }
                // First point for three point circle
                NextAction::CircleThreeEdgePoints => {
                    standing_point = Point { x, y };
                    next_action = NextAction::CircleThreeEdgePoints2;
                    "Click on the second point".to_string()
                }
                // Second point for three point circle
                NextAction::CircleThreeEdgePoints2 => {
                    standing_point_2 = Point { x, y };
                    next_action = NextAction::CircleThreeEdgePoints3;
                    "Click on the third point".to_string()
                }
                // Add the three point circle
                NextAction::CircleThreeEdgePoints3 => {
                    next_action = NextAction::None;
                    let mut circle = math::circle_from_three_points(standing_point, standing_point_2, Point { x, y });
                    circle.color = Color {
                        r: red,
                        g: green,
                        b: blue,
                    };
                    circle.width = width;
                    renderer.add_drawable(circle);
                    "Circle added".to_string()
                }
            };
            let d = renderer.get_drawables();

            ui.set_contextual_text(SharedString::from(contextual_text.as_str()));
            if let Some(image) = renderer.render_overlay(ui.get_viewport_zoom()) {
                ui.set_overlay_image(image);
            }
            ui.set_current_action(next_action);
            let mut my_vec = vec![];

            for dd in d {
                let s = format!("{} - {:?}", dd.id, dd.object_type);
                let s = slint::StandardListViewItem::from(slint::SharedString::from(s.as_str()));
                my_vec.push(s);
                renderer.set_listview_id(dd.id, my_vec.len() as i32 - 1);
                if selected_listview_item == Some(dd.id) {
                    ui.set_current_listview_drawable_item(my_vec.len() as i32 - 1);
                }
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

            let ui_local_handle = handle_copy.unwrap();
            if let Some(image) = renderer_temp.render_background() {
                ui_local_handle.set_map(image);
            }
        });
    });

    ui.run()
}
