use crate::makepad_platform::*;

live_design! {
    link theme_desktop_dark;
    use link::shaders::*;
    
    // GLOBAL PARAMETERS
    pub THEME_COLOR_CONTRAST = 1.0
    pub THEME_COLOR_TINT = #f00
    pub THEME_COLOR_TINT_AMOUNT = 0.0
    pub THEME_SPACE_FACTOR = 6. // Increase for a less dense layout
    pub THEME_CORNER_RADIUS = 2.5
    pub THEME_BEVELING = 0.75
    pub THEME_FONT_SIZE_BASE = 7.5
    pub THEME_FONT_SIZE_CONTRAST = 2.5// Greater values = greater font-size steps between font-formats (i.e. from H3 to H2)

    // DIMENSIONS
    pub THEME_SPACE_1 = (0.5 * (THEME_SPACE_FACTOR))
    pub THEME_SPACE_2 = (1.0 * (THEME_SPACE_FACTOR))
    pub THEME_SPACE_3 = (1.5 * (THEME_SPACE_FACTOR))

    pub THEME_MSPACE_1 = {top: (THEME_SPACE_1), right: (THEME_SPACE_1), bottom: (THEME_SPACE_1), left: (THEME_SPACE_1)} 
    pub THEME_MSPACE_H_1 = {top: 0., right: (THEME_SPACE_1), bottom: 0., left: (THEME_SPACE_1)}
    pub THEME_MSPACE_V_1 = {top: (THEME_SPACE_1), right: 0., bottom: (THEME_SPACE_1), left: 0.}
    pub THEME_MSPACE_2 = {top: (THEME_SPACE_2), right: (THEME_SPACE_2), bottom: (THEME_SPACE_2), left: (THEME_SPACE_2)}
    pub THEME_MSPACE_H_2 = {top: 0., right: (THEME_SPACE_2), bottom: 0., left: (THEME_SPACE_2)}
    pub THEME_MSPACE_V_2 = {top: (THEME_SPACE_2), right: 0., bottom: (THEME_SPACE_2), left: 0.}
    pub THEME_MSPACE_3 = {top: (THEME_SPACE_3), right: (THEME_SPACE_3), bottom: (THEME_SPACE_3), left: (THEME_SPACE_3)}
    pub THEME_MSPACE_H_3 = {top: 0., right: (THEME_SPACE_3), bottom: 0., left: (THEME_SPACE_3)}
    pub THEME_MSPACE_V_3 = {top: (THEME_SPACE_3), right: 0., bottom: (THEME_SPACE_3), left: 0.}

    pub THEME_DATA_ITEM_HEIGHT = 23.0
    pub THEME_DATA_ICON_WIDTH = 16.0
    pub THEME_DATA_ICON_HEIGHT = 24.0

    pub THEME_CONTAINER_CORNER_RADIUS = (THEME_CORNER_RADIUS * 2.)
    pub THEME_TEXTSELECTION_CORNER_RADIUS = (THEME_CORNER_RADIUS * .5)
    pub THEME_TAB_HEIGHT = 32.0,
    pub THEME_SPLITTER_HORIZONTAL = 16.0,
    pub THEME_SPLITTER_SIZE = 10.0,
    pub THEME_SPLITTER_MIN_HORIZONTAL = (THEME_TAB_HEIGHT),
    pub THEME_SPLITTER_MAX_HORIZONTAL = (THEME_TAB_HEIGHT + THEME_SPLITTER_SIZE),
    pub THEME_SPLITTER_MIN_VERTICAL = (THEME_SPLITTER_HORIZONTAL),
    pub THEME_SPLITTER_MAX_VERTICAL = (THEME_SPLITTER_HORIZONTAL + THEME_SPLITTER_SIZE),
    pub THEME_SPLITTER_SIZE = 5.0
    pub THEME_DOCK_BORDER_SIZE: 0.0

    // COLOR PALETTE
    // HIGHER VALUE = HIGHER CONTRAST, RECOMMENDED VALUES: 0.5 - 2.5

    pub THEME_COLOR_W = #FFFFFFFF
    pub THEME_COLOR_W_H = #FFFFFF00
    pub THEME_COLOR_B = #000000FF
    pub THEME_COLOR_B_H = #00000000

    pub THEME_COLOR_WHITE = (mix(THEME_COLOR_W, #FFFFFF00, pow(0.1, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_5 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.35, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_4 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.6, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_3 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.75, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_2 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.9, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_1 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.95, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_U_HIDDEN = (THEME_COLOR_W_H)

    pub THEME_COLOR_D_HIDDEN = (THEME_COLOR_B_H)
    pub THEME_COLOR_D_1 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.85, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_D_2 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.75, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_D_3 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.6, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_D_4 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.4, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_BLACK = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.1, THEME_COLOR_CONTRAST)))

    // BASICS
    pub THEME_COLOR_MAKEPAD = #FF5C39FF

    pub THEME_COLOR_BG_APP = (mix(
        mix(THEME_COLOR_B, THEME_COLOR_TINT, THEME_COLOR_TINT_AMOUNT),
        mix(THEME_COLOR_W, THEME_COLOR_TINT, THEME_COLOR_TINT_AMOUNT),
        pow(0.3, THEME_COLOR_CONTRAST)))
    pub THEME_COLOR_FG_APP = (mix(
        mix(THEME_COLOR_B, THEME_COLOR_TINT, THEME_COLOR_TINT_AMOUNT),
        mix(THEME_COLOR_W, THEME_COLOR_TINT, THEME_COLOR_TINT_AMOUNT),
        pow(0.36, THEME_COLOR_CONTRAST))
    )
    pub THEME_COLOR_BG_HIGHLIGHT = (THEME_COLOR_FG_APP)
    pub THEME_COLOR_BG_UNFOCUSSED = (THEME_COLOR_BG_HIGHLIGHT * 0.85)
    pub THEME_COLOR_APP_CAPTION_BAR = (THEME_COLOR_D_HIDDEN)
    pub THEME_COLOR_DRAG_QUAD = (THEME_COLOR_U_5)

    pub THEME_COLOR_CURSOR_BG = (THEME_COLOR_BLACK)
    pub THEME_COLOR_CURSOR_BORDER = (THEME_COLOR_WHITE)

    pub THEME_COLOR_TEXT_DEFAULT = (THEME_COLOR_U_5)
    pub THEME_COLOR_TEXT_DEFAULT_DARK = (THEME_COLOR_D_4)
    pub THEME_COLOR_TEXT_HL = (THEME_COLOR_TEXT_DEFAULT)

    pub THEME_COLOR_TEXT_PRESSED = (THEME_COLOR_U_4)
    pub THEME_COLOR_TEXT_HOVER = (THEME_COLOR_WHITE)
    pub THEME_COLOR_TEXT_ACTIVE = (THEME_COLOR_U_5)
    pub THEME_COLOR_TEXT_INACTIVE = (THEME_COLOR_U_5)
    pub THEME_COLOR_TEXT_SELECTED = (THEME_COLOR_WHITE)
    pub THEME_COLOR_TEXT_FOCUSED = (THEME_COLOR_U_5)
    pub THEME_COLOR_TEXT_PLACEHOLDER = (THEME_COLOR_U_4)
    pub THEME_COLOR_TEXT_META = (THEME_COLOR_U_4)

    pub THEME_COLOR_TEXT_CURSOR = (THEME_COLOR_WHITE)

    pub THEME_COLOR_BG_CONTAINER = (THEME_COLOR_D_3 * 0.8)
    pub THEME_COLOR_BG_EVEN = (THEME_COLOR_BG_CONTAINER * 0.875)
    pub THEME_COLOR_BG_ODD = (THEME_COLOR_BG_CONTAINER * 1.125)
    pub THEME_COLOR_BG_HIGHLIGHT = (THEME_COLOR_U_1) // Code-blocks and quotes.
    pub THEME_COLOR_BG_HIGHLIGHT_INLINE = (THEME_COLOR_U_3) // i.e. inline code

    pub THEME_COLOR_BEVEL_LIGHT = (THEME_COLOR_U_3)
    pub THEME_COLOR_BEVEL_SHADOW = (THEME_COLOR_D_3)

    // WIDGET COLORS
    pub THEME_COLOR_CTRL_DEFAULT = (THEME_COLOR_U_1)
    pub THEME_COLOR_CTRL_PRESSED = (THEME_COLOR_D_1)
    pub THEME_COLOR_CTRL_HOVER = (THEME_COLOR_U_2)
    pub THEME_COLOR_CTRL_ACTIVE = (THEME_COLOR_D_2)
    pub THEME_COLOR_CTRL_SELECTED = (THEME_COLOR_U_2)
    pub THEME_COLOR_CTRL_INACTIVE = (THEME_COLOR_D_HIDDEN)

    pub THEME_COLOR_FLOATING_BG = #505050FF // Elements that live on top of the UI like dialogs, popovers, and context menus.

    // Background of textinputs, radios, checkboxes etc.
    pub THEME_COLOR_INSET_DEFAULT = (THEME_COLOR_D_1)
    pub THEME_COLOR_INSET_PIT_TOP = (THEME_COLOR_D_4)
    pub THEME_COLOR_INSET_PIT_TOP_HOVER = (THEME_COLOR_D_4)
    pub THEME_COLOR_INSET_PIT_BOTTOM = (THEME_COLOR_D_HIDDEN)

    // Progress bars, slider amounts etc.
    pub THEME_COLOR_AMOUNT_DEFAULT = (THEME_COLOR_U_3)
    pub THEME_COLOR_AMOUNT_DEFAULT_BIG = #A
    pub THEME_COLOR_AMOUNT_HOVER = (THEME_COLOR_U_4)
    pub THEME_COLOR_AMOUNT_ACTIVE = (THEME_COLOR_U_5)
    pub THEME_COLOR_AMOUNT_TRACK_DEFAULT = (THEME_COLOR_D_3)
    pub THEME_COLOR_AMOUNT_TRACK_HOVER = (THEME_COLOR_D_3)
    pub THEME_COLOR_AMOUNT_TRACK_ACTIVE = (THEME_COLOR_D_4)

    // WIDGET SPECIFIC COLORS
    pub THEME_COLOR_DIVIDER = (THEME_COLOR_D_4)

    pub THEME_COLOR_SLIDER_NUB_DEFAULT = (THEME_COLOR_WHITE)
    pub THEME_COLOR_SLIDER_NUB_HOVER = (THEME_COLOR_WHITE)
    pub THEME_COLOR_SLIDER_NUB_ACTIVE = (THEME_COLOR_WHITE)

    pub THEME_COLOR_SLIDES_CHAPTER = (THEME_COLOR_MAKEPAD)
    pub THEME_COLOR_SLIDES_BG = (THEME_COLOR_D_4)

    pub THEME_COLOR_SLIDER_BIG_NUB_TOP = #8
    pub THEME_COLOR_SLIDER_BIG_NUB_TOP_HOVER = #A
    pub THEME_COLOR_SLIDER_BIG_NUB_BOTTOM = #282828
    pub THEME_COLOR_SLIDER_BIG_NUB_BOTTOM_HOVER = #3

    pub THEME_COLOR_CTRL_SCROLLBAR_HOVER = (THEME_COLOR_U_3)

    pub THEME_COLOR_DOCK_CONTAINER = (THEME_COLOR_BG_CONTAINER)
    pub THEME_COLOR_DOCK_TAB_SELECTED = (THEME_COLOR_FG_APP)
    pub THEME_COLOR_DOCK_TAB_SELECTED_MINIMAL = (THEME_COLOR_U_4)


    // TODO: THESE ARE APPLICATION SPECIFIC COLORS THAT SHOULD BE MOVED FROM THE GENERAL THEME TO THE GIVEN PROJECT
    pub THEME_COLOR_HIGH = #C00
    pub THEME_COLOR_MID = #FA0
    pub THEME_COLOR_LOW = #8A0
    pub THEME_COLOR_PANIC = #f0f
    pub THEME_COLOR_ICON_WAIT = (THEME_COLOR_LOW),
    pub THEME_COLOR_ERROR = (THEME_COLOR_HIGH),
    pub THEME_COLOR_WARNING = (THEME_COLOR_MID),
    pub THEME_COLOR_ICON_PANIC = (THEME_COLOR_HIGH)


    // TYPOGRAPHY
    pub THEME_FONT_SIZE_CODE = 9.0
    pub THEME_FONT_LINE_SPACING = 1.43

    pub THEME_FONT_SIZE_1 = (THEME_FONT_SIZE_BASE + 16 * THEME_FONT_SIZE_CONTRAST)
    pub THEME_FONT_SIZE_2 = (THEME_FONT_SIZE_BASE + 8 * THEME_FONT_SIZE_CONTRAST)
    pub THEME_FONT_SIZE_3 = (THEME_FONT_SIZE_BASE + 4 * THEME_FONT_SIZE_CONTRAST)
    pub THEME_FONT_SIZE_4 = (THEME_FONT_SIZE_BASE + 2 * THEME_FONT_SIZE_CONTRAST)
    pub THEME_FONT_SIZE_P = (THEME_FONT_SIZE_BASE + 1 * THEME_FONT_SIZE_CONTRAST)

    pub THEME_FONT_LABEL = {
        font: { path: dep("crate://self/resources/IBMPlexSans-Text.ttf") },
        font2: { path: dep("crate://self/resources/LXGWWenKaiRegular.ttf") },
    } // TODO: LEGACY, REMOVE. REQUIRED BY RUN LIST IN STUDIO ATM
    pub THEME_FONT_REGULAR = {
        font: { path: dep("crate://self/resources/IBMPlexSans-Text.ttf") }
        font2: { path: dep("crate://self/resources/LXGWWenKaiRegular.ttf") },
    }
    pub THEME_FONT_BOLD = {
        font: { path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf") }
        font2: { path: dep("crate://self/resources/LXGWWenKaiBold.ttf") },
    }
    pub THEME_FONT_ITALIC = {
        font: { path: dep("crate://self/resources/IBMPlexSans-Italic.ttf") }
        font2: { path: dep("crate://self/resources/LXGWWenKaiRegular.ttf") },
    }
    pub THEME_FONT_BOLD_ITALIC = {
        font: { path: dep("crate://self/resources/IBMPlexSans-BoldItalic.ttf") },
        font2: { path: dep("crate://self/resources/LXGWWenKaiBold.ttf") },
    }
    pub THEME_FONT_CODE = {
        font: { path: dep("crate://self/resources/LiberationMono-Regular.ttf") }
        font_size: (THEME_FONT_SIZE_CODE)
        //brightness: 1.1
        line_scale: 1.2,
        line_spacing: 1.16
    }
}
