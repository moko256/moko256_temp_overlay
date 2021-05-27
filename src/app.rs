use chrono::{DateTime, Duration, Local, UTC};

use crate::{
    render_primitives::{Align, RenderPrimitives},
    temp_retriever,
};

pub struct App {
    width: f32,
    height: f32,
    need_redraw: bool,
    last_updated: DateTime<UTC>,
    show_state: ShowState,
    pub primitives: Vec<RenderPrimitives>,
}

#[derive(PartialEq)]
struct ShowState {
    time: String,
    cpu_temp: String,
    gpu_temp: String,
}

impl App {
    pub fn new(width: f32, height: f32) -> App {
        let mut app = App {
            width,
            height,
            need_redraw: true,
            last_updated: UTC::now(),
            show_state: App::update_show_state(),
            primitives: Vec::with_capacity(0),
        };
        app.invalidate();
        app
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.invalidate();
    }

    pub fn update_and_check_need_redraw(&mut self) -> bool {
        // If true, redraw is needed.
        let now_instant = UTC::now();
        if (now_instant - self.last_updated) >= Duration::seconds(1) {
            self.last_updated = now_instant;
            let next_show_state = App::update_show_state();
            if self.show_state != next_show_state {
                self.show_state = next_show_state;
                self.invalidate();
            }
        }

        let result = self.need_redraw;
        if result {
            self.need_redraw = false;
        }
        result
    }

    fn update_show_state() -> ShowState {
        ShowState {
            time: Local::now().format("%H:%M").to_string(),
            cpu_temp: temp_retriever::retrieve_cpu_temp_c()
                .map_or_else(|| "--".to_string(), |x| x.to_string())
                + "℃",
            gpu_temp: temp_retriever::retrieve_gpu_temp_c()
                .map_or_else(|| "--".to_string(), |x| x.to_string())
                + "℃",
        }
    }

    fn invalidate(&mut self) {
        self.need_redraw = true;
        self.primitives = vec![
            RenderPrimitives::Clear {
                color: (0x000000, 0.4),
            },
            RenderPrimitives::Text {
                rect: (0.0, 0.0, 64.0, self.height),
                text: self.show_state.time.clone(),
                size: 18.0,
                color: (0xFFFFFF, 1.0),
                v_align: Align::Center,
                h_align: Align::Center,
            },
            RenderPrimitives::Text {
                rect: (64.0, 0.0, 128.0, self.height),
                text: self.show_state.cpu_temp.clone(),
                size: 18.0,
                color: (0xFFFFFF, 1.0),
                v_align: Align::Center,
                h_align: Align::End,
            },
            RenderPrimitives::Text {
                rect: (128.0, 0.0, 192.0, self.height),
                text: self.show_state.gpu_temp.clone(),
                size: 18.0,
                color: (0xFFFFFF, 1.0),
                v_align: Align::Center,
                h_align: Align::End,
            },
        ];
    }
}
