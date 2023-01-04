pub mod objects;

extern crate glium;

use crate::objects::asteroid::Asteroid;
use crate::objects::ship::Ship;
use crate::objects::bullet::Bullet;
use glium::{glutin::{self, event::{self, VirtualKeyCode, WindowEvent}}, implement_vertex, index::PrimitiveType, uniform, Surface};
use objects::Object;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Asteroids");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    implement_vertex!(Vertex, position);

    let mut ship = Ship::new(0.1, 0.0, 0.0);
    let mut asteroids = Vec::<Asteroid>::new();
    let mut bullets = Vec::<Bullet>::new();

    create_asteroids(&mut asteroids, &ship, 10);

    event_loop.run( move |event, _, control_flow| {

        *control_flow = match event {
            event::Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                WindowEvent::Resized(..) => {
                    glutin::event_loop::ControlFlow::Poll
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let event::ElementState::Pressed = input.state {

                        match input.virtual_keycode.unwrap() {
                            VirtualKeyCode::W | VirtualKeyCode::Up => ship.move_forward(),
                            VirtualKeyCode::S | VirtualKeyCode::Down => ship.move_backward(),
                            VirtualKeyCode::A | VirtualKeyCode::Left => ship.rotate_left(),
                            VirtualKeyCode::D | VirtualKeyCode::Right => ship.rotate_right(),
                            VirtualKeyCode::Space => {
                                let ship_pos = ship.get_pos();
                                let ship_angle = ship.get_angle();
                                let bullet = Bullet::new(0.02, ship_pos.0, ship_pos.1, ship_angle);
                                bullets.push(bullet);
                            },
                            VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                                return
                            }
                            _ => ()
                        }
                    }
                    glutin::event_loop::ControlFlow::Poll
                },
                _ => glutin::event_loop::ControlFlow::Poll,
            },
            _ => glutin::event_loop::ControlFlow::Poll,
        };

        draw_objects(&ship, &mut asteroids, &mut bullets, &display);
        check_collisions(&mut ship, &mut asteroids, &mut bullets);

        if asteroids.len() < 5 {
            create_asteroids(&mut asteroids, &ship, 10)
        }
    })
}

fn draw_objects(ship: &Ship, asteroids: &mut Vec<Asteroid>, bullets: &mut Vec<Bullet>, display: &glium::backend::glutin::Display) {

    let ship_angle = ship.get_angle();
    let ship_pos = ship.get_pos();

    // building the uniforms
    let ship_uniforms = uniform! {
        matrix: [
            [ship_angle.cos(), ship_angle.sin(), 0.0, 0.0],
            [-ship_angle.sin(), ship_angle.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [ship_pos.0, ship_pos.1, 0.0, 1.0f32]
        ]
    };

    let vertex_buffer = {
        glium::VertexBuffer::new(display,
            &ship.get_vertices()
        ).unwrap()
    };

    // building the index buffer
    let ship_index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList,
        &[0u16, 1, 2]).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let ship_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 1.0, 0.0, 1.0);
        }
    "#;

    let asteroid_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 1.0, 1.0);
        }
    "#;

    let bullet_fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program_ship = glium::Program::from_source(display, vertex_shader_src, ship_fragment_shader_src, None).unwrap();
    let program_asteroid = glium::Program::from_source(display, vertex_shader_src, asteroid_fragment_shader_src, None).unwrap();
    let program_bullet = glium::Program::from_source(display, vertex_shader_src, bullet_fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(&vertex_buffer, &ship_index_buffer, &program_ship, &ship_uniforms, &Default::default()).unwrap();

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 3, 4, 5, 6]).unwrap();

    for aster in asteroids {
        aster.update_position();
        let vertex_buffer = {
            glium::VertexBuffer::new(display,
                &aster.get_vertices()
            ).unwrap()
        };
        target.draw(&vertex_buffer, &index_buffer, &program_asteroid, &uniforms, &Default::default()).unwrap();
    }

    for bullet in bullets {
        bullet.update_position();
        let vertex_buffer = {
            glium::VertexBuffer::new(display,
                &bullet.get_vertices()
            ).unwrap()
        };
        target.draw(&vertex_buffer, &index_buffer, &program_bullet, &uniforms, &Default::default()).unwrap();
    }
    target.finish().unwrap();
}

