use std::{collections::HashMap, mem};

use gl::{types::GLfloat, Gl};

use crate::{
    block::{Block, Side},
    mymath::BlockPosInWorld,
    texture::{block_texture, texture_atlas::TextureUV},
    vertex::Vertex,
};

#[allow(unused)]
type Point3 = nalgebra::Point3<f32>;
#[allow(unused)]
type Vector3 = nalgebra::Vector3<f32>;
#[allow(unused)]
type Matrix4 = nalgebra::Matrix4<f32>;

pub const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);
pub const DOWN: Vector3 = Vector3::new(0.0, -1.0, 0.0);
pub const NORTH: Vector3 = Vector3::new(1.0, 0.0, 0.0);
pub const SOUTH: Vector3 = Vector3::new(-1.0, 0.0, 0.0);
pub const WEST: Vector3 = Vector3::new(0.0, 0.0, 1.0);
pub const EAST: Vector3 = Vector3::new(0.0, 0.0, -1.0);

const BLOCK_SIZE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

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

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::<f32>::with_capacity(capacity),
            vertex_num: 0,
        }
    }

    pub fn add_block(
        &mut self,
        begin: &BlockPosInWorld,
        block: &Block,
        textures: &HashMap<&str, TextureUV>,
    ) {
        let begin = begin.cast::<f32>();
        self.add_cuboid(&begin, &(begin + BLOCK_SIZE), block, textures);
    }

    // beginはendよりも(-∞, -∞, -∞)に近い
    pub fn add_cuboid(
        &mut self,
        begin: &Point3,
        end: &Point3,
        block: &Block,
        textures: &HashMap<&str, TextureUV>,
    ) {
        // 上面
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, end.y, end.z),
            &end,
            &Point3::new(end.x, end.y, begin.z),
            &textures[block_texture::get_texture_name(block, Side::TOP)],
        );

        // 下面
        self.add_face(
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            &begin,
            &textures[block_texture::get_texture_name(block, Side::BOTTOM)],
        );

        // 南
        self.add_face(
            &Point3::new(begin.x, end.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(begin.x, end.y, end.z),
            &textures[block_texture::get_texture_name(block, Side::SOUTH)],
        );

        // 北
        self.add_face(
            &Point3::new(end.x, end.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(end.x, end.y, begin.z),
            &textures[block_texture::get_texture_name(block, Side::NORTH)],
        );

        // 西
        self.add_face(
            &Point3::new(end.x, end.y, begin.z),
            &Point3::new(end.x, begin.y, begin.z),
            &Point3::new(begin.x, begin.y, begin.z),
            &Point3::new(begin.x, end.y, begin.z),
            &textures[block_texture::get_texture_name(block, Side::WEST)],
        );

        // 東
        self.add_face(
            &Point3::new(begin.x, end.y, end.z),
            &Point3::new(begin.x, begin.y, end.z),
            &Point3::new(end.x, begin.y, end.z),
            &Point3::new(end.x, end.y, end.z),
            &textures[block_texture::get_texture_name(block, Side::EAST)],
        );
    }

    // p1: 左上, p2: 左下, p3: 右下, p4: 右上
    pub fn add_face(&mut self, p1: &Point3, p2: &Point3, p3: &Point3, p4: &Point3, uv: &TextureUV) {
        let normal = (p3 - p1).cross(&(p2 - p4)).normalize();
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

    pub fn generate_vertex_obj(self, gl: &Gl) -> Vertex {
        Vertex::new(
            gl.clone(),
            (self.buffer.len() * mem::size_of::<GLfloat>()) as _,
            self.buffer.as_ptr() as _,
            gl::STATIC_DRAW,
            3usize,
            vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
            vec![3, 3, 2],
            ((3 + 3 + 2) * mem::size_of::<GLfloat>()) as _,
            self.vertex_num,
        )
    }
}
