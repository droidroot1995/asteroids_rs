use std::f32::consts::PI;

use crate::Vertex;

use super::Object;

#[derive(Debug, Clone)]
pub struct Bullet {
    x:f32,
    y:f32,
    size: f32,
    dirx: f32,
    diry: f32,
    vertices: [Vertex; 6],
}

impl Bullet {
    pub fn new(size: f32, x: f32, y: f32, angle: f32) -> Bullet {

        let dirx = (angle + 90.*PI / 180.).cos()*3.;
        let diry = (angle + 90.*PI / 180.).sin()*3.;

        Bullet {
            x,
            y,
            size,
            dirx,
            diry,
            vertices: [
                Vertex{ position: [x - size / 2.0, y - size / 2.0] },
                Vertex{ position: [x + size / 2.0, y - size / 2.0] },
                Vertex{ position: [x + size / 2.0, y + size / 2.0] },
                Vertex{ position: [x + size / 2.0, y + size / 2.0] },
                Vertex{ position: [x - size / 2.0, y + size / 2.0] },
                Vertex{ position: [x - size / 2.0, y - size / 2.0] },
            ]
        }
    }

    pub fn update_position(&mut self) {

        self.x += self.dirx*0.0001;
        self.y += self.diry*0.0001;

        self.vertices = [
            Vertex{ position: [self.x - self.size / 2.0, self.y - self.size / 2.0] }, 
            Vertex{ position: [self.x + self.size / 2.0, self.y - self.size / 2.0] }, 
            Vertex{ position: [self.x + self.size / 2.0, self.y + self.size / 2.0] },
            Vertex{ position: [self.x + self.size / 2.0, self.y + self.size / 2.0] }, 
            Vertex{ position: [self.x - self.size / 2.0, self.y + self.size / 2.0] },
            Vertex{ position: [self.x - self.size / 2.0, self.y - self.size / 2.0] }, 
        ];
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_vertices(&self) -> [Vertex; 6] {
        self.vertices
    }
}

impl Object for Bullet {
    fn get_size(&self) -> f32 {
        self.size
    }

    fn get_pos(&self) -> (f32, f32) {
        self.get_pos()
    }

    fn get_vertices(&self) -> Vec<Vertex>{
        let mut vertices = Vec::<Vertex>::new();
        for vertex in self.vertices {
            vertices.push(vertex)
        }
        vertices
    }
}

impl PartialEq for Bullet {
    fn eq(&self, other: &Self) -> bool {

        let mut equal_vertices_num = 0;
        for i in 0..self.vertices.len() {
            if self.vertices[i].position[0] == other.vertices[i].position[0] && self.vertices[i].position[1] == other.vertices[i].position[1] {
                equal_vertices_num += 1
            }
        }

        let equal = self.x == other.x && self.y == other.y && self.dirx == other.dirx && self.diry == other.diry && self.size == other.size && self.vertices.len() == equal_vertices_num;
        equal
    }
}