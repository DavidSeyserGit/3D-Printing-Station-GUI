use raylib::prelude::*;
use std::error::Error;
use std::path::Path;
mod utils;

struct ColorPalette {
    background: Color,
    triangle: Color,
    line: Color,
    button: Color,
    text: Color,
    gridline: Color,
}

const LIGHT_PALETTE: ColorPalette = ColorPalette {
    background: Color::new(0xF5, 0xF5, 0xF5, 255), // #F5F5F5
    triangle: Color::new(0xEB, 0x47, 0x33, 255), // #eb4733
    line: Color::new(0x18, 0x18, 0x18, 0xD6), // #3c3c3c
    button: Color::new(0x18, 0x18, 0x18, 255),
    text: Color::new(0xF5, 0xF5, 0xF5, 255),
    gridline: Color::new(0x18, 0x18, 0x18, 0xD6),
};

const DARK_PALETTE: ColorPalette = ColorPalette {
    background: Color::new(0x18, 0x18, 0x18, 255),
    triangle: Color::new(0xEB, 0x47, 0x33, 255),
    line: Color::new(0xF5, 0xF5, 0xF5, 255), // #F5F5F5
    button: Color::new(0xF5, 0xF5, 0xF5, 255),
    text: Color::new(0x18, 0x18, 0x18, 255),
    gridline: Color::new(0xA1, 0xA1, 0xA1, 0xA5), // #A1A1A1
};

pub struct Gui {
    pub width: i32,
    pub height: i32,
    pub camera: Camera3D,
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    palette: &'static ColorPalette,
    dark_mode_button: Rectangle,
    is_dark_mode: bool,
    button_text: String,
}

impl Gui {
    pub fn new(width: i32, height: i32) -> Result<Self, Box<dyn Error>> {
        unsafe {
            raylib::ffi::SetConfigFlags(raylib::ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
        }
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title("Slicer6D")
            .build();

        let camera = Camera3D::perspective(
            Vector3 { x: 200.0, y: 300.0, z: 700.0 },
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            45.0,
        );

        let is_dark_mode = false;
        let palette = if is_dark_mode { &DARK_PALETTE } else { &LIGHT_PALETTE };
        let dark_mode_button = Rectangle::new(0.0, 0.0, width as f32, height as f32);
        let button_text = String::from("Dark Mode");
        Ok(Gui {
            width,
            height,
            camera, //might not be need when only 2D
            rl,
            thread,
            palette,
            dark_mode_button,
            is_dark_mode,
            button_text,
        })
    }

    pub fn render(&mut self) {
        while !self.rl.window_should_close(){
            let height = self.rl.get_screen_height();
            let width = self.rl.get_screen_width();
            self.dark_mode_button = Rectangle::new(0.0, height as f32 - 40.0, width as f32, 40.0);

            if self.rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_position = self.rl.get_mouse_position();
    
                if self.dark_mode_button.check_collision_point_rec(mouse_position) {
                    self.is_dark_mode = !self.is_dark_mode;
                    self.palette = if self.is_dark_mode { &DARK_PALETTE } else { &LIGHT_PALETTE };
                    self.button_text = if self.is_dark_mode {
                        String::from("Light Mode")
                    } else {
                        String::from("Dark Mode")
                    };
                }
            }
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(self.palette.background);
            d.draw_circle(width/2, height/2, (width/20) as f32, self.palette.line);
            d.draw_rectangle(0, height - 40, width, height / 10, self.palette.button);
            d.draw_text("Darkmode", 50, height - 30, 20, self.palette.text);
            d.draw_fps(width - 100, height - 30);
        }
    }
}
