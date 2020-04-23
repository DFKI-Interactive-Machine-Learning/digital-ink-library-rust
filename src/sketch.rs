use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde;
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::json;

use super::stroke;

#[derive(Debug, Serialize)]
pub struct Sketch {
    #[serde(rename = "type")]
    pub typ: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub strokes: Vec<stroke::Stroke>,
}

impl Sketch {
    pub fn new(strokes: Vec<stroke::Stroke>) -> Sketch {
        Sketch { typ: String::from("stroke"), meta: HashMap::new(), strokes }
    }

    pub fn strokes(&self) -> &Vec<stroke::Stroke> {
        &self.strokes
    }

    pub fn add_stroke(&mut self, stroke: stroke::Stroke) {
        self.strokes.push(stroke);
    }

    pub fn x_min(&self) -> f64 {
        let mut x_min: f64 = std::f64::MAX;

        for stroke in self.strokes.iter() {
            let stroke_min = stroke.x_min();
            if stroke_min < x_min {
                x_min = stroke_min;
            }
        }

        x_min
    }

    pub fn x_max(&self) -> f64 {
        let mut x_max: f64 = std::f64::MIN;

        for stroke in self.strokes.iter() {
            let stroke_max = stroke.x_max();
            if stroke_max > x_max {
                x_max = stroke_max;
            }
        }

        x_max
    }

    pub fn y_min(&self) -> f64 {
        let mut y_min: f64 = std::f64::MAX;

        for stroke in self.strokes.iter() {
            let stroke_min = stroke.y_min();
            if stroke_min < y_min {
                y_min = stroke_min;
            }
        }

        y_min
    }

    pub fn y_max(&self) -> f64 {
        let mut y_max: f64 = std::f64::MIN;

        for stroke in self.strokes.iter() {
            let stroke_max = stroke.y_max();
            if stroke_max > y_max {
                y_max = stroke_max;
            }
        }

        y_max
    }

    pub fn timestamp_min(&self) -> u64 {
        let mut timestamp_min: u64 = std::u64::MAX;

        for stroke in self.strokes.iter() {
            let stroke_min = stroke.timestamp_min();
            if stroke_min < timestamp_min {
                timestamp_min = stroke_min;
            }
        }

        timestamp_min
    }

    pub fn timestamp_max(&self) -> u64 {
        let mut timestamp_max: u64 = std::u64::MIN;

        for stroke in self.strokes.iter() {
            let stroke_max = stroke.timestamp_max();
            if stroke_max > timestamp_max {
                timestamp_max = stroke_max;
            }
        }

        timestamp_max
    }

    pub fn pressure_min(&self) -> f64 {
        let mut pressure_min: f64 = std::f64::MAX;

        for stroke in self.strokes.iter() {
            let stroke_min = stroke.pressure_min();
            if stroke_min < pressure_min {
                pressure_min = stroke_min;
            }
        }

        pressure_min
    }

    pub fn pressure_max(&self) -> f64 {
        let mut pressure_max: f64 = std::f64::MIN;

        for stroke in self.strokes.iter() {
            let stroke_max = stroke.pressure_max();
            if stroke_max > pressure_max {
                pressure_max = stroke_max;
            }
        }

        pressure_max
    }

    pub fn len(&self) -> usize {
        self.strokes.len()
    }

    /// Offset the x/y coordinates by a given offset
    pub fn offset(&mut self, x_offset: Option<f64>, y_offset: Option<f64>) {
        for stroke in self.strokes.iter_mut() {
            stroke.offset(x_offset, y_offset);
        }
    }

    /// Scale the x/y coordinates by a given factor
    pub fn scale(&mut self, x_factor: Option<f64>, y_factor: Option<f64>) {
        for stroke in self.strokes.iter_mut() {
            stroke.scale(x_factor, y_factor);
        }
    }

    /// Normalize the stroke into a maximum dimension
    pub fn normalize(&mut self, new_size: f64, keep_aspect_ratio: bool) {
        self.offset(Some(- self.x_min()), Some(-self.y_min()));
        let x_factor = Some(new_size / self.x_max());
        let y_factor = Some(new_size / self.y_max());

        if keep_aspect_ratio && self.x_max() >= self.y_max() {
            self.scale(x_factor, x_factor);
        } else if keep_aspect_ratio && self.x_max() < self.y_max() {
            self.scale(y_factor, y_factor);
        } else {
            self.scale(x_factor, y_factor);
        }
    }

    pub fn remove_duplicate_dots(&mut self) {
        for stroke in self.strokes.iter_mut() {
            stroke.remove_duplicate_dots();
        }
    }

    pub fn remove_single_dot_strokes(&mut self) {
        self.strokes.retain(|stroke| stroke.len() > 1);
    }
}

impl fmt::Display for Sketch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self))
    }
}

impl PartialEq for Sketch {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ &&
            self.meta == other.meta &&
            self.strokes == other.strokes
    }
}

impl Eq for Sketch {}


impl<'de> Deserialize<'de> for Sketch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Type, Meta, Strokes };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
                where
                    D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`type` or `meta` or `strokes`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "meta" => Ok(Field::Meta),
                            "strokes" => Ok(Field::Strokes),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SketchVisitor;

        impl<'de> Visitor<'de> for SketchVisitor {
            type Value = Sketch;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Sketch")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Sketch, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let typ = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let meta = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let strokes = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let mut sketch = Sketch::new(strokes);
                sketch.typ = typ;
                sketch.meta = meta;
                Ok(sketch)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Sketch, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut typ = None;
                let mut meta = None;
                let mut strokes = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if typ.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            typ = Some(map.next_value()?);
                        }
                        Field::Meta => {
                            if meta.is_some() {
                                return Err(de::Error::duplicate_field("meta"));
                            }
                            meta = Some(map.next_value()?);
                        }
                        Field::Strokes => {
                            if strokes.is_some() {
                                return Err(de::Error::duplicate_field("strokes"));
                            }
                            strokes = Some(map.next_value()?);
                        }
                    }
                }
                let typ = typ.ok_or_else(|| de::Error::missing_field("type"))?;
                let meta = meta.ok_or_else(|| de::Error::missing_field("meta"))?;
                let strokes = strokes.ok_or_else(|| de::Error::missing_field("strokes"))?;
                let mut sketch = Sketch::new(strokes);
                sketch.meta = meta;
                sketch.typ = typ;
                Ok(sketch)
            }
        }

        const FIELDS: &'static [&'static str] = &["type", "meta", "strokes"];
        deserializer.deserialize_struct("Sketch", FIELDS, SketchVisitor)
    }
}