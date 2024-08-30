mod fileselector;
mod math;
mod rendering;
mod utils;
mod io;

use std::{
    cell::RefCell, path::PathBuf, process::exit, rc::Rc
};

use log::debug;
use math::Point;
use slint::{Model, SharedString, StandardListViewItem, VecModel};

use rendering::{
    background::BackgroundRenderer,
    layer::LayerRenderer,
    overlay::{Circle, OverlayRenderer},
};

use std::env::current_dir;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    env_logger::builder().format_timestamp_millis().init();

    let renderer = Rc::new(RefCell::new(OverlayRenderer::new(1, 1)));

    let layer_renderer = Rc::new(RefCell::new(LayerRenderer::new()));

    let layer_renderer2 = layer_renderer.clone();
    let layer_renderer3 = layer_renderer.clone();
    let overlay = renderer.clone();

    let mut standing_point = Point { x: 0, y: 0 };
    let mut standing_point_2 = Point { x: 0, y: 0 };
    let mut standing_drawable = None;
    let mut next_action = NextAction::None;

    let ui: AppWindow = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    let ui_handle_maximised = ui_handle.clone();
    let ui_handle3 = ui_handle.clone();

    let mut selected_listview_item = None;



    ui.on_load(|| {
        debug!("Stub to save project into file");
        let project = io::Project::load_project("file.mrs").unwrap();
    });

    ui.on_save(|| {
        debug!("Stub to load project from file");
        let mut project = io::Project::new("background.png", vec![], vec![]);
        project.save_project("file.mrs").unwrap();
    });

    ui.on_close(|| {
        debug!("Terminate application");
        exit(0);
    });

    ui.on_show_fileselector_bg(move || {
        log::debug!("Entering on_show_fileselector");
        let file_selector_bg = FileSelector::new().unwrap();
        file_selector_bg.show().unwrap();
        let file_selector_weak = file_selector_bg.as_weak();

        if file_selector_bg.get_path().is_empty() {
            file_selector_bg.set_path(SharedString::from(current_dir().unwrap().to_str().unwrap()));
        }

        let path = file_selector_bg.get_path().to_string();

        let folders = fileselector::get_slint_folders_from_folder(&path);

        file_selector_bg.set_folders(folders);

        let files = fileselector::get_slint_files_from_folder(&path);
        file_selector_bg.set_files(files);

        file_selector_bg.on_send_ok({
            let ui_fs = file_selector_weak.unwrap();
            let ui = ui_handle3.unwrap();
            let overlay = overlay.clone();
            let layer = layer_renderer3.clone();
            move || {
                let parent_path = ui_fs.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let file = ui_fs.get_filename().to_string();
                let image_path = parent_path.join(file);

                let mut renderer_bg = BackgroundRenderer::new(image_path.to_str().unwrap());

                layer.borrow_mut().reset();
                overlay
                    .borrow_mut()
                    .reset(renderer_bg.image_height, renderer_bg.image_width);

                ui.set_map(renderer_bg.render_background().unwrap());

                ui_fs.hide().unwrap();
            }
        });

        file_selector_bg.on_send_cancel({
            let ui_fs = file_selector_weak.unwrap();
            move || {
                ui_fs.hide().unwrap();
            }
        });

        file_selector_bg.on_set_folder({
            let ui_fs = file_selector_weak.unwrap();
            move || {
                let parent_path = ui_fs.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let child_path = ui_fs.get_current_folder().to_string();

                let parent_path = parent_path.join(child_path);
                let parent_path = parent_path.canonicalize().unwrap();

                let parent_path = parent_path.to_str().unwrap();

                ui_fs.set_path(SharedString::from(parent_path));

                let files_bg = fileselector::get_slint_files_from_folder(parent_path);
                ui_fs.set_files(files_bg);

                let folders_bg = fileselector::get_slint_folders_from_folder(parent_path);
                ui_fs.set_folders(folders_bg);
            }
        });

        file_selector_bg.on_load_preview({
            let ui_fs = file_selector_weak.clone();
            move || {
                let ui = ui_fs.unwrap();
                let parent_path = ui.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let file = ui.get_filename().to_string();
                let image_path = parent_path.join(file);
                log::debug!("Entering clossure");
                let ui_fs = ui_fs.clone();
                let _thread = std::thread::spawn(move || {
                    log::debug!("Entering thread");
                    let _ = slint::invoke_from_event_loop(move || {
                        log::debug!("Entering invoke_from_event_loop");
                        let ui = ui_fs.unwrap();
                        log::debug!("Doing it");
                        match slint::Image::load_from_path(image_path.as_path()) {
                            Ok(image) => {
                                ui.set_preview(image);
                                log::debug!("Loading preview image")
                            }
                            Err(e) => {
                                log::warn!("Error loading image: {:?}", e);
                            }
                        }
                    });
                });
            }
        });
    });

    ui.on_show_fileselector(move || {
        log::debug!("Entering on_show_fileselector");
        let file_selector = FileSelector::new().unwrap();
        file_selector.show().unwrap();
        let file_selector_weak = file_selector.as_weak();

        if file_selector.get_path().is_empty() {
            file_selector.set_path(SharedString::from(current_dir().unwrap().to_str().unwrap()));
        }

        let path = file_selector.get_path().to_string();

        let folders = fileselector::get_slint_folders_from_folder(&path);

        file_selector.set_folders(folders);

        let files = fileselector::get_slint_files_from_folder(&path);
        file_selector.set_files(files);

        file_selector.on_send_ok({
            let ui_fs = file_selector_weak.unwrap();
            let ui = ui_handle.unwrap();
            let layer_renderer3 = layer_renderer2.clone();
            move || {
                let parent_path = ui_fs.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let file = ui_fs.get_filename().to_string();
                let image_path = parent_path.join(file);
                let m_per_px = ui.get_m_per_px();
                layer_renderer3.borrow_mut().add_layer(
                    image_path.to_str().unwrap(),
                    0,
                    0,
                    1.,
                    m_per_px,
                );

                let items = VecModel::from(
                    layer_renderer3
                        .borrow()
                        .layers
                        .iter()
                        .map(|layer| LayerDrawable {
                            id: layer.id,
                            data: layer.data.clone(),
                            x: layer.x,
                            y: layer.y,
                            transparency: layer.transparency,
                            m_per_px: layer.m_per_px,
                            file: layer.file.clone(),
                            name: layer.name.clone(),
                        })
                        .collect::<Vec<LayerDrawable>>(),
                );
                debug!("Layer items count: {}", items.row_count());
                ui.set_layers(slint::ModelRc::new(items));

                let layers_list = slint::VecModel::from(
                    layer_renderer3
                        .borrow()
                        .layers
                        .iter()
                        .map(|layer| {
                            slint::StandardListViewItem::from(slint::SharedString::from(
                                layer.name.as_str(),
                            ))
                        })
                        .collect::<Vec<StandardListViewItem>>(),
                );

                ui.set_layers_list(slint::ModelRc::new(layers_list));

                ui_fs.hide().unwrap();
            }
        });

        file_selector.on_send_cancel({
            let ui_fs = file_selector_weak.unwrap();
            move || {
                ui_fs.hide().unwrap();
            }
        });

        file_selector.on_set_folder({
            let ui_fs = file_selector_weak.unwrap();
            move || {
                let parent_path = ui_fs.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let child_path = ui_fs.get_current_folder().to_string();

                let parent_path = parent_path.join(child_path);
                let parent_path = parent_path.canonicalize().unwrap();

                let parent_path = parent_path.to_str().unwrap();

                ui_fs.set_path(SharedString::from(parent_path));

                let files = fileselector::get_slint_files_from_folder(parent_path);
                ui_fs.set_files(files);

                let folders = fileselector::get_slint_folders_from_folder(parent_path);
                ui_fs.set_folders(folders);
            }
        });

        file_selector.on_load_preview({
            let ui_fs = file_selector_weak.clone();
            move || {
                let ui = ui_fs.unwrap();
                let parent_path = ui.get_path().to_string();
                let parent_path = PathBuf::from(&parent_path);
                let file = ui.get_filename().to_string();
                let image_path = parent_path.join(file);
                log::debug!("Entering clossure");
                let ui_fs = ui_fs.clone();
                let _thread = std::thread::spawn(move || {
                    log::debug!("Entering thread");
                    let _ = slint::invoke_from_event_loop(move || {
                        log::debug!("Entering invoke_from_event_loop");
                        let ui = ui_fs.unwrap();
                        log::debug!("Doing it");
                        match slint::Image::load_from_path(image_path.as_path()) {
                            Ok(image) => {
                                ui.set_preview(image);
                                log::debug!("Loading preview image")
                            }
                            Err(e) => {
                                log::warn!("Error loading image: {:?}", e);
                            }
                        }
                    });
                });
            }
        });
    });

    ui.on_image_click({
        log::debug!("Entering on_image_click");
        let ui_handle = ui.as_weak();
        let ui = ui_handle.unwrap();

        let layer_renderer4 = layer_renderer.clone();

        move || {
            let x = ui.get_mouse_x();
            let y = ui.get_mouse_y();
            let red = ui.get_stroke_red().round() as u8;
            let green = ui.get_stroke_green().round() as u8;
            let blue = ui.get_stroke_blue().round() as u8;
            let width = ui.get_stroke_width();

            renderer.borrow_mut().set_width(width);
            renderer.borrow_mut().set_color(red, green, blue);

            log::debug!("Mouse position = {x}, {y}");

            log::debug!("Current action: {:?}", ui.get_current_action());

            let contextual_text = match ui.get_current_action() {
                NextAction::None => None,
                // Get closest object
                NextAction::UpdateSelectedItem => {
                    selected_listview_item = Some(ui.get_current_listview_drawable_item());
                    renderer.borrow_mut().discard_overlay();
                    None
                }
                NextAction::SelectObject => {
                    let closest_object = renderer.borrow().closest_object(Point { x, y });
                    match closest_object {
                        Some(object) => {
                            renderer.borrow_mut().discard_overlay();
                            selected_listview_item = Some(object.id);
                            Some(format!("{:?}", object))
                        }
                        None => {
                            selected_listview_item = None;
                            Some("No line found".to_string())
                        }
                    }
                }
                // Vertical line
                NextAction::Vertical => {
                    renderer
                        .borrow_mut()
                        .add_line(Point { x, y: 0 }, Point { x, y: 480 });
                    Some("Vertical line added".to_string())
                }
                // Horizontal line
                NextAction::Horizontal => {
                    renderer
                        .borrow_mut()
                        .add_line(Point { x: 0, y }, Point { x: 640, y });
                    Some("Horizontal line added".to_string())
                }
                // Point
                NextAction::Point => {
                    renderer.borrow_mut().add_point(Point { x, y });
                    Some("Point added".to_string())
                }
                // First segment point
                NextAction::Segment => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Segment2;
                    Some("Click on the second segment point".to_string())
                }
                // Second segment point
                NextAction::Segment2 => {
                    renderer
                        .borrow_mut()
                        .add_segment(standing_point, Point { x, y });
                    next_action = NextAction::None;
                    Some("Segment added".to_string())
                }
                // First angle computation line
                NextAction::MeasureAngle => {
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(line) => {
                            standing_drawable = Some(line);
                            next_action = NextAction::MeasureAngle2;
                            Some("Click on the second line".to_string())
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Second angle computation line
                NextAction::MeasureAngle2 => {
                    next_action = NextAction::None;
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(line) => {
                            let line1 = standing_drawable.unwrap();
                            let angle = math::angle_between(
                                line1.point1,
                                line1.point2,
                                line.point1,
                                line.point2,
                            );
                            Some(format!(
                                "The angle between {} and {} is {:.2}° / {:.2}°",
                                line1.id,
                                line.id,
                                angle.to_degrees().abs(),
                                180. - angle.to_degrees().abs()
                            ))
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // First line point
                NextAction::Line => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Line2;
                    Some("Click on the second line point".to_string())
                }
                // Second line point
                NextAction::Line2 => {
                    next_action = NextAction::None;
                    renderer
                        .borrow_mut()
                        .add_line(standing_point, Point { x, y });
                    Some("Line added".to_string())
                }
                // First half line point
                NextAction::HalfLine => {
                    standing_point = Point { x, y };
                    next_action = NextAction::HalfLine2;
                    Some("Click on the half broken line point".to_string())
                }
                // Second half line point
                NextAction::HalfLine2 => {
                    next_action = NextAction::None;
                    renderer
                        .borrow_mut()
                        .add_half_line(standing_point, Point { x, y });
                    Some("Half line added".to_string())
                }
                // Circle center
                NextAction::CenterAndEdge => {
                    standing_point = Point { x, y };
                    next_action = NextAction::CenterAndEdge2;
                    Some("Click on any point on the circle".to_string())
                }
                // Second circle point
                NextAction::CenterAndEdge2 => {
                    next_action = NextAction::None;
                    renderer.borrow_mut().add_circle(
                        standing_point,
                        math::distance(standing_point, Point { x, y }),
                    );
                    Some("Circle added".to_string())
                }
                // Delete object
                NextAction::Delete => {
                    let closest_line = renderer.borrow().closest_object(Point { x, y });
                    match closest_line {
                        Some(line) => {
                            // Remove line from drawables based on id
                            renderer.borrow_mut().remove_drawable(line.id);
                            Some("Object deleted".to_string())
                        }
                        None => Some("No object found".to_string()),
                    }
                }
                // First point to measure distance
                NextAction::MeasureTwoPoints => {
                    standing_point = Point { x, y };
                    next_action = NextAction::MeasureTwoPoints2;
                    Some("Click on the second point".to_string())
                }
                // Second point to measure distance
                NextAction::MeasureTwoPoints2 => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let distance = math::distance(standing_point, Point { x, y });
                    debug!("Distance: {} px", distance);
                    Some(format!(
                        "Distance beetwen two points is {:.2} km or {:.1} px",
                        distance * m_per_px / 1000.,
                        distance
                    ))
                }
                // Point to measure distance to line
                NextAction::MeasurePointToLine => {
                    standing_point = Point { x, y };
                    next_action = NextAction::MeasurePointToLine2;
                    Some("Click on the second point".to_string())
                }
                // Line to measure distance
                NextAction::MeasurePointToLine2 => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(line) => {
                            let distance = math::perpendicular_distance(
                                standing_point,
                                line.point1,
                                line.point2,
                            );
                            Some(format!(
                                "Distance beetwen two points is {:.2} km",
                                distance * m_per_px / 1000.
                            ))
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Circle radius
                NextAction::MeasureRadius => {
                    next_action = NextAction::None;
                    let m_per_px = ui.get_m_per_px();
                    let closest_circle = renderer.borrow().closest_circle(Point { x, y });
                    match closest_circle {
                        Some(circle) => {
                            let distance = math::distance(circle.point1, circle.point2);
                            Some(format!(
                                "The radius is {:.2} km",
                                distance * m_per_px / 1000.
                            ))
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Point the parallel line will go through
                NextAction::Parallel => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Parallel2;
                    Some("Click on a line".to_string())
                }
                // Line of reference for the parallel line
                NextAction::Parallel2 => {
                    next_action = NextAction::None;
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(closest_line) => {
                            let (point1, point2) = math::parallel_line(
                                standing_point,
                                closest_line.point1,
                                closest_line.point2,
                            );
                            renderer.borrow_mut().add_line(point1, point2);
                            Some("Parallel line added".to_string())
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // First point for median
                NextAction::TwoPointsMedian => {
                    standing_point = Point { x, y };
                    next_action = NextAction::TwoPointsMedian2;
                    Some("Click on the second point".to_string())
                }
                // Second point for median
                NextAction::TwoPointsMedian2 => {
                    next_action = NextAction::None;
                    let (point1, point2) = math::median_line(standing_point, Point { x, y });
                    renderer.borrow_mut().add_line(point1, point2);
                    Some("Median line added".to_string())
                }
                // Point the perpendicular line will go through
                NextAction::Perpendicular => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Perpendicular2;
                    Some("Click on a line".to_string())
                }
                // Line of reference for the perpendicular line
                NextAction::Perpendicular2 => {
                    next_action = NextAction::None;
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(closest_line) => {
                            let (point1, point2) = math::perpendicular_line(
                                standing_point,
                                closest_line.point1,
                                closest_line.point2,
                            );
                            renderer.borrow_mut().add_line(point1, point2);
                            Some("Perpendicular line added".to_string())
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Point the new lines will go through, computed from given angle
                NextAction::FromAngle => {
                    standing_point = Point { x, y };
                    next_action = NextAction::FromAngle2;
                    Some("Click on a reference line".to_string())
                }
                // Line of reference for the new lines, computed from given angle
                NextAction::FromAngle2 => {
                    next_action = NextAction::None;
                    let closest_line = renderer.borrow().closest_line(Point { x, y });
                    match closest_line {
                        Some(closest_line) => {
                            let (point1, point2) = math::parallel_line(
                                standing_point,
                                closest_line.point1,
                                closest_line.point2,
                            );

                            let ((point1, point2), (point3, point4)) = math::get_lines_from_angles(
                                point1,
                                point2,
                                standing_point,
                                ui.get_angle(),
                            );
                            renderer.borrow_mut().add_line(point1, point2);
                            renderer.borrow_mut().add_line(point3, point4);
                            Some("Lines added".to_string())
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Point the tangent linse will go through
                NextAction::Tangent => {
                    standing_point = Point { x, y };
                    next_action = NextAction::Tangent2;
                    Some("Click on a line".to_string())
                }
                // Circle of reference for the tangent lines
                NextAction::Tangent2 => {
                    next_action = NextAction::None;
                    let closest_circle = renderer.borrow().closest_circle(Point { x, y });
                    match closest_circle {
                        Some(closest_circle) => {
                            if let Some(((point1, point2), (point3, point4))) =
                                math::tangent_lines_to_circle(
                                    standing_point,
                                    closest_circle.center(),
                                    closest_circle.radius(),
                                )
                            {
                                renderer.borrow_mut().add_line(point1, point2);
                                renderer.borrow_mut().add_line(point3, point4);
                                Some("Tangent lines added".to_string())
                            } else {
                                Some("Selected point is inside the circle".to_string())
                            }
                        }
                        None => Some("No line found".to_string()),
                    }
                }
                // Circle with center and radius in km
                NextAction::CircleRadiusLength => {
                    next_action = NextAction::None;
                    let radius = (ui.get_radius() / ui.get_m_per_px() * 1_000.) as i32;
                    renderer
                        .borrow_mut()
                        .add_circle(Point { x, y }, radius as f32);
                    Some("Circle added".to_string())
                }
                // First point for three point circle
                NextAction::CircleThreeEdgePoints => {
                    standing_point = Point { x, y };
                    next_action = NextAction::CircleThreeEdgePoints2;
                    Some("Click on the second point".to_string())
                }
                // Second point for three point circle
                NextAction::CircleThreeEdgePoints2 => {
                    standing_point_2 = Point { x, y };
                    next_action = NextAction::CircleThreeEdgePoints3;
                    Some("Click on the third point".to_string())
                }
                // Add the three point circle
                NextAction::CircleThreeEdgePoints3 => {
                    next_action = NextAction::None;
                    let (center, radius) = math::circle_from_three_points(
                        standing_point,
                        standing_point_2,
                        Point { x, y },
                    );
                    renderer.borrow_mut().add_circle(center, radius);
                    Some("Circle added".to_string())
                }
            };

            let d = renderer.borrow().get_drawables();

            let mut my_vec = vec![];
            for dd in d {
                let s = format!("{} - {:?}", dd.id, dd.object_type);
                let s = slint::StandardListViewItem::from(slint::SharedString::from(s.as_str()));
                my_vec.push(s);
                renderer
                    .borrow_mut()
                    .set_listview_id(dd.id, my_vec.len() as i32 - 1);
                if selected_listview_item.is_some() && selected_listview_item.unwrap() == dd.id {
                    ui.set_current_listview_drawable_item(my_vec.len() as i32 - 1);
                }
            }
            let model = slint::ModelRc::new(VecModel::from(my_vec));
            ui.set_item_list(model);

            if contextual_text.is_some() {
                ui.set_contextual_text(SharedString::from(contextual_text.unwrap().as_str()));
            }
            ui.set_current_action(next_action);

            let items = VecModel::from(
                renderer
                    .borrow()
                    .drawable_images
                    .iter()
                    .map(|d| OverlayDrawable {
                        id: d.id,
                        data: d.data.clone(),
                        x: d.x,
                        y: d.y,
                    })
                    .collect::<Vec<OverlayDrawable>>(),
            );
            debug!("Overlay items count: {}", items.row_count());
            ui.set_overlay_drawables(slint::ModelRc::new(items));

            let items = VecModel::from(
                layer_renderer4
                    .borrow()
                    .layers
                    .iter()
                    .map(|layer| LayerDrawable {
                        id: layer.id,
                        data: layer.data.clone(),
                        x: layer.x,
                        y: layer.y,
                        transparency: layer.transparency,
                        m_per_px: layer.m_per_px,
                        file: layer.file.clone(),
                        name: layer.name.clone(),
                    })
                    .collect::<Vec<LayerDrawable>>(),
            );
            debug!("Layer items count: {}", items.row_count());
            ui.set_layers(slint::ModelRc::new(items));

            let layers_list = slint::VecModel::from(
                layer_renderer4
                    .borrow()
                    .layers
                    .iter()
                    .map(|layer| {
                        slint::StandardListViewItem::from(slint::SharedString::from(
                            layer.name.as_str(),
                        ))
                    })
                    .collect::<Vec<StandardListViewItem>>(),
            );

            ui.set_layers_list(slint::ModelRc::new(layers_list));
        }
    });

    let _thread = std::thread::spawn(move || {
        let app_copy = ui_handle_maximised.clone();
        //Expand the slint window from event loop
        slint::invoke_from_event_loop(move || app_copy.unwrap().window().set_maximized(true))
            .unwrap();

        //Another code that we wanted to execute after the application was launched
        //For example: hide the console window peculiar to slint
    });

    ui.run()
}
