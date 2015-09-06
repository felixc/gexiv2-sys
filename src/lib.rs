// Copyright © 2015 Felix A. Crux <felixc@felixcrux.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! Raw FFI declarations for gexiv2.
//!
//! This library provides Rust FFI declarations for the gexiv2 library, which
//! is a GObject-based wrapper around the Exiv2 library, which provides read
//! and write access to the Exif, XMP, and IPTC metadata in media files.
//!
//! Only FFI declarations are provided here; for a usable Rust library,
//! consider the rexiv2 crate.

#![crate_type = "lib"]

extern crate libc;

use self::libc::{c_char, c_int, c_double, c_long};

/// An opaque structure that serves as a container for a media file's metadata.
#[repr(C)]
pub struct GExiv2Metadata;

/// Container for information about recoverable runtime errors.
#[repr(C)]
pub struct GError {
    /// Part of the program from which the error originated.
    pub domain: u32,
    /// Identifier for the error that occurred.
    pub code: c_int,
    /// Human-readable description of the problem.
    pub message: *const c_char
}

/// All the possible orientations for an image.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Orientation {
    Unspecified,
    Normal,
    HorizontalFlip,
    Rotate180,
    VerticalFlip,
    Rotate90HorizontalFlip,
    Rotate90,
    Rotate90VerticalFlip,
    Rotate270,
}

/// Log levels.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GExiv2LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    MUTE,
}

/// Handler function that receives gexiv2 log messages and processes them as desired.
pub type GExiv2LogHandler = extern fn(level: GExiv2LogLevel, msg: *const c_char);

