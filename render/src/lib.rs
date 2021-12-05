#![allow(dead_code)]

mod platform;

#[macro_use]
mod cx;
#[macro_use]
mod live_prims;

mod live_eval;
mod live_traits;
mod live_cx;

mod area;
mod events;
mod turtle;
mod font;
mod window;
mod view;
mod pass;
mod texture;
mod cursor;
mod menu;
mod animator;
mod gpu_info;
mod draw_vars;
mod geometry;

mod shader;

pub use {
    makepad_derive_live::*,
    //makepad_microserde::*,
    makepad_live_compiler::{
        live_error_origin,
        LiveErrorOrigin,
        LiveNodeOrigin,
        id,
        id_num,
        Mat4,
        Transform,
        vec2,
        vec3,
        vec4,
        Vec2,
        Vec3,
        Vec4,
        Plane,
        Quat,
        LiveRegistry,
        LiveDocNodes,
        LiveId,
        LiveFileId,
        LivePtr,
        LiveNode,
        LiveType,
        LiveTypeInfo,
        LiveTypeField,
        LiveFieldKind,
        LiveTypeKind,
        LiveValue,
        FittedString,
        InlineString,
        LiveModuleId,
        LiveNodeSlice,
        LiveNodeVec,
    },
    
    makepad_shader_compiler::{
        ShaderRegistry, 
        ShaderEnum,
        DrawShaderPtr, 
        ShaderTy,
    },
    crate::{
        cx::{
            Cx,
            PlatformType
        },
        area::{
            Area,
            ViewArea,
            InstanceArea
        },
        events::{
            KeyCode,
            Event,
            Signal,
            Timer,
            NextFrame,
            KeyModifiers,
            FingerInputType,
            FingerDownEvent,
            FingerMoveEvent,
            FingerUpEvent,
            HoverState,
            FingerHoverEvent,
            FingerScrollEvent,
            WindowGeomChangeEvent,
            WindowMovedEvent,
            AnimateEvent,
            NextFrameEvent,
            FileReadEvent,
            TimerEvent,
            SignalEvent,
            FileWriteEvent,
            KeyEvent,
            KeyFocusEvent,
            TextInputEvent,
            TextCopyEvent,
            WindowCloseRequestedEvent,
            WindowClosedEvent,
            WindowResizeLoopEvent,
            WindowDragQueryResponse,
            WindowDragQueryEvent,
            XRButton,
            XRInput,
            XRUpdateEvent,
            WebSocketMessageEvent,
            FingerDragEvent,
            FingerDropEvent,
            DragState,
            DragAction,
            DraggedItem,
            HitOpt,
        },
        cursor::MouseCursor,
        menu::Menu,
        font::Font,
        turtle::{
            LineWrap,
            Layout,
            Walk,
            Align,
            Margin,
            Padding,
            Direction,
            Axis,
            Width,
            Height,
            Rect
        },
        window::Window,
        view::{
            View,
            ManyInstances,
            ViewRedraw
        },
        pass::{
            Pass,
            PassClearColor,
            PassClearDepth
        },
        texture::{Texture,TextureFormat},
        live_traits::{
            OptionAnyAction,
            LiveFactory,
            LiveNew,
            LiveApply,
            LiveHook,
            LiveApplyValue,
            ToLiveValue,
            LiveAnimate,
            ApplyFrom,
            LiveBody,
            AnyAction,
            FrameComponent
        },
        animator::{
            Ease,
            Play,
            Animator,
            AnimatorAction
        },
        draw_vars::{
            DrawShader,
            DrawVars
        },
        geometry::{
            GeometryField,
            Geometry,
        },
        gpu_info::{
            GpuPerformance
        },
        shader::{
            draw_quad::DrawQuad,
            draw_text::DrawText,
            draw_color::DrawColor,
            geometry_gen::{
                GeometryGen,
                GeometryQuad2D,
            },
        },
    }
};

