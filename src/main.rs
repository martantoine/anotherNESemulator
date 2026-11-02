use imgui::Context;
use imgui_glow_renderer::{
    glow::{self, HasContext},
    AutoRenderer,
};
use imgui_sdl2_support::SdlPlatform;
use sdl2::{
    event::Event,
    video::{GLProfile, Window},
    rect::Rect,
    pixels::Color,
};
use std::rc::Rc;
use std::cell::RefCell;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    let window = video_subsystem
        .window("SDL2 + ImGui", 1280, 720)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    window.subsystem().gl_set_swap_interval(1).unwrap();

    let gl = glow_context(&window);
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let mut platform = SdlPlatform::new(&mut imgui);
    let mut renderer = AutoRenderer::new(gl, &mut imgui).unwrap();

    // Create SDL2 canvas for rendering 2D content (rectangles, textures, etc.)
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            platform.handle_event(&mut imgui, &event);
            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        // Prepare ImGui frame
        platform.prepare_frame(&mut imgui, &canvas.window(), &event_pump);
        let ui = imgui.new_frame();

        // Draw SDL2 content here
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear(); // Clear the screen with black

        // Draw SDL2 rectangles or other primitives
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(50, 50, 200, 200)).unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(Rect::new(300, 50, 200, 200)).unwrap();

        // Present the SDL2 content
        canvas.present();  // Important: make sure SDL2 content is presented before switching to OpenGL!

        // Now switch to OpenGL and render ImGui on top
        let draw_data = imgui.render();

        // Clear OpenGL state
        unsafe {
            let gl_ref = gl.borrow();
            gl_ref.clear_color(0.0, 0.0, 0.0, 1.0);  // You can set transparent background for imgui if needed
            gl_ref.clear(glow::COLOR_BUFFER_BIT);
        }

        // Render ImGui on top
        renderer.render(draw_data).unwrap();

        // Swap OpenGL buffers to show ImGui
        canvas.window().gl_swap_window();
    }
}
