// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::time::Duration;

use device_query;
use device_query::DeviceQuery;
use device_query::DeviceState;
use display_info::DisplayInfo;
use tauri::Manager;
use tauri::PhysicalPosition;
use tauri::PhysicalSize;
use tauri::Position;
use windows::Win32::Graphics::Gdi::GetDC;
use windows::Win32::Graphics::Gdi::GetPixel;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let display_infos = DisplayInfo::all().unwrap();
            // don't want to deal with all monitor layouts yet, just using main monitor
            let monitor = display_infos[0].clone();
            let width = monitor.width as f32;
            let height = monitor.height as f32;
            let window_width = 200.0;
            let window_height = 100.0;

            let state = DeviceState::new();

            let window = app.get_window("main").unwrap();
            let _ = window.set_size(PhysicalSize {
                width: window_width,
                height: window_height,
            });
            let _ = window.set_resizable(false);
            tauri::async_runtime::spawn(async move {
                loop {
                    let key = state.get_keys();
                    if key.contains(&device_query::Keycode::LControl) {
                        let coords = state.get_mouse().coords;

                        let rx = coords.0 as f32 / width - 0.5;
                        let ry = coords.1 as f32 / height - 0.5;
                        if rx > 0.0 && ry > 0.0 {
                            // lower right
                            let _r = window.set_position(Position::Physical(PhysicalPosition {
                                x: (coords.0 as f32 - window_width) as i32,
                                y: (coords.1 as f32 - window_height) as i32,
                            }));
                        } else if rx < 0.0 && ry > 0.0 {
                            let _r = window.set_position(Position::Physical(PhysicalPosition {
                                x: coords.0,
                                y: (coords.1 as f32 - window_height) as i32,
                            }));
                        } else if rx < 0.0 && ry < 0.0 {
                            // upper left
                            let _r = window.set_position(Position::Physical(PhysicalPosition {
                                x: coords.0,
                                y: coords.1,
                            }));
                        } else {
                            // upper right
                            let _r = window.set_position(Position::Physical(PhysicalPosition {
                                x: (coords.0 as f32 - window_width) as i32,
                                y: coords.1,
                            }));
                        };

                        let hex = get_pixel(coords.0, coords.1);
                        let _ = window.emit("color", hex);
                        // println!("{:?}", coords);
                        std::thread::sleep(Duration::from_micros(100));
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_pixel(x: i32, y: i32) -> Vec<u8> {
    unsafe {
        let hwnd = GetDC(None);

        let a = GetPixel(hwnd, x, y).0.to_le_bytes();

        let r = a[0];
        let g = a[1];
        let b = a[2];
        vec![r, g, b]
    }
}
