mod util;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, ShowWindow, IsWindowVisible, SW_HIDE, SW_SHOW,
    EnumWindows,
};
use windows::Win32::Foundation::{HWND, LPARAM, BOOL, TRUE, FALSE};
use crate::util::string::StringExt;

#[cfg(windows)]
fn main(){
    let mut defview_hwnd: Option<HWND> = None;
    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(&mut defview_hwnd as *mut _ as isize));
    }
    if let Some(hwnd) = defview_hwnd {
        unsafe {
            let visible = IsWindowVisible(hwnd).as_bool();
            if visible {
                ShowWindow(hwnd, SW_HIDE);
                println!("Hidden");
            } else {
                ShowWindow(hwnd, SW_SHOW);
                println!("Shown");
            }
        }
    } else {
        println!("SHELLDLL_DefView was not found");
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let defview = FindWindowExW(Some(hwnd), None, "SHELLDLL_DefView".to_pcwstr(), None);
    if let Ok(defview_hwnd) = defview {
        let ptr = lparam.0 as *mut Option<HWND>;
        *ptr = Some(defview_hwnd);
        return FALSE;
    }
    TRUE
}
