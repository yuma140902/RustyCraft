use std::mem;

use cgmath;
use cgmath::prelude::SquareMatrix;

use gl::types::*;

mod shader;
mod vertex;
use shader::Shader;
use vertex::Vertex;

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version {}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", 640, 480)
        .opengl()
        .position_centered()
        .build()
        .unwrap();
    println!("OK: init window '{}'", window.title());

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
    println!("OK: init GL context");

    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");
    println!("OK: shader program");

    #[rustfmt::skip]
    let vertex_buffer: [f32; 9] = [
        -1.0, -1.0,  0.0,
         1.0, -1.0,  0.0,
         0.0,  1.0,  0.0,
    ];

    let vertex_obj = Vertex::new(
        (vertex_buffer.len() * mem::size_of::<GLfloat>()) as _,
        vertex_buffer.as_ptr() as _,
        gl::STATIC_DRAW,
        1usize,
        vec![gl::FLOAT],
        vec![3],
        (3 * mem::size_of::<GLfloat>()) as _,
        3,
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

    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            match event {
                Event::Quit { .. }
                | Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        let (width, height) = window.drawable_size();
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);

            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let model_matrix = Matrix4::identity();
        let view_matrix = Matrix4::look_at(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let projection_matrix: Matrix4 = cgmath::perspective(
            cgmath::Deg(45.0f32),
            width as f32 / height as f32,
            0.1,
            100.0,
        );

        unsafe {
            use c_str_macro::c_str;
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);
        }

        vertex_obj.draw();

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        let ui = imgui.frame();
        use imgui::im_str;
        imgui::Window::new(im_str!("Information"))
            .size([300.0, 200.0], imgui::Condition::FirstUseEver)
            .build(&ui, || {
                ui.text(im_str!("OpenGL Test App ver 1.0"));
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
            });
        imgui_renderer.render(ui);

        window.gl_swap_window();

        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60)); // 60FPS
    }
}
