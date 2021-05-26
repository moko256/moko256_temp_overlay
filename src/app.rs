use crate::render_primitives::{Align, RenderPrimitives};

pub struct App<'a> {
    width: f32,
    height: f32,
    need_redraw: bool,
    pub primitives: Vec<RenderPrimitives<'a>>,
}

impl App<'_> {
    pub fn new<'a>(width: f32, height: f32) -> App<'a> {
        let mut app = App {
            width,
            height,
            need_redraw: true,
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

    pub fn need_redraw_and_consume(&mut self) -> bool {
        let result = self.need_redraw;
        if result {
            self.need_redraw = false;
        }
        result
    }

    fn invalidate(&mut self) {
        self.need_redraw = true;
        self.primitives = vec![
                    RenderPrimitives::Clear {
                        color: (0x000000, 1.0)
                    },
                    RenderPrimitives::Text {
                        rect: (
                            32.0,
                            30.0,
                            self.width - 32.0,
                            self.height - 54.0,
                        ),
                        text: "aあ漢 🎈🦀",
                        size: 20.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::Start,
                        h_align: Align::Start,
                    },
                    RenderPrimitives::Text {
                        rect: (
                            32.0,
                            self.height/2.0,
                            self.width -    32.0,
                            self.height,
                        ),
                        text: "吾輩ハ猫デアル - 夏目漱石\n\n吾輩は猫である。名前はまだ無い。どこで生まれたか頓と見當がつかぬ。何ても暗薄いじめじめした所でニャー／＼泣いて居た事丈は記憶して居る。吾輩はこゝで始めて人間といふものを見た。然もあとで聞くとそれは書生といふ人間で一番獰惡な種族であつたさうだ。此書生といふのは時々我々を捕へて煮て食ふといふ話である。然し其當時は何といふ考もなかつたから別段恐しいとも思はなかつた。但彼の掌に載せられてスーと持ち上げられた時何だかフハフハした感じが有つた許りである。掌の上で少し落ち付いて書生の顏を見たが所謂人間といふものゝ見始であらう。此の時妙なものだと思つた感じが今でも殘つて居る。第一毛を以て裝飾されべき筈の顏がつる／＼して丸で藥罐だ。其後猫にも大分逢つたがこんな片輪には一度も出會はした事がない。加之顏の眞中が餘りに突起して居る。そうして其穴の中から時々ぷう／＼と烟を吹く。どうも咽せぽくて實に弱つた。是が人間の飮む烟草といふものである事は漸く此頃知つた。",
                        size: 20.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::Start,
                        h_align: Align::Start,
                    },
                    RenderPrimitives::Text {
                        rect: (
                            0.0,
                            0.0,
                            self.width,
                            self.height,
                        ),
                        text: "左上",
                        size: 14.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::Start,
                        h_align: Align::Start,
                    },
                    RenderPrimitives::Text {
                        rect: (
                            0.0,
                            0.0,
                            self.width,
                            self.height,
                        ),
                        text: "右上",
                        size: 14.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::Start,
                        h_align: Align::End,
                    },
                    RenderPrimitives::Text {
                        rect: (
                            0.0,
                            0.0,
                            self.width,
                            self.height,
                        ),
                        text: "左下",
                        size: 14.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::End,
                        h_align: Align::Start,
                    },
                    RenderPrimitives::Text {
                        rect: (
                            0.0,
                            0.0,
                            self.width,
                            self.height,
                        ),
                        text: "右下",
                        size: 14.0,
                        color: (0xFFFFFF, 1.0),
                        v_align: Align::End,
                        h_align: Align::End,
                    },
                ];
    }
}
