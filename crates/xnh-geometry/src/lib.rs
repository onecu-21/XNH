use serde::{Deserialize, Serialize};
use xnh_core::Millimeters;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Geometry {
    Box(BoxGeometry),
    Cylinder(CylinderGeometry),
    Translate(TranslateGeometry),
    Rotate(RotateGeometry),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxGeometry {
    pub width: Millimeters,
    pub depth: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CylinderGeometry {
    pub radius: Millimeters,
    pub height: Millimeters,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Translation3 {
    pub x: Millimeters,
    pub y: Millimeters,
    pub z: Millimeters,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Direction3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TranslateGeometry {
    pub offset: Translation3,
    pub geometry: Box<Geometry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RotateGeometry {
    pub axis: Direction3,
    pub angle_degrees: f64,
    pub geometry: Box<Geometry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_serializes_with_explicit_type() {
        let geometry = Geometry::Box(BoxGeometry {
            width: Millimeters::new(120.0),
            depth: Millimeters::new(80.0),
            height: Millimeters::new(20.0),
        });

        let json = serde_json::to_value(geometry).expect("geometry should serialize");

        assert_eq!(json["type"], "box");
        assert_eq!(json["width"], 120.0);
    }
}
