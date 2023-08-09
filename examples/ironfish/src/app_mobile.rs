use crate::makepad_widgets::*;

live_design!{
    import makepad_widgets::theme::*;
    import makepad_widgets::frame::*;
    import makepad_draw::shader::std::*;
    
    import makepad_widgets::image::Image;
    import makepad_widgets::label::Label;
    import makepad_widgets::drop_down::DropDown;
    import makepad_widgets::button::Button;
    import makepad_widgets::slider::Slider;
    import makepad_widgets::check_box::CheckBox;
    import makepad_widgets::text_input::TextInput;
    import makepad_widgets::radio_button::RadioButton;
    import makepad_widgets::swipe_list::SwipeList;
    import makepad_widgets::swipe_list::SwipeListEntry;
    
    import makepad_example_ironfish::sequencer::Sequencer;
    import makepad_audio_widgets::display_audio::DisplayAudio;
    import makepad_audio_widgets::piano::Piano;
    
    SPACING_OS = 40.0;
    SPACING_CONTROLS = 7.5;
    SPACING_BASE_PADDING = 6.0;
    
    SSPACING_0 = 0.0
    SSPACING_1 = 4.0
    SSPACING_2 = (SSPACING_1 * 2)
    SSPACING_3 = (SSPACING_1 * 3)
    SSPACING_4 = (SSPACING_1 * 4)
    
    SPACING_0 = {top: (SSPACING_0), right: (SSPACING_0), bottom: (SSPACING_0), left: (SSPACING_0)}
    SPACING_1 = {top: (SSPACING_1), right: (SSPACING_1), bottom: (SSPACING_1), left: (SSPACING_1)}
    SPACING_2 = {top: (SSPACING_2), right: (SSPACING_2), bottom: (SSPACING_2), left: (SSPACING_2)}
    SPACING_3 = {top: (SSPACING_3), right: (SSPACING_3), bottom: (SSPACING_3), left: (SSPACING_3)}
    SPACING_4 = {top: (SSPACING_4), right: (SSPACING_4), bottom: (SSPACING_4), left: (SSPACING_4)}
    
    COLOR_CLEAR = #x3;
    
    COLOR_PLAYARROW_INNER = #xFFFDDDFF;
    
    COLOR_DOWN_OFF = #x00000000;
    COLOR_DOWN_1 = #x00000022;
    COLOR_DOWN_2 = #x00000044;
    COLOR_DOWN_3 = #x00000066;
    COLOR_DOWN_4 = #x00000088;
    COLOR_DOWN_5 = #x000000AA;
    COLOR_DOWN_FULL = #x000000FF;
    
    COLOR_UP_OFF = #xFFFFFF00;
    COLOR_UP_2 = #xFFFFFF11;
    COLOR_UP_3 = #xFFFFFF22;
    COLOR_UP_4 = #xFFFFFF33;
    COLOR_UP_5 = #xFFFFFF44;
    COLOR_UP_6 = #xFFFFFF66;
    COLOR_UP_7 = #xFFFFFF88;
    COLOR_UP_8 = #xFFFFFFCC;
    COLOR_UP_FULL = #xFFFFFFFF;
    
    GRADIENT_A = #x08221D;
    GRADIENT_B = #x3F3769;
    
    FONT_SIZE_H1 = 15.0;
    FONT_SIZE_H2 = 12.0;
    FONT_SIZE_H3 = 9.0;
    
    H2_TEXT_BOLD = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
    }
    
    H2_TEXT_REGULAR = {
        font_size: (FONT_SIZE_H2),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }
    
    H3_TEXT_REGULAR = {
        font_size: (FONT_SIZE_H3),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }
    
    ICO_ARP = dep("crate://self/resources/icons/Icon_Arp.svg")
    ICO_BROWSE = dep("crate://self/resources/icons/Icon_Browse.svg")
    ICO_DOWN = dep("crate://self/resources/icons/Icon_Down.svg")
    ICO_FAV = dep("crate://self/resources/icons/Icon_Favorite.svg")
    ICO_FILTER_BP = dep("crate://self/resources/icons/Icon_Filters_BP.svg")
    ICO_FILTER_BR = dep("crate://self/resources/icons/Icon_Filters_BR.svg")
    ICO_FILTER_HP = dep("crate://self/resources/icons/Icon_Filters_HP.svg")
    ICO_LIVEPLAY = dep("crate://self/resources/icons/Icon_LivePlaying.svg")
    ICO_NEXT = dep("crate://self/resources/icons/Icon_Next.svg")
    ICO_OSC_HARMONIC = dep("crate://self/resources/icons/Icon_OSC_Harmonic.svg")
    ICO_OSC_SAW = dep("crate://self/resources/icons/Icon_OSC_Saw.svg")
    ICO_OSC_SINE = dep("crate://self/resources/icons/Icon_OSC_Sine.svg")
    ICO_OSC_SUPERSAW = dep("crate://self/resources/icons/Icon_OSC_Supersaw.svg")
    ICO_OSC_TRI = dep("crate://self/resources/icons/Icon_OSC_Tri.svg")
    ICO_PANIC = dep("crate://self/resources/icons/Icon_Panic.svg")
    ICO_PLAT_MOBILE = dep("crate://self/resources/icons/Icon_Platform_Mobile.svg")
    ICO_PLAT_DESKTOP = dep("crate://self/resources/icons/Icon_Platform_Desktop.svg")
    ICO_PLAY = dep("crate://self/resources/icons/Icon_Play.svg")
    ICO_PRESET = dep("crate://self/resources/icons/Icon_Presets.svg")
    ICO_PREV = dep("crate://self/resources/icons/Icon_Prev.svg")
    ICO_REDO = dep("crate://self/resources/icons/Icon_Redo.svg")
    ICO_SAVE = dep("crate://self/resources/icons/Icon_Save.svg")
    ICO_SEARCH = dep("crate://self/resources/icons/Icon_Search.svg")
    ICO_SEQ_SWEEP = dep("crate://self/resources/icons/Icon_Seq_Sweep.svg")
    ICO_SEQ = dep("crate://self/resources/icons/Icon_Seq.svg")
    ICO_SETTINGS = dep("crate://self/resources/icons/Icon_Settings.svg")
    ICO_SHARE = dep("crate://self/resources/icons/Icon_Share.svg")
    ICO_UP = dep("crate://self/resources/icons/Icon_Up.svg")
    
    
    // WIDGETS
    
    DividerX = <Frame> {
        walk: {width: Fit, height: Fill, margin: {top: 2.5, right: 1.0, bottom: 2.5, left: 1.0}}
        layout: {flow: Right}
        <Box> {
            walk: {width: 4.0, height: Fill}
            draw_bg: {color: (COLOR_UP_2)}
        }
    }
    
    DividerY = <Frame> {
        walk: {width: Fill, height: Fit, margin: {top: (SPACING_BASE_PADDING), right: (SPACING_BASE_PADDING), bottom: (SPACING_BASE_PADDING), left: (SPACING_BASE_PADDING)}}
        layout: {flow: Down}
        <Box> {
            walk: {width: Fill, height: 1.0}
            draw_bg: {color: (COLOR_DOWN_5)}
        }
    }
    
    FillerX = <Frame> {
        walk: {width: Fill, height: Fit}
    }
    
    FillerY = <Box> {
        walk: {width: Fit, height: 10}
        draw_bg: {color: #f00}
    }
    
    ElementBox = <Frame> {
        walk: {width: Fill, height: Fit}
        layout: {flow: Down, padding: {left: (SPACING_CONTROLS), top: (SPACING_CONTROLS), bottom: (SPACING_CONTROLS), right: (SPACING_CONTROLS)}, spacing: (SPACING_CONTROLS)}
    }
    
    FishPanelContainer = <Frame> {
        layout: {flow: Down},
        walk: {width: Fill, height: Fit}
    }
    
    FishTab = <RadioButton> {
        walk: {height: Fill, width: Fit}
        layout: {align: {x: 0.0, y: 0.5}}
        draw_radio: {
            radio_type: Tab,
            color_inactive: (COLOR_UP_OFF),
        }
        draw_label: {
            color_selected: (COLOR_UP_FULL),
            color_unselected: (COLOR_UP_6),
            color_unselected_hover: (COLOR_UP_6),
            text_style: <H3_TEXT_REGULAR> {}
        }
    }
    
    FishDropDown = <DropDown> {
        walk: {width: Fit}
        layout: {padding: {top: (SPACING_BASE_PADDING), right: 18.0, bottom: (SPACING_BASE_PADDING), left: (SPACING_BASE_PADDING)}}
        
        draw_label: {
            text_style: <H2_TEXT_BOLD> {},
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_UP_6),
                        (COLOR_UP_6),
                        self.focus
                    ),
                    (COLOR_UP_6),
                    self.pressed
                )
            }
        }
        
        popup_menu: {
            menu_item: {
                indent_width: 10.0
                walk: {width: Fit, height: Fit}
                layout: {
                    padding: {left: 15, top: 5, bottom: 5, right: 15},
                }
            }
        }
        draw_bg: {
            fn get_bg(self, inout sdf: Sdf2d) {
                sdf.box(
                    1,
                    1,
                    self.rect_size.x - 2,
                    self.rect_size.y - 2,
                    3
                )
                sdf.stroke_keep(
                    mix((COLOR_UP_OFF), (COLOR_UP_OFF), pow(self.pos.y, .25)),
                    1.
                );
                sdf.fill((COLOR_UP_OFF));
            }
        }
    }
    
    FishButton = <Button> {
        layout: {
            align: {x: 0.5, y: 0.5},
            padding: 10
        }
        
        draw_label: {
            text_style: <H2_TEXT_BOLD> {}
            fn get_color(self) -> vec4 {
                return mix((COLOR_UP_6), (COLOR_UP_6), self.pressed)
            }
        }
        
        draw_bg: {
            // instance pressed: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill(
                    mix(
                        (COLOR_UP_OFF),
                        (COLOR_UP_3),
                        self.pressed
                    )
                );
                
                return sdf.result
            }
        }
        
    }
    
    IconLabelButton = <Button> {
        draw_icon: {
            svg_file: (ICO_REDO)
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_UP_6),
                        (COLOR_UP_7),
                        self.hover
                    ),
                    (COLOR_UP_3),
                    self.pressed
                )
            }
        }
        layout: { align: {x: 0.5, y: 0.5 }}
        icon_walk:{margin:{left:10}, width:16,height:Fit}
        text: "Click"
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill((COLOR_UP_OFF));
                
                return sdf.result
            }
        }
    }

    //     draw_icon:{
    //         // instance hover: 0.0
    //         // instance pressed: 0.0
    //         path:"M7399.39,1614.16C7357.53,1615.77 7324.04,1650.26 7324.04,1692.51C7324.04,1702.28 7316.11,1710.22 7306.33,1710.22C7296.56,1710.22 7288.62,1702.28 7288.62,1692.51C7288.62,1630.8 7337.85,1580.49 7399.14,1578.74L7389.04,1569.44C7381,1562.04 7380.49,1549.51 7387.88,1541.47C7395.28,1533.44 7407.81,1532.92 7415.85,1540.32L7461.76,1582.58C7465.88,1586.37 7468.2,1591.73 7468.15,1597.32C7468.1,1602.91 7465.68,1608.23 7461.5,1611.94L7415.59,1652.71C7407.42,1659.97 7394.9,1659.23 7387.65,1651.06C7380.39,1642.89 7381.14,1630.37 7389.3,1623.12L7399.39,1614.16Z",
    //         // fn get_color(self) -> vec4 {
    //         //     return mix( #ff0, #ff0, self.pressed)
    //         // }
    //         // fn get_color(self) -> vec4 {
    //         //     return mix(#f00, #00f, self.pressed)
    //         // }
    //     }
    // }
    
    IconButton = <Button> {
        draw_icon: {
            svg_file: (ICO_REDO)
            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        (COLOR_UP_6),
                        (COLOR_UP_7),
                        self.hover
                    ),
                    (COLOR_UP_3),
                    self.pressed
                )
            }
        }
        icon_walk: {margin: {left: 0.0}, width: 16, height: Fit}
        label: ""
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill((COLOR_UP_OFF));
                
                return sdf.result
            }
        }
    }
    
    
    FishSlider = <Slider> {
        walk: {
            height: 36,
        }
        label: "Change Me"
        label_text: {text_style: <H2_TEXT_BOLD> {}, color: (COLOR_UP_6)}
        text_input: {
            cursor_margin_bottom: 3.0,
            cursor_margin_top: 4.0,
            select_pad_edges: 3.0
            cursor_size: 2.0,
            empty_message: "0",
            numeric_only: true,
            draw_bg: {
                shape: None
                color: (COLOR_UP_OFF);
                radius: 2.0
            },
        }
        draw_slider: {
            instance line_color: (COLOR_UP_4)
            instance bipolar: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                let top = 25.0;
                
                let in_side = 5.0;
                let x_offset = 5.0
                
                sdf.move_to(mix(in_side + x_offset, self.rect_size.x, self.bipolar), top);
                
                sdf.box(in_side, top - 5, self.rect_size.x - 2 - 2 * in_side, 10, 2) // ridge
                sdf.fill(mix((COLOR_DOWN_1), (COLOR_DOWN_2), self.drag)); // ridge color
                
                let fill_x = self.slide_pos * (self.rect_size.x - in_side * 2 - 9) + x_offset;
                sdf.line_to(fill_x + in_side, top);
                
                sdf.stroke_keep(mix((COLOR_UP_OFF), self.line_color, self.drag), 3.0);
                sdf.stroke(mix(self.line_color, (COLOR_UP_4), self.drag), 2.0)
                
                return sdf.result
            }
        }
    }
    
    TextSlider = <ElementBox> {
        slider = <FishSlider> {
            draw_slider: {
                instance line_color: (COLOR_UP_4)
                instance bipolar: 0.0
                fn pixel(self) -> vec4 {
                    let nub_size = 3
                    
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    let top = 25.0;
                    
                    let in_side = 5.0;
                    let x_offset = 5.0
                    
                    sdf.move_to(mix(in_side + x_offset, self.rect_size.x, self.bipolar), top);
                    
                    return sdf.result
                }
            }
        }
    }
    
    InstrumentSlider = <ElementBox> {
        slider = <FishSlider> {
            draw_slider: {bipolar: 0.0}
        }
    }
    
    InstrumentBipolarSlider = <ElementBox> {
        slider = <FishSlider> {
            draw_slider: {bipolar: 1.0}
        }
    }
    
    InstrumentCheckbox = <ElementBox> {
        checkbox = <CheckBox> {
            layout: {padding: {top: (SPACING_CONTROLS), right: (SPACING_CONTROLS), bottom: (SPACING_CONTROLS), left: (SPACING_CONTROLS)}}
            label: "CutOff1"
            draw_check: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    let left = 3;
                    let sz = 7.0;
                    let c = vec2(left + sz, self.rect_size.y * 0.5);
                    sdf.box(left, c.y - sz, sz * 2.0, sz * 2.2, 1.5); // rounding = 3rd value
                    sdf.fill_keep(#f00)
                    sdf.stroke(mix((COLOR_DOWN_3), (COLOR_UP_FULL), pow(self.pos.y, 3.0)), 1.0) // outline
                    
                    let szs = sz * 0.5;
                    let dx = 1.0;
                    sdf.move_to(left + 4.0, c.y);
                    sdf.line_to(c.x, c.y + szs);
                    sdf.line_to(c.x + szs, c.y - szs);
                    sdf.stroke(mix((COLOR_UP_OFF), (COLOR_UP_6), self.selected), 1.25); // CHECKMARK
                    return sdf.result
                }
            }
            draw_label: {
                text_style: <H2_TEXT_BOLD> {},
                color: (COLOR_UP_6)
            }
        }
    }
    
    InstrumentDropdown = <ElementBox> {
        layout: {align: {y: 0.5}, padding: 0, flow: Right}
        label = <Label> {
            walk: {width: Fit}
            draw_label: {
                color: (COLOR_UP_6)
                text_style: <H2_TEXT_BOLD> {},
            }
        }
        dropdown = <FishDropDown> {
            walk: {margin: {left: (SPACING_CONTROLS), right: (SPACING_CONTROLS)}}
        }
    }
    
    PlayPause = <InstrumentCheckbox> {
        walk: {width: Fit, height: Fit, margin: {top: 10, right: 0, bottom: 0, left: 0}}
        layout: {align: {x: 0.0, y: 0.5}}
        draw_bg: {color: (COLOR_UP_OFF)}
        checkbox = {
            walk: {width: 20, height: 20}
            label: ""
            draw_check: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    let left = 3;
                    let sz = 15.0;
                    let c = vec2(left + sz, sz);
                    // let c = vec2(left + sz, self.rect_size.y);
                    
                    sdf.move_to(0.0, 0.0);
                    sdf.line_to(c.x * 0.75, c.y * 0.5);
                    sdf.line_to(0.0, c.y);
                    sdf.close_path();
                    sdf.fill_keep(mix((COLOR_UP_5), (COLOR_UP_FULL), self.selected))
                    sdf.stroke_keep(
                        mix(
                            mix((COLOR_UP_6), (COLOR_UP_2), pow(self.pos.y, 0.5)),
                            (COLOR_UP_7),
                            self.selected
                        ),
                        1.
                    )
                    return sdf.result
                }
            }
        }
    }
    
    FishToggle = <ElementBox> {
        layout: {padding: <SPACING_0> {}}
        checkbox = <CheckBox> {
            layout: {padding: {top: (SSPACING_0), right: (SSPACING_2), bottom: (SSPACING_0), left: 23}}
            label: "CutOff1"
            label_walk: {margin: {left: 45.0, top: 8, bottom: 8, right: 10}}
            state: {
                selected = {
                    default: off
                    off = {
                        from: {all: Forward {duration: 0.1}}
                        apply: {draw_check: {selected: 0.0}}
                    }
                    on = {
                        cursor: Arrow,
                        from: {all: Forward {duration: 0.1}}
                        apply: {draw_check: {selected: 1.0}}
                    }
                }
            }
            draw_check: {
                instance border_width: 1.0
                instance border_color: #x06
                instance border_color2: #xFFFFFF0A
                size: 8.5;
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                    let sz = self.size;
                    let left = sz + 1.;
                    let c = vec2(left + sz, self.rect_size.y * 0.5);
                    sdf.box(left, c.y - sz, sz * 3.0, sz * 2.0, 0.5 * sz);
                    
                    sdf.stroke_keep((COLOR_UP_3), 1.25)
                    
                    sdf.fill((COLOR_UP_OFF))
                    let isz = sz * 0.65;
                    sdf.circle(left + sz + self.selected * sz, c.y - 0.5, isz);
                    sdf.circle(left + sz + self.selected * sz, c.y - 0.5, 0.425 * isz);
                    sdf.subtract();
                    sdf.circle(left + sz + self.selected * sz, c.y - 0.5, isz);
                    sdf.blend(self.selected)
                    sdf.fill(#xFFF8);
                    return sdf.result
                }
            }
            draw_label: {
                text_style: <H2_TEXT_BOLD> {},
                color: (COLOR_UP_5)
            }
        }
    }
    
    PresetFavorite = <CheckBox> {
        draw_check: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                
                let left = 1;
                let sz = self.rect_size.x / 2;
                let c = vec2(sz, sz);
                
                let csz = 4.0;
                sdf.circle(csz, csz, csz);
                sdf.circle(csz * 3, csz, csz);
                sdf.union();
                
                let squeeze = sz * 0.025;
                let top_offset = 0.6;
                // let top_offset = 0.6;
                sdf.move_to(c.x - sz + squeeze + 1.0, c.y - (sz * top_offset));
                sdf.line_to(c.x - sz * 0.5, c.y - (sz * 0.8));
                sdf.line_to(c.x - squeeze, c.y - (sz * top_offset));
                sdf.line_to(c.x * 0.5, c.y);
                sdf.close_path();
                
                sdf.fill_keep(mix(#141414, #888, self.selected))
                
                return sdf.result
            }
        }
        draw_label: {
            text_style: <H2_TEXT_BOLD> {},
            color: (COLOR_UP_6)
        }
    }
    
    
    SequencerControls = <Frame> {
        layout: {flow: Down, padding: 0, spacing: 0.0, align: {x: 0.0, y: 0.5}}
        walk: {width: Fill, height: Fit, margin: {top: 0.0, right: (SPACING_OS), bottom: 0.0, left: (SPACING_OS)}}
        
        <Frame> {
            walk: {width: Fill, height: Fit, margin: 0.0}
            layout: {align: {x: 0.0, y: 0.5}, spacing: (SPACING_CONTROLS), padding: 0.0}

            playpause = <PlayPause> {}
            
            speed = <TextSlider> {
                walk: {width: 110, height: 35, margin: 0.0}
                draw_bg: {color: (COLOR_UP_OFF)}
                slider = {
                    min: 0.0
                    max: 240.0
                    label: "BPM"
                }
            }
            
            <FillerX> {}

            <Image> {
                source: dep("crate://self/resources/tinrs_mobile.png"),
                walk: {width: (178 * 0.175), height: (121 * 0.175), margin: { top: 0.0, right: 0.0, bottom: 0.0, left: 10.0 } }
                layout: {padding: 0}
            }
            
            share = <IconButton> {
                draw_icon: {
                    svg_file: (ICO_SHARE),
                }
                icon_walk: { height: 14.0, width: Fit }
                walk: {margin: {top: 5.0, right: -7.5, bottom: 5.0, left: 0.0 }}
                layout: {padding: 10.0}
            }
            platformtoggle = <IconButton> {
                draw_icon: { svg_file: (ICO_PLAT_DESKTOP) }
                walk: {margin: { top: 0.0, right: -5, bottom: 0, left: 0 }}
                layout: {padding: 10.0}
            }
            
        }
        
        <Frame> {
            walk: {width: Fill, height: Fit, margin: 0.0}
            layout: {align: {x: 0.0, y: 0.5}, spacing: 0.0, padding: 0.0}
            
            arp = <FishToggle> {
                walk: {margin: 0.0}
                layout: {padding: <SPACING_0> {}}
                checkbox = {
                    label: "Arp"
                    layout: {padding: {top: (SSPACING_0), right: (SSPACING_1), bottom: (SSPACING_0), left: (SSPACING_0)}}
                    walk: {margin: <SPACING_0> {}}
                }
                walk: {width: Fit, height: Fit, margin: <SPACING_0> {}}
            }
            
            rootnote = <InstrumentDropdown> {
                walk: {height: Fit, width: Fit, margin: 0}
                layout: {align: {x: 0.0, y: 0.5}, padding: 0}
                draw_bg: {color: (COLOR_UP_OFF)}
                dropdown = {
                    walk: {margin: 5}
                    labels: ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"]
                    values: [A, Asharp, B, C, Csharp, D, Dsharp, E, F, Fsharp, G, Gsharp]
                }
            }
            
            scaletype = <InstrumentDropdown> {
                walk: {height: Fit, width: 90.0}
                layout: {align: {x: 0.0, y: 0.5}}
                draw_bg: {color: (COLOR_UP_OFF)}
                dropdown = {
                    walk: {margin: 5}
                    labels: ["Minor", "Major", "Dorian", "Pentatonic"]
                    values: [Minor, Major, Dorian, Pentatonic]
                }
            }
            
            <FillerX> {}
            
            grid_up = <IconButton> { draw_icon: { svg_file: (ICO_UP) }, icon_walk: {height: 7.5, width: Fit}, walk: { margin: 0.0 }, layout: {padding: 10.0} }
            grid_down = <IconButton> { draw_icon: { svg_file: (ICO_DOWN) }, icon_walk: {height: 7.5, width: Fit}, walk: { margin: 0.0 }, layout: {padding: 10.0} }
            clear_grid = <IconButton> { draw_icon: { svg_file: (ICO_SEQ_SWEEP) }, walk: { margin: { top: 0.0, right: -5, bottom: 0.0, left: 0.0 } }, layout: {padding: 10.0 } }
            
        }
        
    }
    
    SequencerPanel = <Frame> {
        walk: {width: Fill, height: Fill, margin: 0.0}
        layout: {flow: Down, spacing: 0.0}
        
        sequencer = <Sequencer> {
            walk: {width: Fill, height: Fill, margin: {top: 0, right: 0, bottom: 10, left: 0}}
            //layout: {padding: 0.0}
            
            button: {
                draw_button: {
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(1, 1, self.rect_size.x - 5, self.rect_size.y - 5, 2);
                        sdf.glow_keep(
                            mix(
                                (COLOR_UP_OFF),
                                (COLOR_UP_5),
                                self.active
                            ),
                            1.5
                        );
                        // sdf.glow_keep(
                        //     mix(
                        //         (COLOR_UP_OFF),
                        //         (COLOR_UP_3),
                        //         self.active
                        //     ),
                        //     1.5
                        // );
                        // sdf.glow_keep(
                        //     mix(
                        //         (COLOR_UP_OFF),
                        //         (COLOR_UP_2),
                        //         self.active
                        //     ),
                        //     2.0
                        // );
                        sdf.fill(
                            mix(
                                (COLOR_UP_2),
                                (COLOR_UP_5),
                                self.active
                            )
                        )
                        return sdf.result
                    }
                    
                }
            }
        }
    }
    
    ModeSequencer = <Frame> {
        walk: {width: Fill, height: Fill, margin: {top: (SPACING_OS / 2), right: (SPACING_OS), bottom: (SPACING_OS / 2), left: (SPACING_OS)}}
        layout: {flow: Down}
        
        <SequencerPanel> {walk: {height: Fill, width: Fill}}
        
        PresetNavigation = <Frame> {
            walk: {width: Fill, height: Fit}
            layout: {flow: Right, align: {x: 0.0, y: 0.5}}
            
            <IconButton> {
                draw_icon: {svg_file: (ICO_PREV)}
                icon_walk: {width: 10, height: Fit}
            }
            
            <Frame> {}
            
            <Frame> {
                walk: {width: Fit, height: Fit}
                layout: {flow: Right, align: {x: 0.0, y: 0.5}}
                
                label = <Label> {
                    draw_label: {
                        text_style: {font_size: (FONT_SIZE_H1)},
                        color: (COLOR_UP_6)
                    }
                    label: "Preset Name"
                }
            }
            
            <Frame> {}
            
            <IconButton> {
                draw_icon: {svg_file: (ICO_NEXT)}
                icon_walk: {width: 10, height: Fit}
            }
        }
    }
    
    ChordButtonA = <Button> {
        walk: {width: Fill, height: Fill, margin: 2.5}
        layout: {align: {x: 0.5, y: 0.5}, padding: 0.0}
        
        draw_label: {
            text_style: <H2_TEXT_BOLD> {}
            fn get_color(self) -> vec4 {
                return mix((COLOR_UP_6), (COLOR_UP_6), self.pressed)
            }
        }
        
        draw_bg: {
            instance pressed: 0.0
            instance color_default: 0.0
            instance COLOR_UP_3: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill(
                    mix(
                        self.color_default,
                        self.COLOR_UP_3,
                        self.pressed
                    )
                );
                
                return sdf.result
            }
            
            color_default: (COLOR_UP_2),
            COLOR_UP_3: (COLOR_UP_3);
        }
        
    }
    
    ChordButtonB = <Button> {
        walk: {width: Fill, height: Fill, margin: {top: 2.5, right: 0.0, bottom: 2.5, left: 0}}
        layout: {align: {x: 0.5, y: 0.5}, padding: 0.0}
        
        draw_label: {
            text_style: <H2_TEXT_BOLD> {}
            fn get_color(self) -> vec4 {
                return mix((COLOR_UP_6), (COLOR_UP_6), self.pressed)
            }
        }
        
        draw_bg: {
            instance pressed: 0.0
            instance color_default: 0.0
            instance COLOR_UP_3: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill(
                    mix(
                        self.color_default,
                        (COLOR_UP_2),
                        self.pressed
                    )
                );
                
                return sdf.result
            }
            
            color_default: (COLOR_UP_OFF),
            COLOR_UP_3: (COLOR_UP_3);
        }
        
    }
    
    ChordStrip = <Box> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Right, spacing: 0.0, padding: {top: 0.0, left: 0.0, bottom: 0.0, right: 5}, align: {x: 0.5, y: 0.5}}
        draw_bg: {
            color: (COLOR_UP_2),
            radius: 2.5
        }
        
        
        label = <Label> {
            walk: {width: 40, height: Fill, margin: 5.0}
            draw_label: {
                text_style: <H3_TEXT_REGULAR> {},
                color: (COLOR_UP_6)
            }
            label: "change me"
        }
        
        <ChordButtonA> {}
        <ChordButtonA> {}
        <ChordButtonA> {}
        <ChordButtonA> {walk: {margin: {top: 2.5, right: 5, bottom: 2.5, left: 2.5}}}
        <ChordButtonB> {}
        <DividerX> {}
        <ChordButtonB> {}
        <DividerX> {}
        <ChordButtonB> {}
        <DividerX> {}
        <ChordButtonB> {}
        
    }
    
    ChordPiano = <Frame> {
        layout: {flow: Down, spacing: (SPACING_CONTROLS), padding: {left: (SPACING_OS), right: (SPACING_OS)}}
        
        <ChordStrip> {label = {label: "Em"}}
        <ChordStrip> {label = {label: "Am"}}
        <ChordStrip> {label = {label: "Dm"}}
        <ChordStrip> {label = {label: "G"}}
        <ChordStrip> {label = {label: "C"}}
        <ChordStrip> {label = {label: "F"}}
        <ChordStrip> {label = {label: "Bb"}}
        <ChordStrip> {label = {label: "Bdim"}}
    }
    
    ModePlay = <Frame> {
        layout: {flow: Down, spacing: (SPACING_CONTROLS)}
        walk: {width: Fill, height: Fill, margin: {top: 20}}
        
        <ChordPiano> {}
        
        <Frame> {
            layout: {flow: Right, padding: {top: (SPACING_CONTROLS), right: (SPACING_OS), bottom: (SPACING_OS), left: (SPACING_OS)}, spacing: 10}
            walk: {width: Fill, height: Fit}
            
            <Box> {
                layout: {flow: Down}
                walk: {width: Fill, height: 200}
                draw_bg: {
                    color: (COLOR_UP_2),
                    radius: 5.0
                }
                
            }
            
            <Frame> {
                layout: {flow: Down}
                walk: {width: Fill, height: Fill}
                draw_bg: {color: #f00}
                
                crushamount = <FishSlider> {
                    label: "Crush"
                    slider = {
                        min: 0.0
                        max: 1.0
                        label: "Amount"
                        
                    }
                }
                
                <FillerY> {}
                
                chorusmix = <FishSlider> {
                    label: "Chorus"
                    slider = {
                        min: 0.0
                        max: 1.0
                        label: "Mix"
                    }
                }
                
                <FillerY> {}
                
                delaysend = <FishSlider> {
                    label: "Delay",
                    slider = {
                        min: 0.0
                        max: 1.0
                        label: "Delay Send"
                    }
                }
                
                <FillerY> {}
                
                porta = <FishSlider> {
                    label: "Portamento",
                    slider = {
                        min: 0.0
                        max: 1.0
                        label: "Portamento"
                    }
                }
                
            }
        }
        
    }
    
    FishCheckbox = <CheckBox> {
        draw_check: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                
                let left = 1;
                let sz = 8.0;
                
                let c = vec2(left + sz, self.rect_size.y * 0.5 - 2.0);
                
                sdf.box(left, c.y - sz, sz * 2.5, sz * 2.5, 2.0); // 3rd parameter == corner radius
                sdf.fill_keep(mix((COLOR_UP_2), (COLOR_UP_7), self.selected))
                sdf.stroke(#x888, 0.0) // outline
                
                let szs = sz * 0.5;
                let dx = 1.0;
                
                let offset = 1.5;
                
                sdf.move_to(left + 4.0 + offset, c.y + offset);
                sdf.line_to(c.x + offset, c.y + szs + offset);
                sdf.line_to(c.x + szs + offset, c.y - szs + offset);
                
                sdf.stroke_keep(mix(#fff0, (COLOR_DOWN_FULL), self.selected), 1.75);
                
                
                return sdf.result
            }
        }
        draw_label: {
            text_style: <H2_TEXT_BOLD> {},
            color: (COLOR_UP_6)
        }
    }
    
    PresetListEntry = <SwipeListEntry> {
        layout: {flow: Down, padding: {top: 0, right: 5, bottom: 2.5, left: 5}, align: {x: 0.5, y: 0.5}}
        walk: { width: Fill, height: Fit, margin: 0 }
        
        center: <Frame> {
            layout: {flow: Down, align: {x: 0.5, y: 0.0}, padding: {top: 0, right: 0, bottom: 0, left: 0}}
            walk: {width: Fill, height: Fit, margin: 0.0}
            <Frame> {
                layout: {flow: Right, align: {x: 0.5, y: 0.0}, padding: 0.0}
                walk: { width: Fill, height: Fit, margin: {top: 2.5, right: 0, bottom: 7.5, left: 0}}
                
                label = <Button> {
                    walk: { width: Fill, height: Fill, margin: {top: 5} }
                    layout: {align: {x: 0.0, y: 0.5}, padding: { left: 5 }}
                    draw_label: {
                        fn get_color(self) -> vec4 {
                            return mix( (COLOR_UP_5), (COLOR_UP_3), self.pressed )
                        }
                        text_style: <H2_TEXT_REGULAR>{},
                    }
                    label: "Preset Name"
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            return sdf.result
                        }
                    }
                }
                
                <Box> {
                    walk: {width: Fit, height: Fit}
                    
                    layout: {align: {x: 0.0, y: 0.5}}
                    // presetfavorite = <PresetFavorite> {
                    //     walk: {width: 30, height: 30, margin: {top: 15} }
                    //     label: " "
                    // }
                    
                <IconButton> {
                    draw_icon: {
                        svg_file: (ICO_FAV),
                        fn get_color(self) -> vec4 {
                            return mix(
                                mix(
                                    (COLOR_UP_4),
                                    (COLOR_UP_6),
                                    self.hover
                                ),
                                (COLOR_UP_3),
                                self.pressed
                            )
                        }
                    },
                    icon_walk: {height: 11.0, width: Fit}
            }
                    
                share = <IconButton> {
                    draw_icon: {
                        svg_file: (ICO_SHARE),
                        fn get_color(self) -> vec4 {
                            return mix(
                                mix(
                                    (COLOR_UP_4),
                                    (COLOR_UP_6),
                                    self.hover
                                ),
                                (COLOR_UP_3),
                                self.pressed
                            )
                        }
                    },
                    icon_walk: {height: 12.5, width: Fit}
                }
                    
                }
            }
            <DividerY> {walk: { margin: 0.0}, layout: {padding: 0.0}}
        }
    }
    
    PaginationButton = <FishButton> {
        label: "1"
        walk: {width: 40, height: 40, margin: {top: 20}}
        draw_label: {text_style: {font_size: (FONT_SIZE_H2)}}
        
        draw_bg: {
            // instance pressed: 0.0
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    2.0
                )
                
                sdf.fill(mix((COLOR_UP_2), (COLOR_UP_3), self.pressed));
                
                return sdf.result;
            }
        }
    }
    
    Pagination = <Frame> {
        walk: {width: Fill, height: Fit, margin: {bottom: (SPACING_OS)}}
        layout: {flow: Right, align: {x: 0.5, y: 0.0}, spacing: 10}
        
        <PaginationButton> {label: "…"}
        <PaginationButton> {label: "3"}
        <PaginationButton> {label: "4"}
        <PaginationButton> {label: "5"}
        <PaginationButton> {label: "6"}
        <PaginationButton> {label: "7"}
        <PaginationButton> {label: "…"}
    }
    
    PresetList = <SwipeList> {
        walk: {height: Fill}
        Entry = <PresetListEntry> {}
    }
    
    PresetListFavs = <ScrollY> {
        
        walk: {width: Fill, height: Fill, margin: {top: 5.0, bottom: 5.0}}
        layout: {flow: Down, align: {x: 0.5, y: 0.0},}
    }
    
    ModePresetmanager = <Box> {
        walk: {width: Fill, height: Fill}
        layout: {flow: Down}
        
        <GradientY> {
            draw_bg: {color: (COLOR_DOWN_OFF), color2: (COLOR_DOWN_2)}
            walk: {width: Fill, height: 50}
            layout: {flow: Right}
            
            <Frame> {
                walk: {width: Fill, margin: {top: 0, right: (SPACING_OS), bottom: 0, left: (SPACING_OS)}}
                layout: {align: {x: 0.0, y: 1.0}}
                
                <TextInput> {
                    walk: {width: Fill, height: 50}
                    layout: {align: {x: 0.0, y: 0.5}}
                    text: "Search"
                    draw_label: {
                        text_style: <H2_TEXT_BOLD> {font_size: (FONT_SIZE_H1)}
                    }
                }
                filter_modes = <Frame> {
                    walk: {width: Fit}
                    layout: {spacing: 10, align: {x: 0.0, y: 1.0}}
                    
                    tab1 = <FishTab> {
                        label: "All",
                        walk: {height: Fit, margin: 10}
                        state: {selected = {default: on}},
                        draw_label: {color_selected: (COLOR_UP_8)}
                    }
                    tab2 = <FishTab> {
                        label: "Favorites",
                        walk: {height: Fit, margin: 10}
                        draw_label: {color_selected: (COLOR_UP_8)}
                    }
                }
            }
            
        }
        preset_pages = <Frame> {
            preset_list = <PresetList> {}
            tab2_frame = <PresetListFavs> {visible: false}
        }
        
        // <Pagination> {}
    }
    
    AppMobile = <Frame> {
        design_mode: false,
        walk: {width: Fill, height: Fill}
        layout: {padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down}
        
        <GradientY> {
            draw_bg: {color: (GRADIENT_A), color2: (GRADIENT_B)}
            layout: {flow: Down, spacing: (SPACING_BASE_PADDING)}
            
            os_header_placeholder = <Box> {
                walk: {width: Fill, height: 50, margin: 0}
                layout: {flow: Right, spacing: (SPACING_BASE_PADDING), padding: 0}
                draw_bg: {color: (COLOR_DOWN_1)}
            }
            
            <SequencerControls> {}
            
            application_pages = <Frame> {
                walk: {margin: 0.0}
                layout: {padding: 0.0}
                
                tab1_frame = <ModeSequencer> {}
                tab2_frame = <ModePlay> {visible: false} // TODO: enable again
                tab3_frame = <ModePresetmanager> {visible: false} // TODO: enable again
            }
            
            mobile_menu = <Box> {
                walk: {width: Fill, height: 120}
                layout: {flow: Right, spacing: (SPACING_BASE_PADDING), padding: 20}
                draw_bg: {color: (COLOR_DOWN_2)}
                
                mobile_modes = <Frame> {
                    tab1 = <FishTab> {
                        state: {selected = {default: on}},
                        label: "Sequencer"
                        draw_icon: {
                            svg_file: (ICO_SEQ),
                            fn get_color(self) -> vec4 {
                                return mix(
                                    (COLOR_UP_6),
                                    (COLOR_UP_FULL),
                                    self.selected
                                )
                            }
                        }
                        walk: {width: Fill}
                        icon_walk: {width: 27.5, height: Fit}
                        layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                    }
                    tab2 = <FishTab> {
                        label: "Play",
                        draw_icon: {
                            svg_file: (ICO_LIVEPLAY),
                            fn get_color(self) -> vec4 {
                                return mix(
                                    (COLOR_UP_6),
                                    (COLOR_UP_FULL),
                                    self.selected
                                )
                            }
                        }
                        walk: {width: Fill}
                        icon_walk: {width: 34, height: Fit, margin: {top: 0.0, right: 0.0, bottom: 5.0, left: 0.0}}
                        layout: {flow: Down, spacing: 3.0, align: {x: 0.5, y: 0.5}}
                    }
                    tab3 = <FishTab> {
                        label: "Presets",
                        draw_icon: {
                            svg_file: (ICO_PRESET),
                            fn get_color(self) -> vec4 {
                                return mix(
                                    (COLOR_UP_6),
                                    (COLOR_UP_FULL),
                                    self.selected
                                )
                            }
                        }
                        walk: {width: Fill}
                        icon_walk: {width: 30, height: Fit}
                        layout: {flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}}
                    }
                }
            }
        }
    }
}