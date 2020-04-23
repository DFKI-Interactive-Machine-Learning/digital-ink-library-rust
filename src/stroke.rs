use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::json;

#[derive(Clone, Debug, Serialize)]
pub struct Stroke {
    #[serde(rename = "type")]
    pub typ: String,
    pub meta: HashMap<String, serde_json::Value>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub timestamp: Vec<u64>,
    pub pressure: Vec<f64>,
}

impl Stroke {
    pub fn new(x: Vec<f64>, y: Vec<f64>, timestamp: Vec<u64>, pressure: Vec<f64>) -> Stroke {
        Stroke { typ: String::from("stroke"), meta: HashMap::new(), x, y, timestamp, pressure }
    }

    pub fn x(&self) -> &Vec<f64> {
        &self.x
    }

    pub fn y(&self) -> &Vec<f64> {
        &self.y
    }

    pub fn timestamp(&self) -> &Vec<u64> {
        &self.timestamp
    }

    pub fn pressure(&self) -> &Vec<f64> {
        &self.pressure
    }

    pub fn x_min(&self) -> f64 {
        self.x.iter().fold(std::f64::MAX, |a, &b| a.min(b))
    }

    pub fn x_max(&self) -> f64 {
        self.x.iter().fold(std::f64::MIN, |a, &b| a.max(b))
    }

    pub fn y_min(&self) -> f64 {
        self.y.iter().fold(std::f64::MAX, |a, &b| a.min(b))
    }

    pub fn y_max(&self) -> f64 {
        self.y.iter().fold(std::f64::MIN, |a, &b| a.max(b))
    }

    pub fn timestamp_min(&self) -> u64 {
        self.timestamp.iter().fold(std::u64::MAX, |a, &b| a.min(b))
    }

    pub fn timestamp_max(&self) -> u64 {
        self.timestamp.iter().fold(std::u64::MIN, |a, &b| a.max(b))
    }

    pub fn pressure_min(&self) -> f64 {
        self.pressure.iter().fold(std::f64::MAX, |a, &b| a.min(b))
    }

    pub fn pressure_max(&self) -> f64 {
        self.pressure.iter().fold(std::f64::MIN, |a, &b| a.max(b))
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }

    /// Offset the x/y coordinates by a given offset
    pub fn offset(&mut self, x_offset: Option<f64>, y_offset: Option<f64>) {
        let x_offset = x_offset.unwrap_or(0.);
        let y_offset = y_offset.unwrap_or(0.);

        if x_offset != 0. {
            for i in 0..self.x.len() {
                self.x[i] += x_offset;
            }
        }

        if y_offset != 0. {
            for i in 0..self.y.len() {
                self.y[i] += y_offset;
            }
        }
    }

    /// Scale the x/y coordinates by a given factor
    pub fn scale(&mut self, x_factor: Option<f64>, y_factor: Option<f64>) {
        let x_factor = x_factor.unwrap_or(1.);
        let y_factor = y_factor.unwrap_or(1.);

        if x_factor != 1. {
            for i in 0..self.x.len() {
                self.x[i] *= x_factor;
            }
        }

        if y_factor != 1. {
            for i in 0..self.y.len() {
                self.y[i] *= y_factor;
            }
        }
    }

    /// Removes successive dots with same coordinates
    pub fn remove_duplicate_dots(&mut self) {
        for i in (1..self.x.len()).rev() {
            let current_x = self.x[i];
            let current_y = self.y[i];
            let previous_x = self.x[i - 1];
            let previous_y = self.y[i - 1];

            if current_x == previous_x && current_y == previous_y {
                self.x.swap_remove(i);
                self.y.swap_remove(i);
                self.timestamp.swap_remove(i);
                self.pressure.swap_remove(i);
            }
        }
    }
}

impl fmt::Display for Stroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self))
    }
}


impl PartialEq for Stroke {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ &&
            self.meta == other.meta &&
            self.x == other.x &&
            self.y == other.y &&
            self.timestamp == other.timestamp &&
            self.pressure == other.pressure
    }
}

impl Eq for Stroke {}

pub struct StrokeBuilder {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub timestamp: Vec<u64>,
    pub pressure: Vec<f64>,
}

impl StrokeBuilder {
    pub fn new() -> StrokeBuilder {
        StrokeBuilder { x: vec![], y: vec![], timestamp: vec![], pressure: vec![] }
    }

    pub fn add_point(&mut self, x: f64, y: f64, timestamp: u64, pressure: f64) {
        self.x.push(x);
        self.y.push(y);
        self.timestamp.push(timestamp);
        self.pressure.push(pressure);
    }

    pub fn build(self) -> Stroke {
        Stroke { typ: String::from("stroke"), meta: HashMap::new(), x: self.x, y: self.y, timestamp: self.timestamp, pressure: self.pressure }
    }

    pub fn len(&self) -> usize {
        self.x.len()
    }
}


impl<'de> Deserialize<'de> for Stroke {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        enum Field { Type, Meta, X, Y, Timestamp, Pressure };

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
                        formatter.write_str("`type` or `meta` or `x` or `y` or `timestamp` or `pressure`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                        where
                            E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "meta" => Ok(Field::Meta),
                            "x" => Ok(Field::X),
                            "y" => Ok(Field::Y),
                            "timestamp" => Ok(Field::Timestamp),
                            "pressure" => Ok(Field::Pressure),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct StrokeVisitor;

        impl<'de> Visitor<'de> for StrokeVisitor {
            type Value = Stroke;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Stroke")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Stroke, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let typ = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let meta = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let x = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let y = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let timestamp = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let pressure = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let mut stroke = Stroke::new(x, y, timestamp, pressure);
                stroke.meta = meta;
                stroke.typ = typ;
                Ok(stroke)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Stroke, V::Error>
                where
                    V: MapAccess<'de>,
            {
                let mut typ = None;
                let mut meta = None;
                let mut x = None;
                let mut y = None;
                let mut timestamp = None;
                let mut pressure = None;
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
                        Field::X => {
                            if x.is_some() {
                                return Err(de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        Field::Y => {
                            if y.is_some() {
                                return Err(de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                        Field::Timestamp => {
                            if timestamp.is_some() {
                                return Err(de::Error::duplicate_field("timestamp"));
                            }
                            timestamp = Some(map.next_value()?);
                        }
                        Field::Pressure => {
                            if pressure.is_some() {
                                return Err(de::Error::duplicate_field("pressure"));
                            }
                            pressure = Some(map.next_value()?);
                        }
                    }
                }
                let typ = typ.ok_or_else(|| de::Error::missing_field("type"))?;
                let meta = meta.ok_or_else(|| de::Error::missing_field("meta"))?;
                let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
                let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;
                let pressure = pressure.ok_or_else(|| de::Error::missing_field("pressure"))?;
                let mut stroke = Stroke::new(x, y, timestamp, pressure);
                stroke.meta = meta;
                stroke.typ = typ;
                Ok(stroke)
            }
        }

        const FIELDS: &'static [&'static str] = &["type", "meta", "x", "y", "timestamp", "pressure"];
        deserializer.deserialize_struct("Stroke", FIELDS, StrokeVisitor)
    }
}