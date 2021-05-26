#[allow(dead_code)]
pub enum RenderPrimitives {
    Clear {
        color: (u32, f32),
    },
    FillRect {
        rect: (f32, f32, f32, f32),
        color: (u32, f32),
    },
    StrokeRect {
        rect: (f32, f32, f32, f32),
        color: (u32, f32),
    },
    Text {
        rect: (f32, f32, f32, f32),
        color: (u32, f32),
        text: String,
        size: f32,
        v_align: Align,
        h_align: Align,
    },
}

#[allow(dead_code)]
pub enum Align {
    Start,
    Center,
    End,
}
