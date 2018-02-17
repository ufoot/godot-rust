use sys;
use get_api;
use Variant;
use GodotType;

use std::ffi::CStr;
use std::ops::Range;
use std::str;
use std::slice;
use std::mem::transmute;

pub struct GodotString(pub(crate) sys::godot_string);

macro_rules! impl_methods {
    // Methods that return a GodotString:
    (
        $(pub fn $method:ident(&self) -> Self : $gd_method:ident;)*
    ) => {
        $(
            pub fn $method(&self) -> Self {
                unsafe {
                    GodotString((get_api().$gd_method)(&self.0))
                }
            }
        )*
    };

    // Methods that return a basic type:
    (
        $(pub fn $method:ident(&self) -> $Type:ty : $gd_method:ident;)*
    ) => {
        $(
            pub fn $method(&self) -> $Type {
                unsafe { (get_api().$gd_method)(&self.0) }
            }
        )*
    };
}

impl GodotString {
    pub fn from_str<S>(s: S) -> Self
        where S: AsRef<str>
    {
        unsafe {
            let api = get_api();
            let val = s.as_ref();
            let godot_s = (api.godot_string_chars_to_utf8_with_len)(
                val.as_ptr() as *const _,
                val.len() as _
            );

            GodotString(godot_s)
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            (get_api().godot_string_length)(&self.0) as usize
        }
    }

    impl_methods!(
        pub fn is_empty(&self) -> bool : godot_string_empty;
        pub fn is_numeric(&self) -> bool : godot_string_is_numeric;
        pub fn is_valid_float(&self) -> bool : godot_string_is_valid_float;
        pub fn is_valid_html_color(&self) -> bool : godot_string_is_valid_html_color;
        pub fn is_valid_identifier(&self) -> bool : godot_string_is_valid_identifier;
        pub fn is_valid_integer(&self) -> bool : godot_string_is_valid_integer;
        pub fn is_valid_ip_address(&self) -> bool : godot_string_is_valid_ip_address;
        pub fn is_resource_file(&self) -> bool : godot_string_is_resource_file;
        pub fn is_absolute_path(&self) -> bool : godot_string_is_abs_path;
        pub fn is_relative_path(&self) -> bool : godot_string_is_rel_path;
        pub fn to_f32(&self) -> f32 : godot_string_to_float;
        pub fn to_f64(&self) -> f64 : godot_string_to_double;
        pub fn to_i32(&self) -> i32 : godot_string_to_int;
        pub fn u32_hash(&self) -> u32 : godot_string_hash;
        pub fn u64_hash(&self) -> u64 : godot_string_hash64;
    );
    impl_methods!(
        pub fn camelcase_to_underscore(&self) -> Self : godot_string_camelcase_to_underscore;
        pub fn camelcase_to_underscore_lowercased(&self) -> Self : godot_string_camelcase_to_underscore_lowercased;
        pub fn capitalize(&self) -> Self : godot_string_capitalize;
        pub fn to_lowercase(&self) -> Self : godot_string_to_lower;
        pub fn to_uppercase(&self) -> Self : godot_string_to_upper;
        pub fn get_file(&self) -> Self : godot_string_get_file;
        pub fn get_base_dir(&self) -> Self : godot_string_get_base_dir;
        pub fn simplify_path(&self) -> Self : godot_string_simplify_path;
        pub fn sha256_text(&self) -> Self : godot_string_sha256_text;
        pub fn md5_text(&self) -> Self : godot_string_md5_text;
    );

    pub fn is_valid_hex_number(&self, with_prefix: bool) -> bool {
        unsafe {
            (get_api().godot_string_is_valid_hex_number)(&self.0, with_prefix)
        }
    }

    pub fn begins_with(&self, s: &GodotString) -> bool {
        unsafe {
            (get_api().godot_string_begins_with)(&self.0, &s.0)
        }
    }

    pub fn ends_with(&self, s: &GodotString) -> bool {
        unsafe {
            (get_api().godot_string_ends_with)(&self.0, &s.0)
        }
    }

    pub fn begins_with_c_str(&self, s: &CStr) -> bool {
        unsafe {
            (get_api().godot_string_begins_with_char_array)(&self.0, s.as_ptr())
        }
    }

    pub fn sub_string(&self, range: Range<usize>) -> Self {
        unsafe {
            let count = range.end - range.start;
            GodotString((get_api().godot_string_substr)(&self.0, range.start as i32, count as i32))
        }
    }

    // TODO: many missing methods.
}

impl_basic_traits!(
    for GodotString as godot_string {
        Drop => godot_string_destroy;
        Clone => godot_string_new_copy;
        Eq => godot_string_operator_equal;
        Default => godot_string_new;
    }
);

impl GodotType for GodotString {
    fn to_variant(&self) -> Variant { Variant::from_godot_string(self) }
    fn from_variant(variant: &Variant) -> Option<Self> { variant.to_godot_string() }
}

pub struct Utf8String(pub(crate) sys::godot_char_string);

impl Utf8String {
    pub fn len(&self) -> usize {
        unsafe {
            (get_api().godot_char_string_length)(&self.0) as usize
        }
    }

    fn data(&self) -> &u8 {
        unsafe {
            // casting from *const i8 to &u8
            transmute((get_api().godot_char_string_get_data)(&self.0))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.data(), self.len())
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(self.as_bytes())
        }
    }

    pub fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

impl_basic_traits!(
    for Utf8String as godot_char_string {
        Drop => godot_char_string_destroy;
    }
);


pub struct StringName(pub(crate) sys::godot_string_name);

impl StringName {
    pub fn from_str<S>(s: S)
        where S: AsRef<str>
    {
        let gd_string = GodotString::from_str(s);
        StringName::from_godot_string(&gd_string);
    }

    pub fn from_c_str(s: &CStr) -> Self {
        unsafe {
            let mut result = sys::godot_string_name::default();
            (get_api().godot_string_name_new_data)(&mut result, s.as_ptr());
            StringName(result)
        }
    }

    pub fn from_godot_string(s: &GodotString) -> Self {
        unsafe {
            let mut result = sys::godot_string_name::default();
            (get_api().godot_string_name_new)(&mut result, &s.0);
            StringName(result)
        }
    }

    pub fn get_hash(&self) -> u32 {
        unsafe {
            (get_api().godot_string_name_get_hash)(&self.0)
        }
    }

    pub fn get_name(&self) -> GodotString {
        unsafe {
            GodotString((get_api().godot_string_name_get_name)(&self.0))
        }
    }
}

impl_basic_traits! {
    for StringName as godot_string_name {
        Drop => godot_string_name_destroy;
        Eq => godot_string_name_operator_equal;
    }
}