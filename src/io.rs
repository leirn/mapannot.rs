use std::io::{Read, Error};
use std::fs::File;
use std::convert::From;

use crate::{math::Point, rendering::overlay::{Color, DrawableType, Drawable}};
use crate::LayerDrawable;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProjectLayer {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub m_per_px: f32,
    pub transparency: f32,
    pub file: String,
}

impl From<LayerDrawable> for ProjectLayer {
    fn from(layer: LayerDrawable) -> ProjectLayer {
        ProjectLayer {
            id: layer.id,
            x: layer.x,
            y: layer.y,
            m_per_px: layer.m_per_px,
            transparency: layer.transparency,
            file: layer.file.clone().to_string(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ProjectDrawable {
    pub id: i32,
    pub object_type: DrawableType,
    pub point1: Point,
    pub point2: Point,
    pub color: Color,
    pub width: f32,
}

impl From<Drawable> for ProjectDrawable {
    fn from(layer: Drawable) -> ProjectDrawable {
        ProjectDrawable {
            id: layer.id,
            object_type: layer.object_type,
            point1: layer.point1,
            point2: layer.point2,
            color: layer.color,
            width: layer.width,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub layers: Vec<ProjectLayer>,
    pub background: String,
    pub drawables: Vec<ProjectDrawable>,
}

impl Project {
    pub fn new(background: &str, layers: &Vec<LayerDrawable>, drawables: &Vec<Drawable>) -> Project {
        Project {
            layers: layers.iter().map(|layer| ProjectLayer::from(layer.clone())).collect(),
            background: String::from(background),
            drawables: drawables.iter().map(|drawable| ProjectDrawable::from(drawable.clone())).collect(),
        }
    }

    pub fn load_project(file: &str) -> Result<Project, Error> {
        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let project: Project = serde_json::from_str(&contents)?;
        Ok(project)
    }
    
    pub fn save_project(&mut self, file: &str) -> Result<(), Error> {
        let file = File::create(file)?;
        let file = std::io::BufWriter::new(file);
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}