fn create_asteroids(asteroids: &mut Vec<Asteroid>, ship: &Ship, number: u32) {
    let ship_pos = ship.get_pos();
    let mut rng = rand::thread_rng();

    for _ in 1..number {
        let x = rng.gen_range(-1.0..1.0) as f32;
        let y = rng.gen_range(-1.0..1.0) as f32;
        let size = rng.gen_range(0.05..0.3) as f32;
        let asteroid = Asteroid::new(size, x, y, ship_pos.0, ship_pos.1);
        asteroids.push(asteroid);
    }
}

fn check_collisions(ship: &mut Ship, asteroids: &mut Vec<Asteroid>, bullets: &mut Vec<Bullet>) {

    let mut asteroids_collided = Vec::<Asteroid>::new();
    let mut bullets_collided = Vec::<Bullet>::new();

    for aster in asteroids.iter() {

        if !asteroids_collided.contains(aster) {
            let pos = aster.get_pos();
            if pos.0 >= 1.0 || pos.0 <= -1.0 || pos.1 >= 1.0 || pos.1 <= -1.0 {
                asteroids_collided.push(aster.clone());
                continue;
            }
        }
        
        if check_collision(ship, aster) {
            asteroids.clear();
            bullets.clear();
            ship.set_pos(0.0, 0.0);
            ship.set_angle(0.0);
            break;
        }

        for  bullet in bullets.iter() {
            
            if !bullets_collided.contains(bullet) {

                let pos = aster.get_pos();
                if pos.0 >= 1.0 || pos.0 <= -1.0 || pos.1 >= 1.0 || pos.1 <= -1.0 {
                    bullets_collided.push(bullet.clone());
                    continue;
                }

                if check_collision(bullet, aster) {
                    asteroids_collided.push(aster.clone());
                    bullets_collided.push(bullet.clone());
                }
            }
        }
    }

    for aster in asteroids_collided {
        if let Some(pos) = asteroids.iter().position(|x| *x == aster) {
            asteroids.remove(pos);
        }
    }

    for bullet in bullets_collided {
        if let Some(pos) = bullets.iter().position(|x| *x == bullet) {
            bullets.remove(pos);
        }
    }
}

fn check_collision(obj1: &dyn Object, obj2: &dyn Object) -> bool {

    let mut vertex_inside_object = false;

    let obj1_vertices = obj1.get_vertices();
    let obj2_vertices = obj2.get_vertices();

    let is_triangle = obj2_vertices.len() == 3;
    let is_rectangle = obj2_vertices.len() == 6;

    for vertex in obj1_vertices {
        if is_triangle {
            if inside_triangle(&vertex, &obj2_vertices) {
                vertex_inside_object = true;
                break
            }
        } else if is_rectangle {
            if inside_rectangle(&vertex, &obj2_vertices) {
                vertex_inside_object = true;
                break
            }
        }
    }

    vertex_inside_object
}

fn sign_triangle(point: &Vertex, vertex1: &Vertex, vertex2: &Vertex) -> f32 {
    (point.position[0] - vertex2.position[0]) * (vertex1.position[1] - vertex2.position[1]) - (vertex1.position[0] - vertex2.position[0]) * (point.position[1] - vertex2.position[1])
}

fn inside_triangle(point: &Vertex, vertices: &Vec<Vertex>) -> bool {
    let d1 = sign_triangle(point, &vertices[0], &vertices[1]);
    let d2 = sign_triangle(point, &vertices[1], &vertices[2]);
    let d3 = sign_triangle(point, &vertices[2], &vertices[0]);

    let has_neg = (d1 <= 0.0) || (d2 <= 0.0) || (d3 <= 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}

fn sign_rectangle(point: &Vertex, vertex1: &Vertex, vertex2: &Vertex) -> f32 {
    (vertex2.position[0] - vertex1.position[0]) * (point.position[1] - vertex1.position[1]) - (point.position[0] - vertex1.position[0]) * (vertex2.position[1] - vertex1.position[1])
}

fn inside_rectangle(point: &Vertex, vertices: &Vec<Vertex>) -> bool {
    let d1 = sign_rectangle(point, &vertices[1], &vertices[0]);
    let d2 = sign_rectangle(point, &vertices[2], &vertices[1]);
    let d3 = sign_rectangle(point, &vertices[3], &vertices[2]);
    let d4 = sign_rectangle(point, &vertices[0], &vertices[3]);

    let has_neg = (d1 <= 0.0) || (d2 <= 0.0) || (d3 <= 0.0) || (d4 <= 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0) || (d4 > 0.0);

    !(has_neg && has_pos)
}