#[link(name = "gexiv2")]
extern {
    pub fn gexiv2_get_version() -> c_int;
    pub fn gexiv2_initialize() -> bool;

    // GExiv2Metadata lifecycle management.
    pub fn gexiv2_metadata_new() -> *mut GExiv2Metadata;
    pub fn gexiv2_metadata_free(this: *mut GExiv2Metadata);
    pub fn gexiv2_metadata_open_path(this: *mut GExiv2Metadata, path: *const c_char, error: *mut *mut GError) -> bool;
    pub fn gexiv2_metadata_open_buf(Gthis: *mut GExiv2Metadata, data: *const u8, data_len: c_long, error: *mut *mut GError) -> bool;
    pub fn gexiv2_metadata_save_file(this: *mut GExiv2Metadata, path: *const c_char, error: *mut *mut GError) -> bool;

    // Image information.
    pub fn gexiv2_metadata_get_supports_exif(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_get_supports_iptc(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_get_supports_xmp(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_get_mime_type(this: *mut GExiv2Metadata) -> *const c_char;
    pub fn gexiv2_metadata_get_pixel_width(this: *mut GExiv2Metadata) -> c_int;
    pub fn gexiv2_metadata_get_pixel_height(this: *mut GExiv2Metadata) -> c_int;

    // Tag management.
    pub fn gexiv2_metadata_has_tag(this: *mut GExiv2Metadata, tag: *const c_char) -> bool;
    pub fn gexiv2_metadata_clear_tag(this: *mut GExiv2Metadata, tag: *const c_char) -> bool;
    pub fn gexiv2_metadata_clear(this: *mut GExiv2Metadata);
    pub fn gexiv2_metadata_has_exif(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_clear_exif(this: *mut GExiv2Metadata);
    pub fn gexiv2_metadata_get_exif_tags(this: *mut GExiv2Metadata) -> *const *const c_char;
    pub fn gexiv2_metadata_has_xmp(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_clear_xmp(this: *mut GExiv2Metadata);
    pub fn gexiv2_metadata_get_xmp_tags(this: *mut GExiv2Metadata) -> *const *const c_char;
    pub fn gexiv2_metadata_has_iptc(this: *mut GExiv2Metadata) -> bool;
    pub fn gexiv2_metadata_clear_iptc(this: *mut GExiv2Metadata);
    pub fn gexiv2_metadata_get_iptc_tags(this: *mut GExiv2Metadata) -> *const *const c_char;

    // Tag data getters/setters.
    pub fn gexiv2_metadata_get_tag_string(this: *mut GExiv2Metadata, tag: *const c_char) -> *const c_char;
    pub fn gexiv2_metadata_set_tag_string(this: *mut GExiv2Metadata, tag: *const c_char, value: *const c_char) -> bool;
    pub fn gexiv2_metadata_get_tag_interpreted_string(this: *mut GExiv2Metadata, tag: *const c_char) -> *const c_char;
    pub fn gexiv2_metadata_get_tag_multiple(this: *mut GExiv2Metadata, tag: *const c_char) -> *const *const c_char;
    pub fn gexiv2_metadata_set_tag_multiple(this: *mut GExiv2Metadata, tag: *const c_char, values: *const *const c_char) -> bool;
    pub fn gexiv2_metadata_get_tag_long(this: *mut GExiv2Metadata, tag: *const c_char) -> c_long;
    pub fn gexiv2_metadata_set_tag_long(this: *mut GExiv2Metadata, tag: *const c_char, value: c_long) -> bool;
    pub fn gexiv2_metadata_get_exif_tag_rational(this: *mut GExiv2Metadata, tag: *const c_char, nom: *mut c_int, den: *mut c_int) -> bool;
    pub fn gexiv2_metadata_set_exif_tag_rational(this: *mut GExiv2Metadata, tag: *const c_char, nom: c_int, den: c_int) -> bool;

    // Helper & convenience getters/setters.
    pub fn gexiv2_metadata_get_orientation(this: *mut GExiv2Metadata) -> Orientation;
    pub fn gexiv2_metadata_set_orientation(this: *mut GExiv2Metadata, orientation: Orientation);
    pub fn gexiv2_metadata_get_exposure_time(this: *mut GExiv2Metadata, nom: *mut c_int, den: *mut c_int) -> bool;
    pub fn gexiv2_metadata_get_fnumber(this: *mut GExiv2Metadata) -> c_double;
    pub fn gexiv2_metadata_get_focal_length(this: *mut GExiv2Metadata) -> c_double;
    pub fn gexiv2_metadata_get_iso_speed(this: *mut GExiv2Metadata) -> c_int;
    pub fn gexiv2_metadata_get_comment(this: *mut GExiv2Metadata) -> *const c_char;
    pub fn gexiv2_metadata_set_comment(this: *mut GExiv2Metadata, comment: *const c_char);
    pub fn gexiv2_metadata_clear_comment(this: *mut GExiv2Metadata);

    // GPS-related functions.
    pub fn gexiv2_metadata_get_gps_longitude(this: *mut GExiv2Metadata, longitude: *mut c_double) -> bool;
    pub fn gexiv2_metadata_get_gps_latitude(this: *mut GExiv2Metadata, latitude: *mut c_double) -> bool;
    pub fn gexiv2_metadata_get_gps_altitude(this: *mut GExiv2Metadata, altitude: *mut c_double) -> bool;
    pub fn gexiv2_metadata_get_gps_info(this: *mut GExiv2Metadata, longitude: *mut c_double, latitude: *mut c_double, altitude: *mut c_double) -> bool;
    pub fn gexiv2_metadata_set_gps_info(this: *mut GExiv2Metadata, longitude: c_double, latitude: c_double, altitude: c_double) -> bool;
    pub fn gexiv2_metadata_delete_gps_info(this: *mut GExiv2Metadata);

    // Tag information functions.
    pub fn gexiv2_metadata_is_exif_tag(tag: *const c_char) -> bool;
    pub fn gexiv2_metadata_is_iptc_tag(tag: *const c_char) -> bool;
    pub fn gexiv2_metadata_is_xmp_tag(tag: *const c_char) -> bool;
    pub fn gexiv2_metadata_get_tag_label(tag: *const c_char) -> *const c_char;
    pub fn gexiv2_metadata_get_tag_description(tag: *const c_char) -> *const c_char;
    pub fn gexiv2_metadata_get_tag_type(tag: *const c_char) -> *const c_char;

    // XMP namespace management.
    pub fn gexiv2_metadata_register_xmp_namespace(name: *const c_char, prefix: *const c_char) -> bool;
    pub fn gexiv2_metadata_unregister_xmp_namespace(name: *const c_char) -> bool;
    pub fn gexiv2_metadata_unregister_all_xmp_namespaces();

    // Logging
    pub fn gexiv2_log_get_default_handler() -> GExiv2LogHandler;
    pub fn gexiv2_log_get_handler() -> GExiv2LogHandler;
    pub fn gexiv2_log_set_handler(handler: GExiv2LogHandler);
    pub fn gexiv2_log_get_level() -> GExiv2LogLevel;
    pub fn gexiv2_log_set_level(level: GExiv2LogLevel);
    pub fn gexiv2_log_use_glib_logging();
}
