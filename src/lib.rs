use std::slice::from_raw_parts;
use std::thread;

use encoding::{DecoderTrap, Encoding};
use encoding::all::GB18030;
use napi::{JsFunction, NapiRaw};
use napi::bindgen_prelude::{Buffer, Result};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use windows::Win32::Foundation::{HANDLE, HGLOBAL, HWND};
use windows::Win32::System::DataExchange::{
    CloseClipboard,
    EnumClipboardFormats,
    GetClipboardData,
    GetClipboardFormatNameA,
    OpenClipboard,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

use viewer::ClipboardViewer;

mod viewer;

// lazy_static! {
//     static ref CLIPBOARD_VIEWER: Arc<Mutex<Option<ClipboardViewer>>> = Arc::new(Mutex::new(None));
// }

static mut CLIPBOARD_VIEWER: Option<ClipboardViewer> = None;

#[napi(object)]
struct ClipboardFormat {
    pub id: u32,
    pub name: Option<String>,
}

unsafe fn get_global(handle: HANDLE, size: usize) -> &'static [u8] {
    let h_global = HGLOBAL(handle.0 as *mut _);
    let p = GlobalLock(h_global);
    if !p.is_null() {
        let data: &[u8] = from_raw_parts(p as *const u8, size);
        GlobalUnlock(h_global).ok();
        return data;
    }
    return &[];
}

#[napi]
fn get_clipboard_data(format: u32) -> Buffer {
    let mut data: &[u8] = &[];
    unsafe {
        let h_wnd: HWND = Default::default();
        if OpenClipboard(h_wnd).is_ok() {
            if let Ok(handle) = GetClipboardData(format) {
                let size = GlobalSize(HGLOBAL(handle.0 as *mut _));
                data = get_global(handle, size);
            }
            CloseClipboard().unwrap();
        }
    }
    return Buffer::from(data);
}

#[napi(js_name = "ansi2utf8")]
fn ansi_to_utf8(data: &[u8]) -> String {
    GB18030.decode(&data, DecoderTrap::Ignore)
        .map_or("\x1b[31m[Error when parse ANSI to UTF-8]\x1b[0m".to_string(), |v| v)
}

#[napi]
unsafe fn get_clipboard_format_name(format: u32) -> Option<String> {
    let buffer: &mut [u8] = &mut *vec![0; 256];
    let length = GetClipboardFormatNameA(format, buffer);
    return Some(std::str::from_utf8(&buffer[0..length as usize]).unwrap().to_string());
}

#[napi]
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

#[napi]
fn quit() {
    unsafe {
        if let Some(viewer) = CLIPBOARD_VIEWER.as_ref() {
            viewer.stop();
        }
    }
}

static mut ON_CLIPBOARD_UPDATED_CALLBACKS: Vec<Box<dyn Fn() -> Result<()>>> = Vec::new();

#[napi(ts_args_type = "callback: () => void")]
unsafe fn on_clipboard_update(callback: JsFunction) {
    // println!("the callback function is loaded: ");
    let thread_safe_func: ThreadsafeFunction<(), ErrorStrategy::CalleeHandled> = callback
        .create_threadsafe_function(0, |ctx| ctx.env.create_empty_array().map(|v| vec![v]))
        .unwrap();
    let b = Box::new(move || {
        thread_safe_func.call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
        return Ok(());
    });
    ON_CLIPBOARD_UPDATED_CALLBACKS.push(b);
}

#[napi::module_init]
unsafe fn init_listener() {
    thread::spawn(move || {
        if CLIPBOARD_VIEWER.is_none() {
            CLIPBOARD_VIEWER = Some(ClipboardViewer::new(move || {
                // println!("the clipboard is changed!! length: {}", ON_CLIPBOARD_UPDATED_CALLBACKS.len());
                for f in ON_CLIPBOARD_UPDATED_CALLBACKS.iter() {

                    f().expect("Failed to call callback function");
                }
            }));
            CLIPBOARD_VIEWER.as_ref().unwrap().listen();
        }
        // if let Ok(mut clipboard_viewer) = CLIPBOARD_VIEWER.lock() {
        //     if clipboard_viewer.is_none() {
        //         *clipboard_viewer = Some(ClipboardViewer::new(|| {
        //             println!("the clipboard is changed!!");
        //             let _ = callback();
        //         }));
        //         clipboard_viewer.as_ref().unwrap().listen();
        //     }
        // } else {
        //     println!("Failed to acquire lock on CLIPBOARD_VIEWER");
        // }
    });

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // println!("{:?}", get_clipboard_formats());
    }
}
