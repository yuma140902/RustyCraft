use std::collections::HashMap;

use cgmath::InnerSpace;

use crate::{
    block::{Block, Side},
    texture_atlas::TextureUV,
};

#[allow(unused)]
type Point3 = cgmath::Point3<f32>;
#[allow(unused)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(unused)]
type Matrix4 = cgmath::Matrix4<f32>;

pub const UP: Vector3 = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const DOWN: Vector3 = Vector3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};
pub const NORTH: Vector3 = Vector3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const SOUTH: Vector3 = Vector3 {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};
pub const WEST: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};
pub const EAST: Vector3 = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};

pub struct BufferBuilder {
    buffer: Vec<f32>,
    vertex_num: i32,
}

impl BufferBuilder {
    pub fn new() -> Self {
        Self {
            buffer: Vec::<f32>::new(),
            vertex_num: 0,
        }
    }

    pub fn buffer(self) -> Vec<f32> {
        self.buffer
    }

    pub fn vertex_num(&self) -> i32 {
        self.vertex_num
    }

    // beginはendよりも(-∞, -∞, -∞)に近い
    pub fn add_cuboid(
        &mut self,
        begin: &Point3,
        end: &Point3,
        block: &dyn Block,
        textures: &HashMap<&str, TextureUV>,
    ) {
        // 上面
        self.add_face(
            &Point3 {
                x: begin.x,
                y: end.y,
                z: begin.z,
            },
            &Point3 {
                x: begin.x,
                y: end.y,
                z: end.z,
            },
            &end,
            &Point3 {
                x: end.x,
                y: end.y,
                z: begin.z,
            },
            &textures[block.get_texture_uv(Side::TOP)],
        );

        // 下面
        self.add_face(
            &Point3 {
                x: end.x,
                y: begin.y,
                z: begin.z,
            },
            &Point3 {
                x: end.x,
                y: begin.y,
                z: end.z,
            },
            &Point3 {
                x: begin.x,
                y: begin.y,
                z: end.z,
            },
            &begin,
            &textures[block.get_texture_uv(Side::BOTTOM)],
        );

        // 南
        self.add_face(
            &Point3 {
                x: begin.x,
                y: end.y,
                z: begin.z,
            },
            &Point3 {
                x: begin.x,
                y: begin.y,
                z: begin.z,
            },
            &Point3 {
                x: begin.x,
                y: begin.y,
                z: end.z,
            },
            &Point3 {
                x: begin.x,
                y: end.y,
                z: end.z,
            },
            &textures[block.get_texture_uv(Side::SOUTH)],
        );

        // 北
        self.add_face(
            &Point3 {
                x: end.x,
                y: end.y,
                z: end.z,
            },
            &Point3 {
                x: end.x,
                y: begin.y,
                z: end.z,
            },
            &Point3 {
                x: end.x,
                y: begin.y,
                z: begin.z,
            },
            &Point3 {
                x: end.x,
                y: end.y,
                z: begin.z,
            },
            &textures[block.get_texture_uv(Side::NORTH)],
        );

        // 西
        self.add_face(
            &Point3 {
                x: end.x,
                y: end.y,
                z: begin.z,
            },
            &Point3 {
                x: end.x,
                y: begin.y,
                z: begin.z,
            },
            &Point3 {
                x: begin.x,
                y: begin.y,
                z: begin.z,
            },
            &Point3 {
                x: begin.x,
                y: end.y,
                z: begin.z,
            },
            &textures[block.get_texture_uv(Side::WEST)],
        );

        // 東
        self.add_face(
            &Point3 {
                x: begin.x,
                y: end.y,
                z: end.z,
            },
            &Point3 {
                x: begin.x,
                y: begin.y,
                z: end.z,
            },
            &Point3 {
                x: end.x,
                y: begin.y,
                z: end.z,
            },
            &Point3 {
                x: end.x,
                y: end.y,
                z: end.z,
            },
            &textures[block.get_texture_uv(Side::EAST)],
        );
    }

    // p1: 左上, p2: 左下, p3: 右下, p4: 右上
    pub fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &TextureUV) {
        let normal = (p3 - p1).cross(p2 - p4).normalize();
        #[rustfmt::skip]
        let mut v: Vec<f32> = vec![
            p1.x, p1.y, p1.z, normal.x, normal.y, normal.z, uv.begin_u, uv.end_v,/* UVはtodo */
            p2.x, p2.y, p2.z, normal.x, normal.y, normal.z, uv.begin_u, uv.begin_v,
            p3.x, p3.y, p3.z, normal.x, normal.y, normal.z, uv.end_u, uv.begin_v,

            p1.x, p1.y, p1.z, normal.x, normal.y, normal.z, uv.begin_u, uv.end_v,
            p3.x, p3.y, p3.z, normal.x, normal.y, normal.z, uv.end_u, uv.begin_v,
            p4.x, p4.y, p4.z, normal.x, normal.y, normal.z, uv.end_u, uv.end_v,
        ];

        self.vertex_num += 6;

        self.buffer.append(&mut v);
    }
}
