use std::{io, ops::Drop, path::Path, ptr};

use winapi::{
    shared::{windef::HCURSOR, winerror::ERROR_FILE_NOT_FOUND},
    um::{
        errhandlingapi::GetLastError,
        winuser::{
            CopyIcon, GetCursor, LoadCursorFromFileW, LoadCursorW, SetSystemCursor,
            MAKEINTRESOURCEW,
        },
    },
};

#[derive(Debug)]
pub struct ReplacedCursor {
    cursor: Cursor,
    kind: CursorKind,
}

impl ReplacedCursor {
    pub fn revert(&self) {
        unsafe { SetSystemCursor(self.cursor.handle, self.kind.as_id()) };
    }
}

impl Drop for ReplacedCursor {
    fn drop(&mut self) {
        self.revert();
    }
}

#[derive(Debug, PartialEq)]
pub struct Cursor {
    handle: HCURSOR,
}

impl Cursor {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.to_string_lossy();

        let utf16_path: Vec<u16> = path.encode_utf16().chain(0..=0).collect();
        let handle = unsafe { LoadCursorFromFileW(utf16_path.as_ptr()) };

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

    #[allow(dead_code)]
    pub fn current_system() -> Option<Self> {
        let handle = unsafe { GetCursor() };
        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }

    pub fn load_system(kind: CursorKind) -> Self {
        let cursor = unsafe { LoadCursorW(ptr::null_mut(), MAKEINTRESOURCEW(kind.as_id() as u16)) };
        if cursor.is_null() {
            panic!("TODO: Handle errors")
        }

        let handle = unsafe { CopyIcon(cursor) };

        if handle.is_null() {
            panic!("TODO: Handle errors")
        }

        Self { handle }
    }

    pub fn replace_system(self, kind: CursorKind) -> ReplacedCursor {
        let cursor = Self::load_system(kind);
        // TODO: Handle errors
        unsafe { SetSystemCursor(self.handle, kind.as_id()) };
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
    /// Arrow and question mark
    Help,
    /// I-Beam
    Ibeam,
    /// Slashed circle
    No,
    /// Four-pointed arrow pointing north, south, east, and west
    SizeAll,
    /// Double-pointed arrow pointing northeast and southwest
    SizeSw,
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
            Self::AppStarting => 32650,
            Self::Normal => 32512,
            Self::Crosshair => 32515,
            Self::Hand => 32649,
            Self::Help => 32651,
            Self::Ibeam => 32513,
            Self::No => 32648,
            Self::SizeAll => 32646,
            Self::SizeSw => 32643,
            Self::SizeNs => 32645,
            Self::SizeNwSe => 32642,
            Self::SizeWe => 32644,
            Self::Up => 32516,
            Self::Wait => 32514,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppStarting => "AppStarting",
            Self::Normal => "Normal",
            Self::Crosshair => "Crosshair",
            Self::Hand => "Hand",
            Self::Help => "Help",
            Self::Ibeam => "IBeam",
            Self::No => "No",
            Self::SizeAll => "SizeAll",
            Self::SizeSw => "SizeSW",
            Self::SizeNs => "SizeNS",
            Self::SizeNwSe => "SizeNWSE",
            Self::SizeWe => "SizeWE",
            Self::Up => "Up",
            Self::Wait => "Wait",
        }
    }
}
