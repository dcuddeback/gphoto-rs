extern crate gphoto;

fn main() {
    let version = gphoto::libgphoto2_version();
    println!("libgphoto2 {} {} {} {} {}", version.version(), version.camlibs(), version.compiler(), version.ltdl(), version.exif());
}
