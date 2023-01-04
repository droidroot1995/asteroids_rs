use crate::Vertex;

use super::Object;

#[derive(Debug, Clone)]
pub struct Asteroid {
    x:f32,
    y:f32,
    incx:f32,
    incy:f32,
    size: f32,
    initx:f32,
    inity:f32,
    dirx: f32,
    diry: f32,
    a: f32,
    vertices: [Vertex; 6],
}

impl Asteroid {
    pub fn new(size: f32, x: f32, y: f32, shipx: f32, shipy: f32) -> Asteroid {
        let dirx = shipx - x;
        let diry = shipy - y;

        Asteroid {
            x: x,
            y: y,
            incx: x,
            incy: y,
            size: size,
            initx: x,
            inity: y,
            dirx: dirx,
            diry: diry,
            a: dirx / diry,
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
        if self.dirx.abs() > self.diry.abs() {
            self.x = if self.dirx < 0. { self.incx -= 0.00001; self.incx} else { self.incx += 0.00001; self.incx};
            self.y = (self.a) * (self.x - self.initx) + self.inity;
        } else {
            self.y = if self.diry < 0. { self.incy -= 0.00001; self.incy} else { self.incy += 0.00001; self.incy};
            self.x = ((self.y - self.inity) / self.a) + self.initx;
        }

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

    pub fn update_ship_pos(&mut self, pos: (f32, f32)) {
        self.dirx = pos.0 - self.x;
        self.diry = pos.1 - self.y;
    }
}

impl Object for Asteroid {
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

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {

        let mut equal_vertices_num = 0;
        for i in 0..self.vertices.len() {
            if self.vertices[i].position[0] == other.vertices[i].position[0] && self.vertices[i].position[1] == other.vertices[i].position[1] {
                equal_vertices_num += 1
            }
        }

        let equal = self.x == other.x && self.y == other.y && self.incx == other.incx && self.incy == other.incy && self.dirx == other.dirx && self.diry == other.diry && self.initx == other.initx && self.inity == other.inity && self.a == other.a && self.size == other.size && self.vertices.len() == equal_vertices_num;
        equal
    }
}