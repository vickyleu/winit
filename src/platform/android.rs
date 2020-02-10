#![cfg(any(target_os = "android"))]

use crate::event_loop::{EventLoop, EventLoopWindowTarget};
use crate::window::{Window, WindowBuilder};
use glutin_interface::{
    AndroidWindowParts, NativeDisplay, NativeWindow, NativeWindowSource, RawDisplay, RawWindow,
    Seal,
};
use ndk::configuration::Configuration;
use ndk_glue::Rect;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit_types::dpi::PhysicalSize;
use winit_types::error::Error;

/// Additional methods on `EventLoop` that are specific to Android.
pub trait EventLoopExtAndroid {}

impl<T> EventLoopExtAndroid for EventLoop<T> {}

/// Additional methods on `EventLoopWindowTarget` that are specific to Android.
pub trait EventLoopWindowTargetExtAndroid {}

/// Additional methods on `Window` that are specific to Android.
pub trait WindowExtAndroid {
    fn content_rect(&self) -> Rect;

    fn config(&self) -> Configuration;
}

impl WindowExtAndroid for Window {
    fn content_rect(&self) -> Rect {
        self.window.content_rect()
    }

    fn config(&self) -> Configuration {
        self.window.config()
    }
}

/// Additional methods on `WindowBuilder` that are specific to Android.
pub trait WindowBuilderExtAndroid {}

impl WindowBuilderExtAndroid for WindowBuilder {}

impl NativeWindow for Window {
    fn raw_window(&self) -> RawWindow {
        if let RawWindowHandle::Android(handle) = self.raw_window_handle() {
            RawWindow::Android {
                a_native_window: handle.a_native_window,
                _non_exhaustive_do_not_use: Seal,
            }
        } else {
            unreachable!()
        }
    }

    fn size(&self) -> PhysicalSize<u32> {
        self.inner_size()
    }

    fn scale_factor(&self) -> f64 {
        self.scale_factor()
    }
}

impl<T> NativeDisplay for EventLoopWindowTarget<T> {
    fn raw_display(&self) -> RawDisplay {
        RawDisplay::Android {
            _non_exhaustive_do_not_use: Seal,
        }
    }
}

impl<T> NativeWindowSource for EventLoopWindowTarget<T> {
    type Window = Window;
    type WindowBuilder = WindowBuilder;

    fn build_android(
        &self,
        wb: Self::WindowBuilder,
        _awp: AndroidWindowParts,
    ) -> Result<Self::Window, Error> {
        wb.build(self)
    }
}
