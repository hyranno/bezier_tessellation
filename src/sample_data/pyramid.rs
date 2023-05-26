use crate::mesh::*;

pub fn create() -> Mesh {
    let pi = std::f32::consts::PI;

    Mesh {
        vertices: vec![
            VertexData {
                position: [0.0, 0.5, 0.0],
            },
            VertexData {
                position: [0.5*(pi*0.0/3.0).sin(), 0.0, 0.5*(pi*0.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*2.0/3.0).sin(), 0.0, 0.5*(pi*2.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*4.0/3.0).sin(), 0.0, 0.5*(pi*4.0/3.0).cos()],
            },
        ],

        edges: vec![
            Edge{
                padding: [0,0],
                vertices: [0, 1],
                control_points: [
                    [0.2*(pi*0.0/3.0).sin(), 0.3, 0.2*(pi*0.0/3.0).cos(), 1.0],
                    [0.3*(pi*0.0/3.0).sin(), 0.2, 0.3*(pi*0.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [0, 2],
                control_points: [
                    [0.2*(pi*2.0/3.0).sin(), 0.3, 0.2*(pi*2.0/3.0).cos(), 1.0],
                    [0.3*(pi*2.0/3.0).sin(), 0.2, 0.3*(pi*2.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [0, 3],
                control_points: [
                    [0.2*(pi*4.0/3.0).sin(), 0.3, 0.2*(pi*4.0/3.0).cos(), 1.0],
                    [0.3*(pi*4.0/3.0).sin(), 0.2, 0.3*(pi*4.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [1, 2],
                control_points: [
                    [0.35*(pi*0.0/3.0).sin() +0.15*(pi*2.0/3.0).sin(), 0.0, 0.35*(pi*0.0/3.0).cos() +0.15*(pi*2.0/3.0).cos(), 1.0],
                    [0.15*(pi*0.0/3.0).sin() +0.35*(pi*2.0/3.0).sin(), 0.0, 0.15*(pi*0.0/3.0).cos() +0.35*(pi*2.0/3.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [2, 3],
                control_points: [
                    [0.35*(pi*2.0/3.0).sin() +0.15*(pi*4.0/3.0).sin(), 0.0, 0.35*(pi*2.0/3.0).cos() +0.15*(pi*4.0/3.0).cos(), 1.0],
                    [0.15*(pi*2.0/3.0).sin() +0.35*(pi*4.0/3.0).sin(), 0.0, 0.15*(pi*2.0/3.0).cos() +0.35*(pi*4.0/3.0).cos(), 1.0],
                ]
            },
            Edge{
                padding: [0,0],
                vertices: [3, 1],
                control_points: [
                    [0.35*(pi*4.0/3.0).sin() +0.15*(pi*0.0/3.0).sin(), 0.0, 0.35*(pi*4.0/3.0).cos() +0.15*(pi*0.0/3.0).cos(), 1.0],
                    [0.15*(pi*4.0/3.0).sin() +0.35*(pi*0.0/3.0).sin(), 0.0, 0.15*(pi*4.0/3.0).cos() +0.35*(pi*0.0/3.0).cos(), 1.0],
                ],
            },
        ],
    
        faces: vec![
            Face{edges: [0, 3, 1]},
            Face{edges: [1, 4, 2]},
            Face{edges: [2, 5, 0]},
            Face{edges: [3, 4, 5]},
        ],
    
        /* can be calculated by faces, edges */
        vertex_indices: vec![
            0,1,2,
            0,2,3,
            0,3,1,
            1,2,3
        ],
    
    }
}