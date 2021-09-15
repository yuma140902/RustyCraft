use std::path::Path;

use gl::Gl;
use imgui_sdl2::ImguiSdl2;
use parry3d::shape::Cuboid;
use sdl2::keyboard::KeyboardState;
use sdl2::mouse::MouseState;
use sdl2::video::GLContext;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::TimerSubsystem;
use sdl2::VideoSubsystem;
use specs::DispatcherBuilder;
use specs::{Builder, World, WorldExt};

pub mod block;
pub mod buffer_builder;
pub mod camera_computer;
pub mod chunk;
pub mod components;
mod ecs_resources;
pub mod game_config;
pub mod mymath;
pub mod shader;
mod systems;
pub mod texture;
pub mod vertex;
pub mod world;
use block::Block;
use camera_computer::CameraComputer;
use chunk::Chunk;
use components::*;
use ecs_resources::*;
use mymath::*;
use shader::Program;
use shader::Shader;
use systems::*;
use texture::block_texture;
use texture::block_texture::BlockTextures;
use texture::image_manager::ImageLoadInfo;
use texture::image_manager::ImageManager;
use world::GameWorld;

type Point3 = nalgebra::Point3<f32>;
type Vector3 = nalgebra::Vector3<f32>;
type Matrix4 = nalgebra::Matrix4<f32>;

struct Game<'a> {
    sdl: Sdl,
    _video_subsystem: VideoSubsystem,
    timer_subsystem: TimerSubsystem,
    window: Window,
    _gl_context: GLContext, /* GLContextを誰かが所有していないとOpenGLを使えない */
    gl: Gl,
    shader: Program,
    imgui: imgui::Context,
    imgui_sdl2: ImguiSdl2,
    imgui_renderer: imgui_opengl_renderer::Renderer,
    event_pump: EventPump,
    _image_manager: ImageManager,
    block_atlas_texture: ImageLoadInfo<'a>,
    block_textures: BlockTextures,
    world: GameWorld,
}

impl<'a> Game<'a> {
    fn init() -> Game<'a> {
        let sdl = sdl2::init().unwrap();
        println!("OK: init SDL2: {}", sdl2::version::version());
        let video_subsystem = sdl.video().unwrap();
        println!("OK: init SDL2 Video Subsystem");
        let timer_subsystem = sdl.timer().unwrap();
        println!("OK: init SDL2 Timer Subsystem");

