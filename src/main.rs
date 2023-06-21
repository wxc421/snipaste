use sdl2::video::{Window, WindowBuilder};
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut window1 = create_window(&video, 0);
    let id1 = window1.id();

    // let mut canvas = window1.into_canvas().build().unwrap();
    // canvas.set_draw_color(Color::RGBA(0, 0, 0, 100));
    // canvas.clear();
    // canvas.present();

    let window2 = create_window(&video, 1);
    let id2 = window2.id();

    // 进行事件循环
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut start_pos = None;
    let mut end_pos = None;

    let mut canvas1 = window1.into_canvas().build().unwrap();
    let mut canvas2 = window2.into_canvas().build().unwrap();


    let mut last_windows_id = 0;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window { win_event, window_id, .. } => {
                    // 根据窗口ID判断是哪个窗口触发了事件
                    if window_id == id1 {
                        // 窗口1触发的事件
                        println!("Window 1 event: {:?}", win_event);
                    } else if window_id == id2 {
                        // 窗口2触发的事件
                        println!("Window 2 event: {:?}", win_event);
                    }
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
        // 清除画布
        canvas1.set_draw_color(Color::RGB(255, 255, 255));
        canvas1.clear();

        canvas2.set_draw_color(Color::RGB(255, 255, 255));
        canvas2.clear();

        // 绘制当前矩形
        if let (Some(start), Some(end)) = (start_pos, end_pos) {
            let rect = Rect::new(start.0, start.1, (end.0 - start.0) as u32, (end.1 - start.1) as u32);
            canvas1.set_draw_color(Color::RGB(255, 0, 0));
            canvas1.fill_rect(rect).unwrap();
        }
        canvas1.window_mut().show();
        canvas2.window_mut().show();
        // 将画布显示在窗口上
        canvas1.present();
        canvas2.present();
    }
}

// 创建窗口并返回窗口和事件循环
fn create_window(video: &VideoSubsystem, display_index: i32) -> Window {
    let display_bounds = video.display_bounds(display_index).unwrap();

    let mut window = WindowBuilder::new(
        video,
        &format!("Window {}", display_index),
        display_bounds.width() as u32 - 200,
        display_bounds.height() as u32 - 200,
    )
        .position(display_bounds.x(), display_bounds.y())
        .hidden()
        .build()
        .unwrap();

    // window.set_opacity(0.5).expect("TODO: panic message");

    window
}