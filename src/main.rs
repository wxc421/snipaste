use std::{fs, thread};
use std::time::Duration;
use sdl2::video::{Window, WindowBuilder};
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use screenshots::Screen;


// pub fn capture<'a>() -> Surface<'a> {
//     let screens = Screen::all().unwrap();
//     let image = screens[0].capture().unwrap();
//     let width = image.width();
//     let height = image.height();
//     let mut vec: Vec<u8> = image.into();
//
//     let surface = Surface::from_data(
//         vec.as_mut_slice(),
//         width,
//         height,
//         width * height * 3,
//         PixelFormatEnum::Unknown,
//     ).unwrap();
//
//     surface
// }

fn main() {

    let screens = Screen::all().unwrap();
    let image = screens[0].capture().unwrap();
    let width = image.width();
    let height = image.height();
    let mut vec: Vec<u8> = image.into();
    println!("{:?}",&vec[..160]);

    // let mut buffer = image.to_png().unwrap();
    // fs::write(format!("target/777777-{}.png", screens[0].display_info.id), buffer).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut window1 = create_window(&video, 0);
    let id = window1.id();

    // 进行事件循环
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut start_pos = None;
    let mut end_pos = None;

    let mut canvas = window1.into_canvas().build().unwrap();




    let surface = Surface::from_data(
        vec.as_mut_slice(),
        width,
        height,
        width * 4,
        // PixelFormatEnum::RGBA32,
        PixelFormatEnum::BGRA32,
    ).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(surface).unwrap();
    texture.with_lock()

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window { win_event, window_id, .. } => {
                    // 根据窗口ID判断是哪个窗口触发了事件
                    if window_id == id {
                        // 窗口1触发的事件
                        println!("Window 1 event: {:?}", win_event);
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main;
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        start_pos = Some((x as i32, y as i32));
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == sdl2::mouse::MouseButton::Left {
                        end_pos = start_pos;
                        start_pos = None;
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    if let Some((start_x, start_y)) = start_pos {
                        end_pos = Some((x as i32, y as i32));
                    }
                }
                // 其他类型的事件
                _ => {}
            }
        }

        // 设置绘制颜色模式为混合模式
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        // 设置透明度
        // canvas.set_draw_color(Color::RGBA(0, 0, 255, 128));
        // thread::sleep(Duration::from_secs(5));
        // canvas.window_mut().set_opacity(0.5);
        // thread::sleep(Duration::from_secs(5));
        // 清除画布
        // canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        // canvas.clear();


        let masks = PixelFormatEnum::RGB24.into_masks().unwrap();

        // let surface = Surface::from_pixelmasks(512, 512, masks).unwrap();

        // println!("vec.len():{}", vec.len());
        // println!("{}", width);
        // println!("{}", height);


        // 绘制当前矩形
        if let (Some(start), Some(end)) = (start_pos, end_pos) {
            let rect = Rect::new(start.0, start.1, (end.0 - start.0) as u32, (end.1 - start.1) as u32);


            canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));
            canvas.fill_rect(rect).unwrap();


            let mut new_texture = sdl2::render::Texture::new(&canvas.texture_creator, PixelFormatEnum::RGBA8888, sdl2::render::TextureAccess::Static)
                .map_err(|e| format!("Could not create texture: {}", e))?;

            new_texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
                let surface = background_texture.query();
                let (width, height) = (surface.width as usize, surface.height as usize);
                let source_pixels = background_texture.without_lock().unwrap();
                let source_pitch = surface.pitch as usize;
                for y in 0..selected_area_rect.height() {
                    let source_y = selected_area_rect.y() as usize + y;
                    let source_row = &source_pixels[(source_y * source_pitch)..((source_y + 1) * source_pitch)];
                    let target_row = &mut pixels[(y * pitch)..((y + 1) * pitch)];
                    let source_start = selected_area_rect.x() as usize * 4;
                    let source_end = source_start + selected_area_rect.width() as usize * 4;
                    let target_start = 0;
                    let target_end = selected_area_rect.width() as usize * 4;
                    target_row[target_start..target_end].copy_from_slice(&source_row[source_start..source_end]);
                }
            })?;

            canvas.copy(&new_texture, None, Some(selected_area_rect))?
        }
        // let src_rect = None; // 使用整个纹理
        let dst_rect = sdl2::rect::Rect::new(100, 100, 200, 200); // 目标区域的矩形坐标
        canvas.copy(&texture, None, None).expect("TODO: panic message");
        // 将画布显示在窗口上
        canvas.present();

        canvas.window_mut().show();
    }
}

// 创建窗口并返回窗口和事件循环
fn create_window(video: &VideoSubsystem, display_index: i32) -> Window {
    let display_bounds = video.display_bounds(display_index).unwrap();

    let mut window = WindowBuilder::new(
        video,
        &format!("Window {}", display_index),
        display_bounds.width() as u32,
        display_bounds.height() as u32,
    )
        .position(display_bounds.x(), display_bounds.y())
        .hidden()
        .borderless()
        .build()
        .unwrap();

    // window.set_opacity(0.4).expect("TODO: panic message");

    window
}