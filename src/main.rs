extern crate hlua;
extern crate sdl2;

use hlua::{AnyLuaValue, Lua};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

static TARGET_FPS: u32 = 30;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Abbiocco", 512, 512)
        .position_centered()
        .build()
        .unwrap();

    let canvas = RefCell::new(window.into_canvas().build().unwrap());

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut lua = Lua::new();

    lua.set(
        "rectfill",
        hlua::function4(|x0: i32, y0: i32, x1: u32, y1: u32| {
            canvas
                .borrow_mut()
                .set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .borrow_mut()
                .fill_rect(Rect::new(x0, y0, x1, y1))
                .unwrap();
        }),
    );

    lua.set(
        "circfill",
        hlua::function3(|x: i32, y: i32, r: u32| {
            // canvas.borrow_mut().set_draw_color(Color::RGB(255, 210, 0));
        }),
    );

    lua.execute_from_reader::<(), _>(File::open(&Path::new("resources/main.lua")).unwrap())
        .unwrap();

    'running: loop {
        // Start draw stuff

        canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
        canvas.borrow_mut().clear();

        {
            let mut update_func: hlua::LuaFunction<_> = lua.get("_update").unwrap();
            let _update_ret: hlua::AnyLuaValue = update_func.call().unwrap();
        }

        {
            let mut draw_func: hlua::LuaFunction<_> = lua.get("_draw").unwrap();
            let _draw_ret: hlua::AnyLuaValue = draw_func.call().unwrap();
        }

        // let cur_frame: i32 = lua.get("cur_frame").unwrap();
        // println!("{:?}", cur_frame);

        // End draw stuff

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.borrow_mut().present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
