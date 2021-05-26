use direct2d::{
    brush::SolidColorBrush,
    enums::DrawTextOptions,
    render_target::{GenericRenderTarget, HwndRenderTarget},
    stroke_style::StrokeStyleBuilder,
    RenderTarget,
};
use directwrite::{
    enums::{ParagraphAlignment, TextAlignment},
    TextFormat,
};
use std::ffi::c_void;
use winapi::shared::windef::HWND__;

use crate::render_primitives::{Align, RenderPrimitives};

pub struct AppRenderer {
    render_target: GenericRenderTarget,
    d2d_factory: direct2d::Factory,
    dwrite_factory: directwrite::Factory,
    hwnd: *mut c_void,
}

impl AppRenderer {
    pub fn new(hwnd: *mut c_void, width: u32, height: u32) -> AppRenderer {
        let dwrite_factory = directwrite::Factory::new().unwrap();
        let d2d_factory = direct2d::factory::Factory::new().unwrap();
        let render_target = AppRenderer::create_render_target(&d2d_factory, hwnd, width, height);

        AppRenderer {
            render_target,
            d2d_factory,
            dwrite_factory,
            hwnd,
        }
    }

    fn create_render_target(
        d2d_factory: &direct2d::Factory,
        hwnd: *mut c_void,
        width: u32,
        height: u32,
    ) -> GenericRenderTarget {
        HwndRenderTarget::create(d2d_factory)
            .with_pixel_size(width, height)
            .with_dpi(96.0, 96.0) // Non scale dpi
            .with_hwnd(hwnd as *mut HWND__)
            .build()
            .unwrap()
            .as_generic()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.render_target =
            AppRenderer::create_render_target(&self.d2d_factory, self.hwnd, width, height);
    }

    pub fn draw(&mut self, render_primitives: &[RenderPrimitives]) {
        self.render_target.begin_draw();

        for p in render_primitives {
            match p {
                RenderPrimitives::Clear { color } => {
                    self.render_target.clear(*color);
                }
                RenderPrimitives::StrokeRect { rect, color } => {
                    let brush = SolidColorBrush::create(&self.render_target)
                        .with_color(*color)
                        .build()
                        .unwrap();
                    self.render_target.draw_rectangle(
                        *rect,
                        &brush,
                        1.0,
                        Some(&StrokeStyleBuilder::new(&self.d2d_factory).build().unwrap()),
                    );
                }
                RenderPrimitives::FillRect { rect, color } => {
                    let brush = SolidColorBrush::create(&self.render_target)
                        .with_color(*color)
                        .build()
                        .unwrap();
                    self.render_target.fill_rectangle(*rect, &brush);
                }
                RenderPrimitives::Text {
                    rect,
                    color,
                    text,
                    size,
                    v_align,
                    h_align,
                } => {
                    let brush = SolidColorBrush::create(&self.render_target)
                        .with_color(*color)
                        .build()
                        .unwrap();

                    let t_format = TextFormat::create(&self.dwrite_factory)
                        .with_size(*size)
                        .with_family("Yu Gothic UI")
                        .with_weight(directwrite::enums::FontWeight::SemiLight)
                        .build()
                        .unwrap();
                    let p_align = match v_align {
                        Align::Start => ParagraphAlignment::Near,
                        Align::Center => ParagraphAlignment::Center,
                        Align::End => ParagraphAlignment::Far,
                    };
                    t_format.set_paragraph_alignment(p_align).unwrap();

                    let t_align = match h_align {
                        Align::Start => TextAlignment::Leading,
                        Align::Center => TextAlignment::Center,
                        Align::End => TextAlignment::Trailing,
                    };
                    t_format.set_text_alignment(t_align).unwrap();

                    self.render_target.draw_text(
                        &text,
                        &t_format,
                        *rect,
                        &brush,
                        DrawTextOptions::ENABLE_COLOR_FONT,
                    );
                }
            }
        }

        self.render_target.end_draw().unwrap();
    }
}
