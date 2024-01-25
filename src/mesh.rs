use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

struct Vertex {
    position: Vec3,
    normal: Vec3,
}

struct Rect {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Rect {
    fn new(origin: Vec3, normal: Vec3, height: f32, width: f32, offset: f32) -> Self {
            
            if normal == Vec3::ZERO {
                panic!("Cannot find perpendicular vectors for a zero vector");
            }
        
            // Choose an arbitrary vector that is not parallel to 'v'.
            // For example, if 'v' is not parallel to the x-axis, use Vec3::X.
            let arbitrary_vector = if normal.x.abs() > 0.1 { Vec3::Y } else { Vec3::X };
        
            // First perpendicular vector
            let mut perp_1 = normal.cross(arbitrary_vector).normalize();
        
            // Second perpendicular vector
            let mut perp_2 = normal.cross(perp_1).normalize();
    
            perp_1 *= width*0.5 - offset*0.5;
            perp_2 *= height*0.5 - offset*0.5;
    
            let vertices = vec![
                Vertex { position: origin + (-perp_1) + (-perp_2), normal, },
                Vertex { position: origin + (perp_1) + (-perp_2), normal, },
                Vertex { position: origin + (perp_1) + (perp_2), normal, },
                Vertex { position: origin + (-perp_1) + (perp_2), normal, },
            ];
    
            let indices: Vec<u32> = vec![
                0, 1, 2, 2, 3, 0,
            ];
    
            Rect {
                vertices,
                indices,
            }
    }
}

struct Square {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Square {
    fn new(origin: Vec3, normal: Vec3, size: f32, offset: f32) -> Self {

        let rect = Rect::new(origin, normal, size, size, offset);

        Square {
            vertices: rect.vertices,
            indices: rect.indices,
        }
    }
}

struct Box {
    sides: Vec<Square>,
}

impl Box {
    fn new(origin: Vec3, length: f32, width: f32, height: f32, offset: f32) -> Self {
        let mut sides = vec![];
        for side_offset in vec![
            Vec3::X * (width - offset),
            Vec3::Y * (height - offset),
            Vec3::Z * (length - offset),
        ] {
            sides.push(Square::new(origin + side_offset*0.5, side_offset.normalize(), 1.0, offset));
            sides.push(Square::new(origin - side_offset*0.5, -side_offset.normalize(), 1.0, offset));
        }

        Box {
            sides,
        }
    
    }
}

struct Cube {
    sides: Vec<Square>,
}

impl Cube {
    fn new(origin: Vec3, offset: f32) -> Self {
        let box_shape = Box::new(origin, 1.0, 1.0, 1.0, offset);

        Cube {
            sides: box_shape.sides,
        }
    
    }
}

pub struct CargoHull {
    cubes: Vec<Cube>,
}

impl CargoHull {
    pub fn new(translation: Vec3, length: i32, width: i32, height: i32, offset: f32) -> CargoHull {
        let mut cubes = vec![];
        for z_offset in 0..length {
            let z = z_offset as f32;

            let mut sides: Vec<f32> = vec![];
            if width == 2 {
                sides.push(-1.0);
                sides.push(1.0);
            } else {
                sides.push(0.0);
            }

            for x_offset in sides.iter() {
                let x = *x_offset;
                for y_offset in 0..height {
                    let y = y_offset as f32;
                    let origin = Vec3::new(x, y, z) + translation;
                    cubes.push(Cube::new(origin, offset));
                }
            }
        }

        CargoHull {
            cubes,
        }
    }
}



impl From<CargoHull> for Mesh {
    fn from(hull: CargoHull) -> Self {

        let mut positions: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut indices_offset = 0;

        for cube in hull.cubes.iter() {
            for square in cube.sides.iter() {
                for vertex in square.vertices.iter() {
                    positions.push(vertex.position.into());
                    normals.push(vertex.normal.into());
                }
                for index in square.indices.iter() {
                    indices.push(index + indices_offset);
                }
                indices_offset += 4;
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_indices(Some(Indices::U32(indices)))
    }
}