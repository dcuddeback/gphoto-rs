extern crate gphoto;

use std::path::Path;

fn main() {
    let mut context = match gphoto::Context::new() {
        Ok(c) => c,
        Err(err) => panic!("error creating context: {}", err)
    };

    // open camera

    println!("opening camera ...");
    let mut camera = match gphoto::Camera::autodetect(&mut context) {
        Ok(c) => c,
        Err(err) => panic!("error opening camera: {}", err)
    };
    println!(" (done)");

    // capture image

    println!("capturing image ...");
    let capture = match camera.capture_image(&mut context) {
        Ok(c) => c,
        Err(err) => panic!("error capturing image: {}", err)
    };
    println!(" (done) {:?}", capture.basename());

    // download file

    let mut file = match gphoto::FileMedia::create(Path::new(capture.basename().as_ref())) {
        Ok(f) => f,
        Err(err) => panic!("error saving file: {}", err)
    };

    println!("downloading ...");
    if let Err(err) = camera.download(&mut context, &capture, &mut file) {
        panic!("error downloading file: {}", err);
    }
    println!(" (done)");
}
