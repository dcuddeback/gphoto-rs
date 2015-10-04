extern crate gphoto;

fn main() {
    let mut context = match gphoto::Context::new() {
        Ok(c) => c,
        Err(err) => panic!("error creating context: {}", err)
    };

    let mut camera = match gphoto::Camera::autodetect(&mut context) {
        Ok(c) => c,
        Err(err) => panic!("error opening camera: {}", err)
    };

    {
        let port = camera.port();

        println!("[port info]");
        println!("port type = {:?}", port.port_type());
        println!("port name = {:?}", port.name());
        println!("port path = {:?}", port.path());
    }

    let abilities = camera.abilities();

    println!("\n[abilities]");
    println!("      device type = {:?}", abilities.device_type());
    println!("            model = {:?}", abilities.model());
    println!("    driver status = {:?}", abilities.driver_status());
    println!("       port types = {:?}", abilities.port_types());
    println!("           speeds = {:?}", abilities.speeds());
    println!("camera operations = {:?}", abilities.camera_operations());
    println!("  file operations = {:?}", abilities.file_operations());
    println!("folder operations = {:?}", abilities.folder_operations());
    println!("       USB vendor = {:?}", abilities.usb_vendor());
    println!("      USB product = {:?}", abilities.usb_product());
    println!("        USB class = {:?}", abilities.usb_class());
    println!("     USB subclass = {:?}", abilities.usb_subclass());
    println!("     USB protocol = {:?}", abilities.usb_protocol());

    match camera.storage(&mut context) {
        Ok(storage) => {
            for s in storage {
                println!("\n[storage]");
                println!("       base dir = {:?}", s.base_dir());
                println!("          label = {:?}", s.label());
                println!("    description = {:?}", s.description());
                println!("   storage type = {:?}", s.storage_type());
                println!("filesystem type = {:?}", s.filesystem_type());
                println!("    access type = {:?}", s.access_type());
                println!("    capacity kb = {:?}", s.capacity_kbytes());
                println!("        free kb = {:?}", s.free_kbytes());
                println!("    free images = {:?}", s.free_images());
            }
        },
        Err(err) => println!("\ncould not retrieve storage information: {}", err)
    }

    if let Ok(s) = camera.summary(&mut context) {
        println!("\n[summary]\n{}", s);
    }

    if let Ok(s) = camera.manual(&mut context) {
        println!("\n[manual]\n{}", s);
    }

    if let Ok(s) = camera.about_driver(&mut context) {
        println!("\n[driver]\n{}", s);
    }
}
