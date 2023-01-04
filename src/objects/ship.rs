use std::f32::consts::PI;

use crate::Vertex;

use super::Object;

pub struct Ship {
    x:f32,
    y:f32,
    size: f32,
    angle: f32,
    vertices: [Vertex; 3],
}

impl Ship {
    pub fn new(size: f32, x: f32, y: f32) -> Ship {
        let vertices = [
            Vertex{ position: [x, y + f32::sqrt(3.0) * size / 3.] }, 
            Vertex{ position: [x - size / 2.0, y - f32::sqrt(3.0) * size / 6.] }, 
            Vertex{ position: [x + size / 2.0, y - f32::sqrt(3.0) * size / 6.] }
        ];

        Ship {
            x,
            y,
            size,
            angle: 0.0,
            vertices,
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.angle = angle;
    }

    pub fn get_vertices(&self) -> [Vertex; 3] {
        self.vertices
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn move_forward(&mut self) {
        self.x -= 0.01*self.angle.sin();
        self.y += 0.01*self.angle.cos();

        if self.x >= 1.0 {
            self.x = -0.99;
        } else if self.x <= -1.0 {
            self.x = 0.99;
        }

        if self.y >= 1.0 {
            self.y = -0.99;
        } else if self.y <= -1.0 {
            self.y = 0.99;
        }
    }

    pub fn move_backward(&mut self) {
        self.x += 0.01*self.angle.sin();
        self.y -= 0.01*self.angle.cos();

        if self.x >= 1.0 {
            self.x = -0.99;
        } else if self.x <= -1.0 {
            self.x = 0.99;
        }

        if self.y >= 1.0 {
            self.y = -0.99;
        } else if self.y <= -1.0 {
            self.y = 0.99;
        }
    }

    pub fn rotate_left(&mut self) {
        self.angle += 3.*(PI / 180.0);
    }

    pub fn rotate_right(&mut self) {
        self.angle -= 3.*(PI / 180.0);
    }
}

impl Object for Ship {
    fn get_size(&self) -> f32 {
        self.size
    }

    fn get_pos(&self) -> (f32, f32) {
        self.get_pos()
    }

    fn get_vertices(&self) -> Vec<Vertex>{
        let mut vertices = Vec::<Vertex>::new();
        vertices.push(Vertex{ position: [self.x, self.y + f32::sqrt(3.0) * self.size / 3.] }); 
        vertices.push(Vertex{ position: [self.x - self.size / 2.0, self.y - f32::sqrt(3.0) * self.size / 6.] });
        vertices.push(Vertex{ position: [self.x + self.size / 2.0, self.y - f32::sqrt(3.0) * self.size / 6.] });
        vertices
    }
}