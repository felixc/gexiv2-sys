//! Example of how to access raw tag data via gexiv2.
//!
//! To run it, try:
//!   $ cargo run --features raw-tag-access --example raw_tag_access

mod open_buf;

#[cfg(feature = "raw-tag-access")]
mod example {
    extern crate gexiv2_sys as gexiv2;
    extern crate glib_sys as glib;
    extern crate libc;

    use std::ffi;
    use std::ptr;
    use std::slice;
    use std::str;

    use gexiv2_sys::GError;

    pub fn example() {
        unsafe {
            let metadata = crate::open_buf::make_new_metadata();
            let mut err: *mut GError = ptr::null_mut();

            let tag = ffi::CString::new("Exif.Image.ImageDescription").unwrap();
            let tag_value = ffi::CString::new("Raw Tag Access Example").unwrap();
            gexiv2::gexiv2_metadata_set_tag_string(metadata, tag.as_ptr(), tag_value.as_ptr(), &mut err);

            let raw_tag_struct = gexiv2::gexiv2_metadata_get_tag_raw(metadata, tag.as_ptr(), &mut err);

            let mut raw_tag_buffer_size: usize = 0;
            let raw_tag_buffer =
                glib::g_bytes_unref_to_data(raw_tag_struct, &mut raw_tag_buffer_size) as *mut u8;

            let raw_tag_value = slice::from_raw_parts(raw_tag_buffer, raw_tag_buffer_size);
            println!("{:?}", str::from_utf8(raw_tag_value));

            gexiv2::gexiv2_metadata_free(metadata);
        }
    }
}

#[cfg(not(feature = "raw-tag-access"))]
mod example {
    pub fn example() {
        println!("You have not enabled the 'raw-tag-access' feature!");
        println!("Try the --features raw-tag-access argument to Cargo.");
    }
}

fn main() {
    example::example();
}
