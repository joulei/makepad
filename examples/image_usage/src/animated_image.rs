use makepad_widgets::*;

live_design! {
    import makepad_widgets::image::Image;

    MyAnimatedImage = {{MyAnimatedImage}} {
        image: <Image> {
            image: dep("crate://self/resources/tinrs_mobile.png"),
            walk: {width: 90, height: 66},
        }

        state: {
            fade = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {filter: vec4(1., 1., 1., 1.)} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: vec4(1., 1., 1., 1.)}}
                    apply: {
                        image: { draw_bg: {filter: vec4(1., 1., 1., .0)} }
                    }
                }
            }

            scale = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {scale: 1.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {scale: 0.0} }
                    }
                }
            }

            rotate = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {angle: 0.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {angle: 6.28318}}
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct MyAnimatedImage {
    #[state]
    state: LiveState,
}

impl LiveHook for MyAnimatedImage {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        _apply_from: ApplyFrom,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
    }
}
