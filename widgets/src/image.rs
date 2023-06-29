use crate::{frame::*, register_widget};
use crate::{makepad_derive_widget::*, makepad_draw::*, widget::*};

live_design! {
    import makepad_draw::shader::std::*;

    Image = {{Image}} {
            show_bg: true,
            draw_bg: {
                shape: Solid,
                fill: Image
                texture image: texture2d
                instance scale: vec2(1., 1.)
                instance pan: vec2(0., 0.)
                instance angle: 0.
                instance filter: vec4(1., 1., 1., 1.)

                fn filter(self) {
                    return vec4(1., 1., 1., 1.);
                }

                fn rotation_padding(w: float, h: float) -> float {
                    let d = max(w, h);
                    return ((sqrt(d * d * 2.) / d) - 1.) / 2.;
                }

                fn rotate_2d_from_center(v: vec2, a: float) -> vec2 {
                    let ca = cos(-a);
                    let sa = sin(-a);
                    let p = v - vec2(.5, .5);
                    return vec2(p.x * ca - p.y * sa, p.x * sa + p.y * ca) + vec2(.5, .5);
                }

                fn get_color(self, rot_padding: float) -> vec4 {
                    // Current position is a traslated one, so let's get the original position
                    let current_pos = self.pos.xy - vec2(rot_padding, rot_padding);
                    let original_pos = rotate_2d_from_center(current_pos, self.angle);

                    // Scale the current position by the scale factor
                    let scaled_pos = (original_pos - vec2(.5, .5)) / self.scale + vec2(.5, .5);

                    // Take pixel color from the original image
                    let color = sample2d(self.image, scaled_pos).xyzw;

                    return color * self.filter();
                }

                fn pixel(self) -> vec4 {
                    let rot_padding = rotation_padding(self.rect_size.x, self.rect_size.y);

                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    let translation_offset = self.rect_size * rot_padding;
                    sdf.translate(translation_offset.x, translation_offset.y);

                    let center = self.rect_size * .5;
                    sdf.rotate(self.angle, center.x, center.y);

                    let scaled_size = self.rect_size * self.scale;
                    let offset = (self.rect_size - scaled_size) * .5;
                    sdf.box(offset.x, offset.y, scaled_size.x, scaled_size.y, 1);

                    sdf.fill(self.get_color(rot_padding));
                    return sdf.result
                }

                fn vertex(self) -> vec4 {
                    let rot_padding = rotation_padding(self.rect_size.x, self.rect_size.y);

                    // I don't know if different draw_clip values are properly supported
                    let clipped: vec2 = clamp(
                        self.geom_pos * self.rect_size * (1. + rot_padding * 2) + self.rect_pos,
                        self.draw_clip.xy,
                        self.draw_clip.zw * (1. + rot_padding * 2)
                    );

                    self.pos = (clipped - self.rect_pos) / self.rect_size;
                    return self.camera_projection * (self.camera_view * (
                        self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
                    ));
                }
            }
        }
}

#[derive(Live)]
pub struct Image {
    #[live]
    walk: Walk,
    #[deref]
    frame: Frame,
}

impl LiveHook for Image {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Image)
    }
}

impl Widget for Image {
    fn redraw(&mut self, cx: &mut Cx) {
        self.frame.redraw(cx);
    }

    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct ImageRef(WidgetRef);
