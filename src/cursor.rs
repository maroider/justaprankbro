use std::{io, ops::Drop, path::Path, ptr};

use winapi::{
    shared::{ntdef::HANDLE, windef::HICON, winerror::ERROR_FILE_NOT_FOUND},
    um::{
        errhandlingapi::GetLastError,
        winuser::{
            CopyImage, LoadImageW, SetSystemCursor, IMAGE_CURSOR, LR_LOADFROMFILE, LR_SHARED,
            MAKEINTRESOURCEW,
        },
    },
};

use missing_from_winapi::{
    OCR_APPSTARTING, OCR_CROSS, OCR_HAND, OCR_IBEAM, OCR_NO, OCR_NORMAL, OCR_SIZEALL, OCR_SIZENESW,
    OCR_SIZENS, OCR_SIZENWSE, OCR_SIZEWE, OCR_UP, OCR_WAIT,
};

#[derive(Debug)]
pub struct ReplacedCursor {
    cursor: Cursor,
    kind: CursorKind,
}

impl ReplacedCursor {
    pub fn revert(&self) {
        unsafe { SetSystemCursor(self.cursor.handle as HICON, self.kind.as_id()) };
    }
}

impl Drop for ReplacedCursor {
    fn drop(&mut self) {
        self.revert();
    }
}

#[derive(Debug, PartialEq)]
pub struct Cursor {
    handle: HANDLE,
}

impl Cursor {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.to_string_lossy();

        let utf16_path: Vec<u16> = path.encode_utf16().chain(0..=0).collect();
        let handle = unsafe {
            LoadImageW(
                ptr::null_mut(),
                utf16_path.as_ptr(),
                IMAGE_CURSOR,
                0,
                0,
                LR_SHARED | LR_LOADFROMFILE,
            )
        };

        if handle.is_null() {
            let error_code = unsafe { GetLastError() };

            let err = io::Error::from_raw_os_error(error_code as i32);
            if error_code == ERROR_FILE_NOT_FOUND {
                Err(err)
            } else {
                panic!("Unexpected error while loading cursor from file: {}", err)
            }
        } else {
            Ok(Self { handle })
        }
    }

    pub fn load_system(kind: CursorKind) -> Self {
        let cursor = unsafe {
            LoadImageW(
                ptr::null_mut(),
                MAKEINTRESOURCEW(kind.as_id() as u16),
                IMAGE_CURSOR,
                0,
                0,
                LR_SHARED,
            )
        };
        if cursor.is_null() {
            panic!("TODO: Handle errors")
        }

        let handle = unsafe { CopyImage(cursor, IMAGE_CURSOR, 0, 0, 0) };
        if handle.is_null() {
            panic!("TODO: Handle errors")
        }

        Self { handle }
    }

    pub fn replace_system(self, kind: CursorKind) -> ReplacedCursor {
        let cursor = Self::load_system(kind);
        // TODO: Handle errors
        unsafe { SetSystemCursor(self.handle as HICON, kind.as_id()) };
        ReplacedCursor { cursor, kind }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum CursorKind {
    /// Standard arrow and small hourglass
    AppStarting,
    /// The normal pointer
    Normal,
    /// Crosshair
    Crosshair,
    /// Hand
    Hand,
    /// I-Beam
    Ibeam,
    /// Slashed circle
    No,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll,
    /// Double-pointed arrow pointing northeast and southwest
    SizeNeSw,
    /// Double-pointed arrow pointing north and south
    SizeNs,
    /// Double-pointed arrow pointing northwest and southeast
    SizeNwSe,
    /// Double-pointed arrow pointing west and east
    SizeWe,
    /// Vertical arrow
    Up,
    /// Hourglass
    Wait,
}

impl CursorKind {
    pub fn as_id(self) -> u32 {
        match self {
            Self::AppStarting => OCR_APPSTARTING,
            Self::Normal => OCR_NORMAL,
            Self::Crosshair => OCR_CROSS,
            Self::Hand => OCR_HAND,
            Self::Ibeam => OCR_IBEAM,
            Self::No => OCR_NO,
            Self::SizeAll => OCR_SIZEALL,
            Self::SizeNeSw => OCR_SIZENESW,
            Self::SizeNs => OCR_SIZENS,
            Self::SizeNwSe => OCR_SIZENWSE,
            Self::SizeWe => OCR_SIZEWE,
            Self::Up => OCR_UP,
            Self::Wait => OCR_WAIT,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppStarting => "AppStarting",
            Self::Normal => "Normal",
            Self::Crosshair => "Crosshair",
            Self::Hand => "Hand",
            Self::Ibeam => "IBeam",
            Self::No => "No",
            Self::SizeAll => "SizeAll",
            Self::SizeNeSw => "SizeNeSw",
            Self::SizeNs => "SizeNS",
            Self::SizeNwSe => "SizeNWSE",
            Self::SizeWe => "SizeWE",
            Self::Up => "Up",
            Self::Wait => "Wait",
        }
    }
}

#[allow(dead_code)]
mod missing_from_winapi {
    pub const OCR_NORMAL: u32 = 32512;
    pub const OCR_IBEAM: u32 = 32513;
    pub const OCR_WAIT: u32 = 32514;
    pub const OCR_CROSS: u32 = 32515;
    pub const OCR_UP: u32 = 32516;
    /// Use OCR_SIZEALL instead
    pub const OCR_SIZE: u32 = 32640;
    /// Use OCR_NORMAL instead
    pub const OCR_ICON: u32 = 32641;
    pub const OCR_SIZENWSE: u32 = 32642;
    pub const OCR_SIZENESW: u32 = 32643;
    pub const OCR_SIZEWE: u32 = 32644;
    pub const OCR_SIZENS: u32 = 32645;
    pub const OCR_SIZEALL: u32 = 32646;
    /// Use OIC_WINLOGO instead
    pub const OCR_ICOCUR: u32 = 32647;
    pub const OCR_NO: u32 = 32648;
    pub const OCR_HAND: u32 = 32649;
    pub const OCR_APPSTARTING: u32 = 32650;
}
