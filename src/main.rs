pub mod keyboard;
pub mod render_surface;
pub mod shaders;

use egui::style::Margin;
use glium::backend::Facade;
use glium::glutin;
use glium::glutin::event::VirtualKeyCode;
use glium::IndexBuffer;
use glium::Surface;
use glium::VertexBuffer;
use keyboard::Keyboard;
use nalgebra::Perspective3;
use opengl_renderer::renderer::Renderable;
use opengl_renderer::renderer::Renderer;
use opengl_renderer::utils::camera::Camera;
use opengl_renderer::{system_loop::SystemLoop, window::Window};
use render_surface::RenderSurface;
use shaders::mandelbrot::MandelbrotShader;
use std::rc::Rc;
use std::sync::Mutex;

fn main() {
    // generate window and event loop
    let window = create_window();
    let facade = window.display.clone().get_context().clone();
    let mut event_loop = SystemLoop::new(window);

    // window visibility state handling.
    let mut debug_open = false;
    let mut help_open = false;

    // generate the textures that will be rendered to.
    let mut render_texture = RenderSurface::new(&facade, 100, 100).unwrap();
    let egui_texture: egui::TextureId = event_loop
        .get_egui_glium_mut()
        .painter
        .register_native_texture(render_texture.texture.clone(), Default::default());

    let mut renderer = Renderer::new();

    let mut camera = Camera::new();
    camera.position = [0.0, 0.0, 3.0].into();

    // generate the quat that will be used for the mandelbrot shader
    let square = opengl_renderer::utils::shapes::get_quad();
    let square_buffers = (
        VertexBuffer::new(&facade, &square).unwrap(),
        IndexBuffer::new(
            &facade,
            glium::index::PrimitiveType::TriangleStrip,
            &[0u32, 1, 2, 1, 3, 2],
        )
        .unwrap(),
    );

    // load mandelbrot shader
    let mut mandelbrot = MandelbrotShader::load_from_fs(&facade);

    // start listening to keyboard events
    let keyboard = keyboard_listener(&mut event_loop);

    // add a render loop
    event_loop.subscribe_render(move |render_info| {
        render_info.target.clear_color(0.0, 0.0, 0.0, 1.0);

        // handle keyboard inputs
        {
            let keyboard = keyboard.lock().unwrap();

            // define base speeds of both panning and zooming. This will be based on the frame time.
            let mut speed = 0.75 * render_info.delta.as_secs_f32() * mandelbrot.zoom as f32;
            let mut zoom_speed =
                (0.5 * render_info.delta.as_secs_f32() * mandelbrot.zoom).max(f32::MIN);

            // Slow down the pan and zoom speed if shift is pressed
            if keyboard.get_shift() {
                speed *= 0.2;
                zoom_speed *= 0.2;
            }

            // pan the camera
            if keyboard.get_key(&VirtualKeyCode::W) {
                camera.position.y += speed;
            }
            if keyboard.get_key(&VirtualKeyCode::S) {
                camera.position.y -= speed;
            }
            if keyboard.get_key(&VirtualKeyCode::D) {
                camera.position.x += speed;
            }
            if keyboard.get_key(&VirtualKeyCode::A) {
                camera.position.x -= speed;
            }

            // zoom the camera
            if keyboard.get_key(&VirtualKeyCode::E) {
                mandelbrot.zoom -= zoom_speed;
            }
            if keyboard.get_key(&VirtualKeyCode::Q) {
                mandelbrot.zoom += zoom_speed;
            }

            // reset the camera
            if keyboard.get_key(&VirtualKeyCode::R) {
                mandelbrot.zoom = 1.0;
                camera.position *= 0.0;
            }
        }

        egui::TopBottomPanel::top("topbar").show(&render_info.egui_glium.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("show debug").clicked() {
                    debug_open = true;
                }
                if ui.button("show help").clicked() {
                    help_open = true;
                }
            });
        });

        egui::Window::new("help")
            .collapsible(false)
            .open(&mut help_open)
            .show(&render_info.egui_glium.egui_ctx, |ui| {
                ui.label("W: Up; A: Left; S: Down; D: Right");
                ui.label("Q: Zoom Out; E: Zoom In");
                ui.label("R: Reset Camera");
            });

        // Display some basic debug information like fps
        egui::Window::new("debug")
            .collapsible(false)
            .anchor(egui::Align2::RIGHT_TOP, [0.0; 2])
            .open(&mut debug_open)
            .show(&render_info.egui_glium.egui_ctx, |ui| {
                ui.label(&format!(
                    "time: {:.2}",
                    render_info.delta.as_secs_f32() * 1000.0,
                ));

                ui.label(&format!(
                    "fps: {:.2}",
                    1.0 / render_info.delta.as_secs_f32()
                ));

                ui.label(&format!(
                    "res: {}x{}",
                    render_texture.width(),
                    render_texture.height()
                ));

                ui.label(&format!("polygons: {}", renderer.get_polygons()));
            });

        // Main rendering context
        egui::CentralPanel::default()
            .frame(egui::Frame::none().inner_margin(Margin::same(0.0)))
            .show(&render_info.egui_glium.egui_ctx, |ui| {
                let size = ui.available_size();

                // resize the render texture if the window size changed
                let mut size_px = ui.available_size();
                size_px.x *= render_info.egui_glium.egui_ctx.pixels_per_point();
                size_px.y *= render_info.egui_glium.egui_ctx.pixels_per_point();

                if size_px.x != render_texture.width() as f32
                    || size_px.y != render_texture.height() as f32
                {
                    render_texture
                        .resize(&facade, size_px.x as u32, size_px.y as u32)
                        .unwrap();
                    render_info.egui_glium.painter.replace_native_texture(
                        egui_texture,
                        render_texture.texture.clone(),
                        egui::TextureOptions::default(),
                    );
                }

                // set up buffer to be rendered to
                let mut buffer = render_texture.frame_buffer(&facade).unwrap();
                buffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                // generate and populate the scene
                let mut scene = renderer.begin_scene();

                scene.scene_data.projection = Perspective3::new(
                    render_texture.width().max(1) as f32 / render_texture.height().max(1) as f32,
                    70.0f32.to_radians(),
                    0.1,
                    100000.0,
                )
                .as_matrix()
                .clone()
                .into();

                scene.scene_data.camera = camera.clone();

                // set scene vars for screen size
                scene.scene_data.set_scene_object(ScreenDim {
                    width: render_texture.width(),
                    height: render_texture.height(),
                });

                // give the scene the mandelbrot data
                mandelbrot.pos = camera.position.xy().into();
                scene.publish(&square_buffers.0, &square_buffers.1, &mandelbrot);

                // render scene to texture
                scene.finish(&mut Renderable::from(&mut buffer));

                // show our rendered texture, but the image is upside, down so let's change the uv
                // coords of the image
                ui.add(
                    egui::widgets::Image::new(egui_texture, size).uv(egui::Rect {
                        min: [0.0, 1.0].into(),
                        max: [1.0, 0.0].into(),
                    }),
                );
            });
    });

    // start the application loop
    event_loop.start();
}

fn create_window() -> Window {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("Mandelbrot Viewer");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_stencil_buffer(0)
        .with_vsync(true)
        .with_srgb(true);

    Window::create(window_builder, context_builder)
}

fn keyboard_listener(event_loop: &mut SystemLoop) -> SharedKeyboard {
    let main_keyboard = Rc::new(Mutex::new(Keyboard::new()));
    let keyboard = main_keyboard.clone();
    event_loop.subscribe_events(move |event| {
        use glutin::event::*;

        match event {
            Event::DeviceEvent { event, .. } => {
                if let DeviceEvent::Key(key) = event {
                    if let Some(keycode) = key.virtual_keycode {
                        keyboard
                            .lock()
                            .unwrap()
                            .set_key(keycode, key.state == ElementState::Pressed);
                    }
                }
            }
            _ => (),
        };
    });
    main_keyboard
}

pub struct ScreenDim {
    width: u32,
    height: u32,
}

pub type SharedKeyboard = Rc<Mutex<Keyboard>>;
