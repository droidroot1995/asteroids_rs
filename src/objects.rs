use crate::Vertex;

pub mod asteroid;
pub mod ship;
pub mod bullet;

pub trait Object {
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_size(&self) -> f32;
    fn get_pos(&self) -> (f32, f32);
}