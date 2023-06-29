use makepad_widgets::*;

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::image::Image;
    import crate::animated_image::*;

    App = {{App}} {
        ui: <DesktopWindow>{
            show_bg: true
            layout: {
                flow: Down,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                }
            },
            walk: {
                width: Fill,
                height: Fill
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#7, #3, self.pos.y);
                }
            }

            // Just a static image
            static_image = <Image> {
                image: dep("crate://self/resources/tinrs_mobile.png"),
                walk: {width: 90, height: 66},
                draw_bg: {
                    angle: 2., // todo: make this into a rotation angle, seprate from the state angle.

                    // you can use the filter function to modify the colors of the image at the pixel level.
                    // in this case we just modify the alpha.
                    fn filter(self) -> vec4 { return vec4(1., 1., 1., .5) }
                },
            }

            // A custom animated image using LiveState
            animated_image = <MyAnimatedImage> {}
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::animated_image::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let _actions = self.ui.handle_widget_event(cx, event);
    }
}
