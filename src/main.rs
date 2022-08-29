use std::path::Path;
use std::time::Instant;

use nameof::name_of_type;
use parry3d::shape::Cuboid;
use specs::DispatcherBuilder;
use specs::{Builder, World, WorldExt};

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

pub mod block;
pub mod camera_computer;
pub mod chunk;
pub mod components;
mod ecs_resources;
pub mod game_config;
pub mod mymath;
mod systems;
pub mod texture;
pub mod world;
use block::Block;
use camera_computer::CameraComputer;
use chunk::Chunk;
use components::*;
use ecs_resources::*;
use mymath::*;
use systems::*;
use texture::block_texture;
use texture::block_texture::BlockTextures;
use world::GameWorld;

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

    let vao_config = VaoConfigBuilder::new(&game.shader).build();
    let vertex_obj = game
        .world
        .get_chunk(&chunk_zero_pos)
        .unwrap()
        .generate_vertex_obj(gl, &game.block_textures, &vao_config);
    println!("OK: init main VBO and VAO");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Angle2>();
    world.register::<Input>();
    world.register::<Collider>();
    world.register::<OnGround>();
    world.insert(DeltaTick(0));
    world.insert(game.world);
    println!("OK: init ECS World");
    let player = world
        .create_entity()
        .with(Position(Point3::new(4.0, 2.5, 4.0)))
        .with(Velocity::default())
        .with(Acceleration::gravity())
        .with(Angle2::new(Deg(225.0f32), Deg(0.0f32)))
        .with(Input::new())
        .with(Collider(Cuboid::new(Vector3::new(0.15, 0.45, 0.15))))
        .with(OnGround(false))
        .build();
    println!("OK: spawn player");
    let mut dispatcher = DispatcherBuilder::new()
        .with(AngleController, name_of_type!(AngleController), &[])
        .with(
            VelocityController,
            name_of_type!(VelocityController),
            &[name_of_type!(AngleController)],
        )
        .with(
            VelocityUpdater,
            name_of_type!(VelocityUpdater),
            &[name_of_type!(VelocityController)],
        )
        .with(
            CollisionHandler,
            name_of_type!(CollisionHandler),
            &[name_of_type!(VelocityUpdater)],
        )
        .with(
            PositionUpdater,
            name_of_type!(PositionUpdater),
            &[name_of_type!(CollisionHandler)],
        )
        .build();
    println!("OK: init ECS Dispatcher");

    let camera = CameraComputer::new();
    println!("OK: init camera computer");

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

    let mut last_tick = Instant::now();

    let width = 900;
    let height = 480;

    'main: loop {
        if game.window.process_event() {
            break 'main;
        }

        // DeltaTickリソースを更新
        {
            let mut delta_tick = world.write_resource::<DeltaTick>();
            delta_tick.0 = last_tick.elapsed().as_millis() as u32;
            last_tick = Instant::now();
        }
        dispatcher.dispatch(&mut world);
        let player_pos = world.read_storage::<Position>();
        let player_pos = player_pos.get(player).unwrap();
        let player_angle = world.read_storage::<Angle2>();
        let player_angle = player_angle.get(player).unwrap();
        let player_vel = world.read_storage::<Velocity>();
        let _player_vel = player_vel.get(player).unwrap();
        let player_acc = world.read_storage::<Acceleration>();
        let _player_acc = player_acc.get(player).unwrap();
        let player_is_on_ground = world.read_storage::<OnGround>();
        let _player_is_on_ground = player_is_on_ground.get(player).unwrap();

        unsafe {
            if depth_test {
                gl.Enable(gl::DEPTH_TEST);
            } else {
                gl.Disable(gl::DEPTH_TEST);
            }

            if blend {
                gl.Enable(gl::BLEND);
                gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            } else {
                gl.Disable(gl::BLEND);
            }

            if wireframe {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            } else {
                gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            if culling {
                gl.Enable(gl::CULL_FACE);
            } else {
                gl.Disable(gl::CULL_FACE);
            }
        }

        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);

            gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let model_matrix =
            nalgebra_glm::scale(&Matrix4::identity(), &Vector3::new(0.5f32, 0.5f32, 0.5f32));
        let view_matrix = camera.compute_view_matrix(&player_angle, &player_pos);
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
                TripleFloat(player_pos.0.x, player_pos.0.y, player_pos.0.z),
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
