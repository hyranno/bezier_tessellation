
use crate::mesh::*;

pub fn create() -> Mesh {
    let pi = std::f32::consts::PI;

    Mesh {
        vertices: vec![
            VertexData {
                position: [0.5*(pi*0.0/3.0).sin(), 0.0, 0.5*(pi*0.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*1.0/3.0).sin(), 0.5, 0.5*(pi*1.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*2.0/3.0).sin(), 0.0, 0.5*(pi*2.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*3.0/3.0).sin(), 0.5, 0.5*(pi*3.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*4.0/3.0).sin(), 0.0, 0.5*(pi*4.0/3.0).cos()],
            },
            VertexData {
                position: [0.5*(pi*5.0/3.0).sin(), 0.5, 0.5*(pi*5.0/3.0).cos()],
            },
        ],

        edges: vec![
            Edge{
                padding: [0,0],
                vertices: [0, 1],
                control_points: [
                    [0.52*(pi*1.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*1.0/9.0).cos(), 1.0],
                    [0.52*(pi*2.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*2.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [1, 2],
                control_points: [
                    [0.52*(pi*4.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*4.0/9.0).cos(), 1.0],
                    [0.52*(pi*5.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*5.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [2, 3],
                control_points: [
                    [0.52*(pi*7.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*7.0/9.0).cos(), 1.0],
                    [0.52*(pi*8.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*8.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [3, 4],
                control_points: [
                    [0.52*(pi*10.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*10.0/9.0).cos(), 1.0],
                    [0.52*(pi*11.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*11.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [4, 5],
                control_points: [
                    [0.52*(pi*13.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*13.0/9.0).cos(), 1.0],
                    [0.52*(pi*14.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*14.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [5, 0],
                control_points: [
                    [0.52*(pi*16.0/9.0).sin(), 0.5*(2.0/3.0), 0.52*(pi*16.0/9.0).cos(), 1.0],
                    [0.52*(pi*17.0/9.0).sin(), 0.5*(1.0/3.0), 0.52*(pi*17.0/9.0).cos(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [0, 2],
                control_points: [
                    [0.5*(pi*0.0/3.0).sin() +0.3*(pi*0.0/3.0).cos(), 0.0, 0.5*(pi*0.0/3.0).cos() -0.3*(pi*0.0/3.0).sin(), 1.0],
                    [0.5*(pi*2.0/3.0).sin() -0.3*(pi*2.0/3.0).cos(), 0.0, 0.5*(pi*2.0/3.0).cos() +0.3*(pi*2.0/3.0).sin(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [2, 4],
                control_points: [
                    [0.5*(pi*2.0/3.0).sin() +0.3*(pi*2.0/3.0).cos(), 0.0, 0.5*(pi*2.0/3.0).cos() -0.3*(pi*2.0/3.0).sin(), 1.0],
                    [0.5*(pi*4.0/3.0).sin() -0.3*(pi*4.0/3.0).cos(), 0.0, 0.5*(pi*4.0/3.0).cos() +0.3*(pi*4.0/3.0).sin(), 1.0],
                ]
            },
            Edge{
                padding: [0,0],
                vertices: [4, 0],
                control_points: [
                    [0.5*(pi*4.0/3.0).sin() +0.3*(pi*4.0/3.0).cos(), 0.0, 0.5*(pi*4.0/3.0).cos() -0.3*(pi*4.0/3.0).sin(), 1.0],
                    [0.5*(pi*0.0/3.0).sin() -0.3*(pi*0.0/3.0).cos(), 0.0, 0.5*(pi*0.0/3.0).cos() +0.3*(pi*0.0/3.0).sin(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [1, 3],
                control_points: [
                    [0.5*(pi*1.0/3.0).sin() +0.3*(pi*1.0/3.0).cos(), 0.5, 0.5*(pi*1.0/3.0).cos() -0.3*(pi*1.0/3.0).sin(), 1.0],
                    [0.5*(pi*3.0/3.0).sin() -0.3*(pi*3.0/3.0).cos(), 0.5, 0.5*(pi*3.0/3.0).cos() +0.3*(pi*3.0/3.0).sin(), 1.0],
                ],
            },
            Edge{
                padding: [0,0],
                vertices: [3, 5],
                control_points: [
                    [0.5*(pi*3.0/3.0).sin() +0.3*(pi*3.0/3.0).cos(), 0.5, 0.5*(pi*3.0/3.0).cos() -0.3*(pi*3.0/3.0).sin(), 1.0],
                    [0.5*(pi*5.0/3.0).sin() -0.3*(pi*5.0/3.0).cos(), 0.5, 0.5*(pi*5.0/3.0).cos() +0.3*(pi*5.0/3.0).sin(), 1.0],
                ]
            },
            Edge{
                padding: [0,0],
                vertices: [5, 1],
                control_points: [
                    [0.5*(pi*5.0/3.0).sin() +0.3*(pi*5.0/3.0).cos(), 0.5, 0.5*(pi*5.0/3.0).cos() -0.3*(pi*5.0/3.0).sin(), 1.0],
                    [0.5*(pi*1.0/3.0).sin() -0.3*(pi*1.0/3.0).cos(), 0.5, 0.5*(pi*1.0/3.0).cos() +0.3*(pi*1.0/3.0).sin(), 1.0],
                ],
            },
        ],

        faces: vec![
            Face{edges: [0, 1, 6]},
            Face{edges: [1, 9, 2]},
            Face{edges: [2, 3, 7]},
            Face{edges: [3, 10, 4]},
            Face{edges: [4, 5, 8]},
            Face{edges: [5, 11, 0]},
            Face{edges: [6, 7, 8]},
            Face{edges: [11, 10, 9]},
        ],

        /* can be calculated by faces, edges */
        vertex_indices: vec![
            0,1,2,
            2,1,3,
            2,3,4,
            4,3,5,
            4,5,0,
            0,5,1,
            0,2,4,
            5,3,1
        ],

    }
}
