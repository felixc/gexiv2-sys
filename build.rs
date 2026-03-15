// Copyright © 2015-2026 Felix A. Crux <felixc@felixcrux.com> and contributors
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

//! Confirm gexiv2 library exists on the system.

fn main() {
    let system_library = pkg_config::Config::new()
        .atleast_version("0.10")
        .probe("gexiv2")
        .unwrap_or_else(|e| {
            eprintln!(
                "\nThe gexiv2 library was not found by pkg-config/pkgconf on your system.\n\n\
                 Consult README.md or SETUP.md for suggestions on how to acquire it."
            );
            panic!("{}", e);
        });

    let version_parts: Vec<u32> = system_library
        .version
        .split('.')
        .map(|s| s.parse().expect("Unable to parse version number."))
        .collect();

    assert!(
        version_parts.len() >= 2,
        "Failed to parse gexiv2 version '{}' into 'major.minor' format",
        system_library.version
    );

    let major = version_parts[0];
    let minor = version_parts[1];

    let milestones = [(0, 10), (0, 12), (0, 14), (0, 16)];

    for &(milestone_major, milestone_minor) in &milestones {
        let flag = format!("gexiv2_v{}_{}_or_newer", milestone_major, milestone_minor);
        println!("cargo:rustc-check-cfg=cfg({})", flag);
        if (major, minor) >= (milestone_major, milestone_minor) {
            println!("cargo:rustc-cfg={}", flag);
        }
    }
}