        {
            let gl_attr = video_subsystem.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 3);
            let (major, minor) = gl_attr.context_version();
            println!("OK: init OpenGL: version {}.{}", major, minor);
        }

        let window = video_subsystem
            .window("SDL", 900, 480)
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        println!("OK: init window '{}'", window.title());

        let _gl_context = window.gl_create_context().unwrap();
        let gl = Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
        println!("OK: init GL context");

        let vert_shader = Shader::from_vert_file(gl.clone(), "rsc/shader/shader.vs").unwrap();
        let frag_shader = Shader::from_frag_file(gl.clone(), "rsc/shader/shader.fs").unwrap();
        let shader = Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();
        println!("OK: shader program");

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
        let imgui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| {
            video_subsystem.gl_get_proc_address(s) as _
        });
        {
            use imgui::im_str;
            println!(
                "OK: init ImGui (Platform: {}, Renderer: {})",
                imgui.platform_name().unwrap_or(im_str!("Unknown")),
                imgui.renderer_name().unwrap_or(im_str!("Unknown"))
            );
        }

        let event_pump = sdl.event_pump().unwrap();
        println!("OK: init event pump");

        let mut image_manager = ImageManager::new(gl.clone());
        println!("OK: init ImageManager");
        let block_atlas_texture = image_manager
            .load_image(
                Path::new("rsc/image/atlas/blocks.png"),
                "atlas/blocks",
                true,
            )
            .unwrap();
        println!(
            "OK: load {} {}x{}, #{}",
            block_atlas_texture.id,
            block_atlas_texture.width,
            block_atlas_texture.height,
            block_atlas_texture.gl_id
        );
        let block_textures = block_texture::get_textures_in_atlas(
            block_atlas_texture.width,
            block_atlas_texture.height,
        );

        let world = GameWorld::new();

        Game {
            sdl,
            _video_subsystem: video_subsystem,
            timer_subsystem,
            window,
            _gl_context,
            gl,
            shader,
            imgui,
            imgui_sdl2,
            imgui_renderer,
            event_pump,
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

    let vertex_obj = game
        .world
        .get_chunk(&chunk_zero_pos)
        .unwrap()
        .generate_vertex_obj(&gl, &game.block_textures);
    println!("OK: init main VBO and VAO");

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Force>();
    world.register::<Angle2>();
    world.register::<Input>();
    world.register::<Collider>();
    world.insert(DeltaTick(0));
    world.insert(game.world);
    println!("OK: init ECS World");
    let player = world
        .create_entity()
        .with(Position::new(Point3::new(4.0, 2.5, 4.0)))
        .with(Velocity::default())
        .with(Acceleration::gravity())
        .with(Force::default())
        .with(Angle2::new(Deg(225.0f32), Deg(0.0f32)))
        .with(Input::new())
        .with(Collider(Cuboid::new(Vector3::new(0.15, 0.45, 0.15))))
        .build();
    println!("OK: spawn player");
    let mut dispatcher = DispatcherBuilder::new()
        .with(AngleController, "angle controller", &[])
        .with(
            VelocityController,
            "velocity controller",
            &["angle controller"],
        )
        .with(
            VelocityUpdater,
            "velocity updater",
            &["velocity controller"],
        )
        .with(
            VelocityAdjusterForCollisions,
            "velocity adjuster",
            &["velocity updater"],
        )
        .with(PositionUpdater, "position updater", &["velocity adjuster"])
        .build();
    println!("OK: init ECS Dispatcher");

    let camera = CameraComputer::new();
    println!("OK: init camera computer");

    let (width, height) = game.window.drawable_size();
    let center_x: i32 = width as i32 / 2;
    let center_y: i32 = height as i32 / 2;
    game.sdl
        .mouse()
        .warp_mouse_in_window(&game.window, center_x, center_y);

    /* デバッグ用 */
    let mut depth_test = true;
    let mut blend = true;
    let mut wireframe = false;
    let mut culling = true;
    let mut alpha: f32 = 1.0;
    let mut is_paused = false;
    let mut show_imgui = false;
    /* ベクトルではなく色 */
    let mut material_specular = Vector3::new(0.2, 0.2, 0.2);
    let mut material_shininess: f32 = 0.1;
    let mut light_direction = Vector3::new(1.0, 1.0, 0.0);
    /* ambient, diffuse, specular はベクトルではなく色 */
    let mut ambient = Vector3::new(0.3, 0.3, 0.3);
    let mut diffuse = Vector3::new(0.5, 0.5, 0.5);
    let mut specular = Vector3::new(0.2, 0.2, 0.2);

    let mut last_tick = game.timer_subsystem.ticks();

    'main: loop {
        for event in game.event_pump.poll_iter() {
            game.imgui_sdl2.handle_event(&mut game.imgui, &event);
            if game.imgui_sdl2.ignore_event(&event) {
                continue;
            }

            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    is_paused = !is_paused;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    show_imgui = !show_imgui;
                }
                _ => {}
            }
        }

        let (width, height) = game.window.drawable_size();

        // DeltaTickリソースを更新
        {
            let mut delta_tick = world.write_resource::<DeltaTick>();
            let current_tick = game.timer_subsystem.ticks();
            delta_tick.0 = current_tick - last_tick;
            last_tick = current_tick;
        }
        // Inputコンポーネントを更新
        if !is_paused {
            let mut input = world.write_storage::<Input>();
            let mouse = MouseState::new(&game.event_pump);
            let keyboard = KeyboardState::new(&game.event_pump);
            let center_x: i32 = width as i32 / 2;
            let center_y: i32 = height as i32 / 2;
            *input.get_mut(player).unwrap() = Input {
                mouse_delta: nalgebra::Vector2::<i32>::new(
                    center_x - mouse.x(),
                    center_y - mouse.y(),
                ),
                pressed_keys: keyboard.pressed_scancodes().collect(),
            };
            // マウスを中心に戻す
            game.sdl
                .mouse()
                .warp_mouse_in_window(&game.window, center_x, center_y);
        }
        dispatcher.dispatch(&mut world);
        let player_pos = world.read_storage::<Position>();
        let player_pos = player_pos.get(player).unwrap();
        let player_angle = world.read_storage::<Angle2>();
        let player_angle = player_angle.get(player).unwrap();

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

        unsafe {
            use c_str_macro::c_str;
            let shader = &game.shader;
            shader.set_used();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);
            shader.set_float(c_str!("uAlpha"), alpha);
            shader.set_vec3(
                c_str!("uViewPosition"),
                player_pos.0.x,
                player_pos.0.y,
                player_pos.0.z,
            );
            shader.set_vector3(c_str!("uMaterial.specular"), &material_specular);
            shader.set_float(c_str!("uMaterial.shininess"), material_shininess);
            shader.set_vector3(c_str!("uLight.direction"), &light_direction);
            shader.set_vector3(c_str!("uLight.ambient"), &ambient);
            shader.set_vector3(c_str!("uLight.diffuse"), &diffuse);
            shader.set_vector3(c_str!("uLight.specular"), &specular);
        }

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, game.block_atlas_texture.gl_id);
            vertex_obj.draw_triangles();
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }
        if show_imgui {
            game.imgui_sdl2.prepare_frame(
                game.imgui.io_mut(),
                &game.window,
                &game.event_pump.mouse_state(),
            );

            let ui = game.imgui.frame();
            use imgui::im_str;
            imgui::Window::new(im_str!("Information"))
                .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                .position([5.0, 5.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("OpenGL Sandbox 1.0"));

                    ui.separator();

                    ui.text(format!("FPS: {:.1}", ui.io().framerate));
                    let display_size = ui.io().display_size;
                    ui.text(format!(
                        "Display Size: ({:.1}, {:.1})",
                        display_size[0], display_size[1]
                    ));
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1}, {:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));

                    ui.separator();

                    ui.checkbox(im_str!("Depth Test"), &mut depth_test);
                    ui.checkbox(im_str!("Blend"), &mut blend);
                    ui.checkbox(im_str!("Wireframe"), &mut wireframe);
                    ui.checkbox(im_str!("Culling"), &mut culling);

                    ui.separator();

                    ui.text(format!(
                        "Position: ({:.2}, {:.2}, {:.2})",
                        player_pos.0.x, player_pos.0.y, player_pos.0.z
                    ));
                    ui.text(format!("Pitch: {:?}", player_angle.pitch()));
                    ui.text(format!("Yaw: {:?}", player_angle.yaw()));
                    ui.text(format!("Pause: {}", is_paused));
                    ui.text(format!(
                        "Pressed Keys: {:?}",
                        world
                            .read_storage::<Input>()
                            .get(player)
                            .unwrap()
                            .pressed_keys
                    ));
                });
            imgui::Window::new(im_str!("Light"))
                .size([300.0, 450.0], imgui::Condition::FirstUseEver)
                .position([600.0, 10.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    imgui::Slider::new(im_str!("Alpha"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut alpha);

                    ui.separator();

                    imgui::Slider::new(im_str!("Material Specular X"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut material_specular.x);
                    imgui::Slider::new(im_str!("Material Specular Y"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut material_specular.y);
                    imgui::Slider::new(im_str!("Material Specular Z"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut material_specular.z);

                    imgui::Slider::new(im_str!("Material Shininess"))
                        .range(0.0..=2.0)
                        .build(&ui, &mut material_shininess);

                    ui.separator();

                    imgui::Slider::new(im_str!("Direction X"))
                        .range(-1.0..=1.0)
                        .build(&ui, &mut light_direction.x);
                    imgui::Slider::new(im_str!("Direction Y"))
                        .range(-1.0..=1.0)
                        .build(&ui, &mut light_direction.y);
                    imgui::Slider::new(im_str!("Direction Z"))
                        .range(-1.0..=1.0)
                        .build(&ui, &mut light_direction.z);

                    ui.separator();

                    imgui::Slider::new(im_str!("Ambient R"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut ambient.x);
                    imgui::Slider::new(im_str!("Ambient G"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut ambient.y);
                    imgui::Slider::new(im_str!("Ambient B"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut ambient.z);

                    ui.separator();

                    imgui::Slider::new(im_str!("Diffuse R"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut diffuse.x);
                    imgui::Slider::new(im_str!("Diffuse G"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut diffuse.y);
                    imgui::Slider::new(im_str!("Diffuse B"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut diffuse.z);

                    ui.separator();

                    imgui::Slider::new(im_str!("Specular R"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut specular.x);
                    imgui::Slider::new(im_str!("Specular G"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut specular.y);
                    imgui::Slider::new(im_str!("Specular B"))
                        .range(0.0..=1.0)
                        .build(&ui, &mut specular.z);
                });

            game.imgui_sdl2.prepare_render(&ui, &game.window);
            game.imgui_renderer.render(ui);
        }

        game.window.gl_swap_window();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)); // 60FPS
    }
}
