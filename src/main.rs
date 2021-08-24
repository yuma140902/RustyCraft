use std::mem;
use std::path::Path;

use cgmath;
use cgmath::prelude::SquareMatrix;

use gl::types::*;
use gl::Gl;

pub mod block;
pub mod block_texture;
pub mod buffer_builder;
pub mod camera_computer;
pub mod game_config;
pub mod image_manager;
pub mod player;
pub mod shader;
pub mod texture_atlas;
pub mod vertex;
use buffer_builder::BufferBuilder;
use camera_computer::CameraComputer;
use image_manager::ImageManager;
use player::Player;
use player::PlayerController;
use shader::Program;
use shader::Shader;
use vertex::Vertex;

#[allow(unused)]
type Point3 = cgmath::Point3<f32>;
#[allow(unused)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(unused)]
type Matrix4 = cgmath::Matrix4<f32>;

fn main() {
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

    let mut image_manager = ImageManager::new(gl.clone());
    println!("OK: init ImageManager");
    let block_atlas_tex = image_manager
        .load_image(
            Path::new("rsc/image/atlas/blocks.png"),
            "atlas/blocks",
            true,
        )
        .unwrap();
    println!(
        "OK: load {} {}x{}, #{}",
        block_atlas_tex.id, block_atlas_tex.width, block_atlas_tex.height, block_atlas_tex.gl_id
    );
    let block_textures =
        block_texture::get_textures_in_atlas(block_atlas_tex.width, block_atlas_tex.height);

    let vert_shader = Shader::from_vert_file(gl.clone(), "rsc/shader/shader.vs").unwrap();
    let frag_shader = Shader::from_frag_file(gl.clone(), "rsc/shader/shader.fs").unwrap();
    let shader = Program::from_shaders(gl.clone(), &[vert_shader, frag_shader]).unwrap();
    println!("OK: shader program");

    let mut buffer_builder = BufferBuilder::new();
    {
        buffer_builder.add_cuboid(
            &Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            &block::blocks::GRASS_BLOCK,
            &block_textures,
        );
        buffer_builder.add_cuboid(
            &Point3 {
                x: 1.2,
                y: 0.0,
                z: 0.0,
            },
            &Point3 {
                x: 2.0,
                y: 0.5,
                z: 1.5,
            },
            &block::blocks::GRASS_BLOCK,
            &block_textures,
        );
    }
    let vertex_num = buffer_builder.vertex_num();
    let vertex_buffer = buffer_builder.buffer();

    let vertex_obj = Vertex::new(
        gl.clone(),
        (vertex_buffer.len() * mem::size_of::<GLfloat>()) as _,
        vertex_buffer.as_ptr() as _,
        gl::STATIC_DRAW,
        3usize,
        vec![gl::FLOAT, gl::FLOAT, gl::FLOAT],
        vec![3, 3, 2],
        ((3 + 3 + 2) * mem::size_of::<GLfloat>()) as _,
        vertex_num,
    );
    println!("OK: init main VBO and VAO");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
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

    let mut event_pump = sdl.event_pump().unwrap();
    println!("OK: init event pump");

    let mut player = Player::new();
    println!("OK: generate Player");
    let mut controller = PlayerController::new(&timer_subsystem);
    println!("OK: init player controller");
    let camera = CameraComputer::new();
    println!("OK: init camera computer");

    /* デバッグ用 */
    let mut depth_test = true;
    let mut blend = true;
    let mut wireframe = false;
    let mut culling = true;
    let mut alpha: f32 = 1.0;
    /* ベクトルではなく色 */
    let mut material_specular = Vector3 {
        x: 0.2,
        y: 0.2,
        z: 0.2,
    };
    let mut material_shininess: f32 = 0.1;
    let mut light_direction = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 0.0,
    };
    /* ambient, diffuse, specular はベクトルではなく色 */
    let mut ambient = Vector3 {
        x: 0.3,
        y: 0.3,
        z: 0.3,
    };
    let mut diffuse = Vector3 {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    };
    let mut specular = Vector3 {
        x: 0.2,
        y: 0.2,
        z: 0.2,
    };

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
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
                    if !controller.is_paused() {
                        controller.pause();
                    } else {
                        controller.resume();
                    }
                }
                _ => {}
            }
        }
        controller.update_player(&mut player, &sdl, &window, &event_pump, &timer_subsystem);

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

        let (width, height) = window.drawable_size();
        unsafe {
            gl.Viewport(0, 0, width as i32, height as i32);

            gl.ClearColor(1.0, 1.0, 1.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let model_matrix = Matrix4::identity();
        let view_matrix = camera.compute_view_matrix(&player);
        let projection_matrix: Matrix4 = cgmath::perspective(
            cgmath::Deg(45.0f32),
            width as f32 / height as f32,
            0.1,
            100.0,
        );

        unsafe {
            use c_str_macro::c_str;
            shader.set_used();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);
            shader.set_float(c_str!("uAlpha"), alpha);
            shader.set_vec3(
                c_str!("uViewPosition"),
                player.position().x,
                player.position().y,
                player.position().z,
            );
            shader.set_vector3(c_str!("uMaterial.specular"), &material_specular);
            shader.set_float(c_str!("uMaterial.shininess"), material_shininess);
            shader.set_vector3(c_str!("uLight.direction"), &light_direction);
            shader.set_vector3(c_str!("uLight.ambient"), &ambient);
            shader.set_vector3(c_str!("uLight.diffuse"), &diffuse);
            shader.set_vector3(c_str!("uLight.specular"), &specular);
        }

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, block_atlas_tex.gl_id);
            vertex_obj.draw_triangles();
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let ui = imgui.frame();
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

                ui.text(format!("Position: {:?}", player.position()));
                ui.text(format!("Pitch: {:?}", player.pitch()));
                ui.text(format!("Yaw: {:?}", player.yaw()));
                ui.text(format!("Pause: {}", controller.is_paused()));
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

                #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient R")).range(0.0..=1.0)
                        .build(&ui, &mut ambient.x);
                #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient G")).range(0.0..=1.0)
                        .build(&ui, &mut ambient.y);
                #[rustfmt::skip]
                    imgui::Slider::new(im_str!("Ambient B")).range(0.0..=1.0)
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

        imgui_sdl2.prepare_render(&ui, &window);
        imgui_renderer.render(ui);

        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)); // 60FPS
    }
}
