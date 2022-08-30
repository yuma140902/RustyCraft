use std::path::Path;

use re::gl;
use re::gl::Gl;
use re::shader::Program;
use re::shader::Shader;
use re::shader::UniformVariables;
use re::Context;
use re::ImageLoadInfo;
use re::ImageManager;
use re::ReverieEngine;
use re::VaoConfigBuilder;
use re::Window;
use reverie_engine as re;

mod block;
mod block_texture;
mod camera_computer;
mod chunk;
mod game_config;
mod mymath;
mod player;
mod world;
use block::Block;
use block_texture::BlockTextures;
use camera_computer::CameraComputer;
use chunk::Chunk;
use mymath::*;
use world::GameWorld;

use crate::player::Player;

#[allow(dead_code)]
type Point3 = nalgebra::Point3<f32>;
type Vector3 = nalgebra::Vector3<f32>;
type Matrix4 = nalgebra::Matrix4<f32>;

struct Game<'a> {
    _engine: ReverieEngine,
    window: Window,
    gl_context: Context<glutin::RawContext<glutin::PossiblyCurrent>>,
    gl: Gl,
    shader: Program,
    _image_manager: ImageManager,
    block_atlas_texture: ImageLoadInfo<'a>,
    block_textures: BlockTextures,
    world: GameWorld,
}

impl<'a> Game<'a> {
    fn init() -> Game<'a> {
        let engine = ReverieEngine::new();
        println!("OK: init ReverieEngine");
        let window = engine.create_window();
        println!("OK: init window");
        let context = window.create_context_glutin();
        println!("OK: init glutin context");

        let gl = context.gl();
        println!("OK: init GL context");

        let vert_shader = Shader::from_vert_file(gl.clone(), "rsc/shader/shader.vs").unwrap();
        let frag_shader = Shader::from_frag_file(gl.clone(), "rsc/shader/shader.fs").unwrap();
        let shader = Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();
        println!("OK: shader program");

        let mut image_manager = ImageManager::new(gl.clone());
        println!("OK: init ImageManager");
        let image = image::open(Path::new("rsc/image/atlas/blocks.png")).unwrap();
        let block_atlas_texture = image_manager
            .load_image(image, "atlas/blocks", true)
            .unwrap();
        println!(
            "OK: load {} {}x{}, #{}",
            block_atlas_texture.id,
            block_atlas_texture.width,
            block_atlas_texture.height,
            block_atlas_texture.gl_id
        );
        let block_textures = block_texture::get_textures_in_atlas();

        let world = GameWorld::new();

        Game {
            _engine: engine,
            window,
            gl_context: context,
            gl,
            shader,
            _image_manager: image_manager,
            block_atlas_texture,
            block_textures,
            world,
        }
    }
}

fn main() {
    let mut game = Game::init();

    let gl = &game.gl;

    let chunk_zero_pos = ChunkPos::new(nalgebra::Point3::<i32>::new(0, 0, 0));
    let mut chunk = Chunk::new(chunk_zero_pos);
    for i in 0..16 {
        for j in 0..16 {
            chunk.set_block(&Block::GrassBlock, &BlockPosInChunk::new(i, 0, j).unwrap());
            chunk.set_block(&Block::GrassBlock, &BlockPosInChunk::new(0, i, j).unwrap());
            chunk.set_block(&Block::GrassBlock, &BlockPosInChunk::new(i, j, 0).unwrap());
        }
    }
    for i in 1..15 {
        chunk.set_block(&Block::GrassBlock, &BlockPosInChunk::new(i, i, 15).unwrap());
    }
    chunk.set_block(&Block::GrassBlock, &BlockPosInChunk::new(3, 3, 3).unwrap());
    game.world.add_chunk(chunk).unwrap();

    /* デバッグ用 */
    let depth_test = true;
    let blend = true;
    let wireframe = false;
    let culling = true;
    let alpha: f32 = 1.0;
    /* ベクトルではなく色 */
    let material_specular = Vector3::new(0.2, 0.2, 0.2);
    let material_shininess: f32 = 0.1;
    let light_direction = Vector3::new(1.0, 1.0, 0.0);
    /* ambient, diffuse, specular はベクトルではなく色 */
    let ambient = Vector3::new(0.3, 0.3, 0.3);
    let diffuse = Vector3::new(0.5, 0.5, 0.5);
    let specular = Vector3::new(0.2, 0.2, 0.2);
    let vao_config = VaoConfigBuilder::new(&game.shader)
        .depth_test(depth_test)
        .blend(blend)
        .wireframe(wireframe)
        .culling(culling)
        .alpha(alpha)
        .material_specular(material_specular)
        .material_shininess(material_shininess)
        .ambient(ambient)
        .diffuse(diffuse)
        .specular(specular)
        .build();

    let vertex_obj = game
        .world
        .get_chunk(&chunk_zero_pos)
        .unwrap()
        .generate_vertex_obj(gl, &game.block_textures, &vao_config);
    println!("OK: init main VBO and VAO");

    let player = Player::default();
    println!("OK: spawn player");

    let camera = CameraComputer::new();
    println!("OK: init camera computer");

    let width = 800;
    let height = 600;

    'main: loop {
        if game.window.process_event() {
            break 'main;
        }

        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);

            gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let model_matrix =
            nalgebra_glm::scale(&Matrix4::identity(), &Vector3::new(0.5f32, 0.5f32, 0.5f32));
        let view_matrix = camera.compute_view_matrix(&player.angle, &player.pos);
        let projection_matrix: Matrix4 = Matrix4::new_perspective(
            width as f32 / height as f32,
            *Deg(45.0f32).rad(),
            0.1,
            100.0,
        );

        let uniforms = {
            use c_str_macro::c_str;
            use re::shader::Uniform::*;
            let mut uniforms = UniformVariables::new();
            uniforms.add(c_str!("uModel"), Matrix4(&model_matrix));
            uniforms.add(c_str!("uView"), Matrix4(&view_matrix));
            uniforms.add(c_str!("uProjection"), Matrix4(&projection_matrix));
            uniforms.add(c_str!("uAlpha"), Float(alpha));
            uniforms.add(
                c_str!("uViewPosition"),
                TripleFloat(player.pos.x, player.pos.y, player.pos.z),
            );
            uniforms.add(c_str!("uMaterial.specular"), Vector3(&material_specular));
            uniforms.add(c_str!("uMaterial.shininess"), Float(material_shininess));
            uniforms.add(c_str!("uLight.direction"), Vector3(&light_direction));
            uniforms.add(c_str!("uLight.ambient"), Vector3(&ambient));
            uniforms.add(c_str!("uLight.diffuse"), Vector3(&diffuse));
            uniforms.add(c_str!("uLight.specular"), Vector3(&specular));
            uniforms
        };

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, game.block_atlas_texture.gl_id);
            vertex_obj.draw_triangles(&uniforms);
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        game.gl_context.swap_buffers();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)); // 60FPS
    }
}
