use crate::{
    dnd::DndProvider,
    mime::{ClipboardLoadData, ClipboardStoreData},
    ClipboardProvider,
};

use dnd::{DndAction, DndDestinationRectangle, DndSurface, Icon};
use mime::{AllowedMimeTypes, AsMimeTypes};
use raw_window_handle::{HasDisplayHandle, RawDisplayHandle};
use std::{borrow::Cow, error::Error, sync::Arc};
#[cfg(feature = "wayland")]
use wayland::DndSender;

#[cfg(feature = "wayland")]
pub use clipboard_wayland as wayland;
#[cfg(feature = "x11")]
pub use clipboard_x11 as x11;

pub enum Clipboard {
    #[cfg(feature = "wayland")]
    Wayland(wayland::Clipboard),
    #[cfg(feature = "x11")]
    X11(x11::Clipboard),
}

impl ClipboardProvider for Clipboard {
    fn read(&self) -> Result<String, Box<dyn Error>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.read(),
            #[cfg(feature = "x11")]
            Clipboard::X11(c) => c.read().map_err(Box::from),
        }
    }

    fn write(&mut self, contents: String) -> Result<(), Box<dyn Error>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.write(contents),
            #[cfg(feature = "x11")]
            Clipboard::X11(c) => c.write(contents).map_err(Box::from),
        }
    }

    fn read_primary(&self) -> Option<Result<String, Box<dyn Error>>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => Some(c.read_primary()),
            #[cfg(feature = "x11")]
            Clipboard::X11(c) => Some(c.read_primary().map_err(Box::from)),
        }
    }

    fn write_primary(
        &mut self,
        contents: String,
    ) -> Option<Result<(), Box<dyn Error>>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => Some(c.write_primary(contents)),
            #[cfg(feature = "x11")]
            Clipboard::X11(c) => {
                Some(c.write_primary(contents).map_err(Box::from))
            }
        }
    }

    fn read_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        T: mime::AllowedMimeTypes,
    {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => {
                let ret = c.read_data::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }

    fn write_data<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        T: mime::AsMimeTypes,
    {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => {
                Some(c.write_data::<ClipboardStoreData<T>>(contents))
            }
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }

    fn read_primary_data<T: 'static>(&self) -> Option<Result<T, Box<dyn Error>>>
    where
        T: mime::AllowedMimeTypes,
    {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => {
                let ret = c.read_primary_data::<ClipboardLoadData<T>>();
                Some(ret.map(|ret| ret.0))
            }
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }

    fn read_primary_raw(
        &self,
        allowed: Vec<String>,
    ) -> Option<Result<(Vec<u8>, String), Box<dyn Error>>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => Some(c.read_primary_raw(allowed)),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }

    fn read_raw(
        &self,
        allowed: Vec<String>,
    ) -> Option<Result<(Vec<u8>, String), Box<dyn Error>>> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => Some(c.read_raw(allowed)),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }

    fn write_primary_data<T: Send + Sync + 'static>(
        &mut self,
        contents: ClipboardStoreData<T>,
    ) -> Option<Result<(), Box<dyn Error>>>
    where
        T: mime::AsMimeTypes,
    {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => {
                Some(c.write_primary_data::<ClipboardStoreData<T>>(contents))
            }
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => None,
        }
    }
}

impl DndProvider for Clipboard {
    fn init_dnd(
        &self,
        tx: Box<dyn dnd::Sender<DndSurface> + Send + Sync + 'static>,
    ) {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.init_dnd(DndSender(Arc::from(tx))),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => {}
        }
    }

    fn start_dnd<D: AsMimeTypes + Send + 'static>(
        &self,
        internal: bool,
        source_surface: DndSurface,
        icon_surface: Option<Icon>,
        content: D,
        actions: DndAction,
    ) {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.start_dnd(
                internal,
                source_surface,
                icon_surface,
                content,
                actions,
            ),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => {}
        }
    }

    fn end_dnd(&self) {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.end_dnd(),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => {}
        }
    }

    fn register_dnd_destination(
        &self,
        surface: DndSurface,
        rectangles: Vec<DndDestinationRectangle>,
    ) {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => {
                c.register_dnd_destination(surface, rectangles)
            }
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => {}
        }
    }

    fn set_action(&self, action: DndAction) {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.set_action(action),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => {}
        }
    }

    fn peek_offer<D: AllowedMimeTypes + 'static>(
        &self,
        mime_type: Option<Cow<'static, str>>,
    ) -> std::io::Result<D> {
        match self {
            #[cfg(feature = "wayland")]
            Clipboard::Wayland(c) => c.peek_offer::<D>(mime_type),
            #[cfg(feature = "x11")]
            Clipboard::X11(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "DnD not supported",
            )),
        }
    }
}

pub unsafe fn connect<W: HasDisplayHandle + ?Sized>(
    window: &W,
) -> Result<Clipboard, Box<dyn Error>> {
    match window.display_handle()?.as_raw() {
        #[cfg(feature = "wayland")]
        RawDisplayHandle::Wayland(handle) => Ok(Clipboard::Wayland(
            wayland::Clipboard::connect(handle.display.as_ptr()),
        )),
        #[cfg(feature = "x11")]
        RawDisplayHandle::Xlib(_) | RawDisplayHandle::Xcb(_) => {
            Ok(Clipboard::X11(x11::Clipboard::connect()?))
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "unsupported clipboard display backend",
        )
        .into()),
    }
}
