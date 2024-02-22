use std::slice::from_raw_parts;
use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
// use windows::core::PCSTR;
use windows::Win32::Foundation::{HANDLE, HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{
    OpenClipboard,
    CloseClipboard,
    EnumClipboardFormats,
    GetClipboardData,
    GetClipboardFormatNameA
};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

#[napi(object)]
struct ClipboardFormat {
    pub id: u32,
    pub name: Option<String>,
}

// TODO too
// #[napi(js_name = "registerClipboardFormat")]
// unsafe fn register_clipboard_format(format_name: String) -> u32 {
//     return RegisterClipboardFormatA(PCSTR(format_name.as_ptr()));
// }

unsafe fn get_global(handle: HANDLE, size: usize) -> &'static[u8] {
    let h_global = HGLOBAL(handle.0 as *mut _);
    let p = GlobalLock(h_global);
    if !p.is_null() {
        let data: &[u8] = from_raw_parts(p as *const u8, size);
        GlobalUnlock(h_global).unwrap();
        return data;
    }
    return &[];
}

#[napi(js_name = "getClipboardData")]
fn get_clipboard_data(format: u32) -> Buffer {
    let mut data: &[u8] = &[];
    let s;
    // let b;
    unsafe {
        let h_wnd: HWND = Default::default();
        if OpenClipboard(h_wnd).is_ok() {
            let handle = GetClipboardData(format).unwrap();

            let size = GlobalSize(HGLOBAL(handle.0 as *mut _));
            data = get_global(handle, size);
            CloseClipboard().unwrap();
        }
    }
    #[cfg(windows)]
    match format {
        1 | 7 => {
            s = ansi_to_utf8(data);
            data = s.as_bytes();
            if let Some(&last_byte) = data.last() {
                println!("{:?}", data);
                if last_byte == 0 {
                    data = &data[..data.len() - 1];
                }
            }
        }
        _ => {}
    }
    // if let Some(&last_byte) = data.last() {
    //     if last_byte == 0 {
    //         data = &data[..data.len()-1];
    //     }
    // }
    return Buffer::from(data);
}

fn ansi_to_utf8(data: &[u8]) -> String {
    GB18030.decode(data, DecoderTrap::Strict).map_or("\x1b[31m[Error when parse ANSI to UTF-8]\x1b[0m".to_string(), |v|v)
}

#[napi(js_name = "getClipboardFormatName")]
unsafe fn get_clipboard_format_name(format: u32) -> Option<String> {
    let buffer: &mut [u8] = &mut *vec![0; 256];
    let length = GetClipboardFormatNameA(format, buffer);
    return Some(std::str::from_utf8(&buffer[0..length as usize]).unwrap().to_string());
}

#[napi(js_name = "getClipboardFormats")]
fn get_clipboard_formats() -> Vec<ClipboardFormat> {
    let mut vec: Vec<ClipboardFormat> = Vec::new();
    unsafe {
        let h_wnd: HWND = Default::default();
        if OpenClipboard(h_wnd).is_ok() {
            let mut format = EnumClipboardFormats(0);
            while format != 0 {
                vec.push(ClipboardFormat {
                    id: format,
                    name: get_clipboard_format_name(format),
                });
                format = EnumClipboardFormats(format);
            }

            CloseClipboard().unwrap();
        }
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
