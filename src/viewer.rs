use windows::core::PCSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::DataExchange::{AddClipboardFormatListener, RemoveClipboardFormatListener};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage, RegisterClassA, TranslateMessage, WINDOW_EX_STYLE, WINDOW_STYLE, WM_CLIPBOARDUPDATE, WM_CREATE, WM_DESTROY, WNDCLASSA};

pub struct ClipboardViewer {
    wc: WNDCLASSA,
    h_wnd: HWND,
    callback: Box<dyn Fn() -> ()>,
}


unsafe extern "system" fn default_proc(hwnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match u_msg {
        WM_CREATE => {
            // 注册剪贴板监视器
            println!("register clipboard listener");
            AddClipboardFormatListener(hwnd).ok();
        }
        WM_CLIPBOARDUPDATE => {
            // 处理剪贴板更新
            // 在这里可以添加你自己的处理逻辑
            println!("the clipboard is changed!!");
        }
        WM_DESTROY => {
            // 移除剪贴板监视器
            println!("remove clipboard listener");
            RemoveClipboardFormatListener(hwnd).ok();
            PostQuitMessage(0);
        }
        _ => {
            return DefWindowProcA(hwnd, u_msg, w_param, l_param);
        }
    }
    LRESULT(0)
}

impl ClipboardViewer {
    pub(crate) fn new<T: Fn() -> () + 'static>(callback: T) -> Self {
        let wc = WNDCLASSA {
            lpszClassName: PCSTR("OI!".as_ptr()),
            lpfnWndProc: Some(default_proc),
            ..Default::default()
        };
        let mut result = Self {
            wc,
            h_wnd: HWND::default(),
            callback: Box::new(callback),
        };
        unsafe {
            RegisterClassA(&wc);
            result.h_wnd = CreateWindowExA(
                WINDOW_EX_STYLE(0),
                wc.lpszClassName,
                PCSTR("".as_ptr()),
                WINDOW_STYLE(0),
                0,
                0,
                0,
                0,
                HWND::default(),
                None,
                None,
                None
            );
        }

        return result
    }

    pub(crate) fn listen(&self) {
        let mut msg = Default::default();
        unsafe {
            while let b_ret = GetMessageA(&mut msg, self.h_wnd, 0, 0) {
                if b_ret.0 == -1 {
                    // handle the error and possibly exit
                    eprintln!("error: GetMessage failed");
                } else {
                    if msg.message == WM_CLIPBOARDUPDATE {
                        (self.callback)();
                        continue;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        }
    }

    pub(crate) fn stop(&self) {
        unsafe {
            PostQuitMessage(0);
        }
    }
}
