use raylib::prelude::*;
use std::error::Error;
use std::sync::{Arc, Mutex};

mod utils;

struct ColorPalette {
    background: Color,
    line: Color,
    button: Color,
    text: Color,
}

pub struct Gui {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    palette: &'static ColorPalette,
    dark_mode_button: Rectangle,
    button_text: String,
    latest_msg: Arc<Mutex<String>>, 
    input_fields: [Rectangle; 3],
    input_texts: [String; 3],
    are_inputs_active: [bool; 3],
    current_palette_index: usize, // Keep track of the current palette
}

const PALETTES: &[&ColorPalette] = &[&LIGHT_PALETTE, &DARK_PALETTE, &BONITA, &MONOKAI, &GRUVBOX, &FOREST, &CYBERPUNK];

const PALETTE_NAMES: &[&str] = &["Light Mode", "Dark Mode", "Bonita", "Monokai", "Gruvbox", "Forest", "Cyberpunk"];


impl Gui {
    pub fn new(width: i32, height: i32) -> Result<Self, Box<dyn Error>> {
        unsafe {
            raylib::ffi::SetConfigFlags(raylib::ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
        }
        let (rl, thread) = raylib::init()
            .size(width, height)
            .title("GUI")
            .build();

        let is_dark_mode = false;
        let palette = if is_dark_mode { &DARK_PALETTE } else { &LIGHT_PALETTE };
        let dark_mode_button = Rectangle::new(0.0, 0.0, width as f32, height as f32);
        let button_text = String::from("Light Theme"); 

        let input_fields = [Rectangle::new(0.0, 0.0, 0.0, 0.0); 3];
        let input_texts = [String::new(), String::new(), String::new()];
        let are_inputs_active = [false, false, false];
        let current_palette_index = 0;

        //ROS
        let latest_msg = Arc::new(Mutex::new(String::new())); // Initialize as Arc<Mutex<String>>
        let latest_msg_clone = latest_msg.clone(); // Clone for the subscriber

        let _node = rosrust::init("gui");
        let _subscriber = rosrust::subscribe("chatter", 2, move |v: rosrust_msg::std_msgs::String| {
            // Callback for handling received messages
            rosrust::ros_info!("Received: {}", v.data);
            let mut msg = latest_msg_clone.lock().unwrap(); // Acquire lock
            *msg = v.data; // Update the message
        })
      .unwrap();

        Ok(Gui {
            rl,
            thread,
            palette,
            dark_mode_button,
            button_text,
            latest_msg,
            input_fields,
            input_texts,
            are_inputs_active,
            current_palette_index,
        })
    }

    pub fn render(&mut self) {
        let latest_msg_clone = self.latest_msg.clone();
            let _subscriber_info = rosrust::subscribe("chatter", 2, move |v: rosrust_msg::std_msgs::String| {
                // Callback for handling received messages
                let mut msg = latest_msg_clone.lock().unwrap(); // Acquire lock within the callback
                *msg = v.data; // Update the message
            })
          .unwrap();
        let button_text = String::from(PALETTE_NAMES[self.current_palette_index]);
        
        println!("{}", button_text);

        while !self.rl.window_should_close(){
            let height = self.rl.get_screen_height();
            let width = self.rl.get_screen_width();
            self.dark_mode_button = Rectangle::new(0.0, height as f32 - 40.0, width as f32, 40.0);

            let input_width = 80.0;
            let input_height = 20.0;
            let input_y = height as f32 - 200.0;
            let input_x_starts = [10.0, 110.0, 210.0];

            for i in 0..3 {
                self.input_fields[i] = Rectangle::new(input_x_starts[i], input_y, input_width, input_height);
            }


//COLLISION CHECK FOR THE BOXES
            if self.rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let mouse_position = self.rl.get_mouse_position();
    
                if self.dark_mode_button.check_collision_point_rec(mouse_position) {
                    // Cycle through the palettes
                    self.current_palette_index = (self.current_palette_index + 1) % PALETTES.len();
                    self.palette = PALETTES[self.current_palette_index];
                    self.button_text = String::from(PALETTE_NAMES[self.current_palette_index]);
                }
                for i in 0..3 {
                    if self.input_fields[i].check_collision_point_rec(mouse_position) {
                        self.are_inputs_active[i] = true;
                    } else {
                        self.are_inputs_active[i] = false;
                    }
                }
            }


// GET THE INPUT
            for i in 0..3 {
                if self.are_inputs_active[i] {
                    if self.rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_BACKSPACE) {
                        if !self.input_texts[i].is_empty() {
                            self.input_texts[i].pop();
                        }
                    }

                    while let Some(key) = self.rl.get_char_pressed() {
                        self.input_texts[i].push(key);
                    }

                    // Check for Enter key press *while* the input is active
                    if self.rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
                        self.are_inputs_active[i] = false; // Deactivate input field
                        utils::process_input(
                            &self.input_texts[0],
                            &self.input_texts[1],
                            &self.input_texts[2],
                            "publish"
                        );
                        // Optionally clear input fields after processing:
                        self.input_texts = [String::new(), String::new(), String::new()];
                    }
                }
            }

//DRAWING
            let mut d = self.rl.begin_drawing(&self.thread);
            let msg = self.latest_msg.lock().unwrap();  
            d.clear_background(self.palette.background);
            d.draw_rectangle(0, height - 40, width, height / 10, self.palette.button);
            d.draw_fps(width - 100, height - 30);
            d.draw_text(self.button_text.as_str(), 50, height - 30, 20, self.palette.text);
            d.draw_line(width/2, 0, width/2, height, self.palette.line);
            //Pick Robot
            let text_pick = "Pick Robot";
            let text_pick_width = d.measure_text(text_pick, 30);
            d.draw_text(text_pick, (width / 4) - (text_pick_width / 2), 20, 30, self.palette.line);
            d.draw_text(&*msg,50,height-100,20,self.palette.line);
            d.draw_text("x", 45, height-225, 17, self.palette.line);
            d.draw_text("y", 145, height-225, 17, self.palette.line);
            d.draw_text("z", 245, height-225, 17, self.palette.line);
            //Inspection Robot
            let text_inspect = "Inspection Robot";
            let text_inspect_width = d.measure_text(text_inspect, 30);
            d.draw_text(text_inspect, (width / 4) - (text_inspect_width / 2) + (width/2), 20, 30, self.palette.line);
            d.draw_text(&*msg,(width/2)+50,height-100,20,self.palette.line);

