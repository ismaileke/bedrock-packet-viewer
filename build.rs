#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();

    res.set_icon_with_id("src/assets/logo.ico", "0");

    res.set("FileDescription", "Bedrock Protocol Packet Viewer")
        .set("ProductName", "Bedrock Packet Viewer")
        .set("CompanyName", "ismaileke")
        .set("LegalCopyright", "Copyright © 2026 ismaileke. All rights reserved.")
        .set("OriginalFilename", "bedrock_packet_viewer.exe")
        .set("InternalName", "bedrock_packet_viewer")
        .set("ProductVersion", "1.0.0");

    if let Err(e) = res.compile() {
        eprintln!("Failed to set icon: {}", e);
        std::process::exit(1);

    }
}

#[cfg(not(windows))]
fn main() {}