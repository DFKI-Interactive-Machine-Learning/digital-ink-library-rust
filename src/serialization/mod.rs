pub mod json_serializer;

pub trait SerializableInkObject {
    fn to_json(&self) -> String;
}