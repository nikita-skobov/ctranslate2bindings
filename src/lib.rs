use std::os::raw;
use std::ffi::{CStr, CString};

#[repr(C)]
pub struct FfiTranslator {
    _dummy: [u8; 0],
}

#[repr(C)]
pub struct FfiTranslateResult {
    pub m_data: *mut *const raw::c_char,
    pub size: raw::c_int,
}

#[link(name = "ctranslate2")]
extern {
    pub fn ct2_translator_new(model_path: *const raw::c_char) -> *mut FfiTranslator;

    // TODO: add free

    pub fn ct2_translator_translate(t: *mut FfiTranslator, data: *mut *const raw::c_char, length: raw::c_int) -> FfiTranslateResult;
}


struct TranslatorInner {
    ptr: *mut FfiTranslator,
}

pub struct Translator {
    inner: TranslatorInner,
}

impl Translator {
    pub fn new(path: &str) -> Option<Translator> {
        let path = CString::new(path).ok()?;
        let translator_inner = unsafe { ct2_translator_new(path.as_ptr()) };
        if translator_inner.is_null() {
            return None;
        }
        let translator = Translator {
            inner: TranslatorInner {
                ptr: translator_inner
            }
        };
        Some(translator)
    }

    pub fn translate(&self, data: Vec<&str>) -> Vec<String> {
        let mut newvec = vec![];
        let size = data.len();
        for s in data {
            let cstr = CString::new(s).ok().unwrap();
            newvec.push(cstr);
        }
        let mut newvec2 = vec![];
        for cstr in &newvec {
            newvec2.push(cstr.as_ptr());
        }
        let ffi_translate_result = unsafe {
            ct2_translator_translate(self.inner.ptr, newvec2.as_mut_ptr(), size as raw::c_int)
        };

        // println!("Got result of size {}", ffi_translate_result.size);
        let stuff: &[*const raw::c_char] = unsafe {
            core::slice::from_raw_parts(
                ffi_translate_result.m_data,
                ffi_translate_result.size as usize
            )
        };
        let mut myrustvec: Vec<String> = vec![];
        for c_str in stuff {
            let actualstr = unsafe { CStr::from_ptr(*c_str) };
            let s = actualstr.to_str().unwrap();
            let s = s.to_string();
            myrustvec.push(s);
        }
        // println!("{:#?}", myrustvec);
        myrustvec
    }
}