            //drawing input fields
            for i in 0..3 {
                d.draw_rectangle_rec(self.input_fields[i], self.palette.button);
                d.draw_text(&self.input_texts[i], (self.input_fields[i].x + 5.0) as i32, (self.input_fields[i].y + 5.0) as i32, 10, self.palette.text); // Offset text slightly
            }

        }
    }
    
}


const LIGHT_PALETTE: ColorPalette = ColorPalette {
    background: Color::new(0xF5, 0xF5, 0xF5, 255), // #F5F5F5
    line: Color::new(0x18, 0x18, 0x18, 0xD6), // #3c3c3c
    button: Color::new(0x18, 0x18, 0x18, 255),
    text: Color::new(0xF5, 0xF5, 0xF5, 255),
};

const DARK_PALETTE: ColorPalette = ColorPalette {
    background: Color::new(0x18, 0x18, 0x18, 255),
    line: Color::new(0xF5, 0xF5, 0xF5, 255), // #F5F5F5
    button: Color::new(0xF5, 0xF5, 0xF5, 255),
    text: Color::new(0x18, 0x18, 0x18, 255),
};

const BONITA: ColorPalette = ColorPalette {
    background: Color::new(0xFF, 0xC0, 0xCB, 255), // #FFC0CB (Pink)
    line: Color::new(0xFF, 0x14, 0x93, 255), // #FF1493 (Deep Pink)
    button: Color::new(0xDB, 0x70, 0x93, 255), // #DB7093 (Pale Violet Red)
    text: Color::new(0x8B, 0x00, 0x8B, 255), // #8B008B (Dark Magenta)
};

const MONOKAI: ColorPalette = ColorPalette {
    background: Color::new(39, 40, 34, 255), // #272822
    line: Color::new(248, 248, 242, 255), // #F8F8F2
    button: Color::new(102, 217, 239, 255), // #66D9EF
    text: Color::new(248, 248, 242, 255), // #F8F8F2
};
const GRUVBOX: ColorPalette = ColorPalette {
    background: Color::new(40, 40, 40, 255), // #282828
    line: Color::new(235, 219, 178, 255), // #EBDBB2
    button: Color::new(53, 33, 0, 255), // #352100
    text: Color::new(213, 196, 161, 255), // #D5C4A1
};

const FOREST: ColorPalette = ColorPalette {
    background: Color::new(34, 49, 34, 255), // Dark green
    line: Color::new(85, 107, 47, 255), // Olive drab
    button: Color::new(107, 142, 35, 255), // Olive drab
    text: Color::new(173, 255, 47, 255), // Green yellow
};

const CYBERPUNK: ColorPalette = ColorPalette {
    background: Color::new(10, 10, 10, 255), // Almost black
    line: Color::new(255, 20, 147, 255), // Deep pink
    button: Color::new(0, 255, 255, 255), // Cyan
    text: Color::new(255, 255, 0, 255), // Yellow
};