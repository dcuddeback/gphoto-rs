# gphoto

The `gphoto` crate provides a safe wrapper around the native `libgphoto2` library.

## Dependencies
In order to use the `gphoto` crate, you must have a Unix system with the `libgphoto2` library
installed where it can be found by `pkg-config`.

On Debian-based Linux distributions, install the `libgphoto2-dev` package:

```
sudo apt-get install libgphoto2-dev
```

On OS X, install `libgphoto2` with Homebrew:

```
brew install libgphoto2
```

## Usage
Add `gphoto` as a dependency in `Cargo.toml`:

```toml
[dependencies]
gphoto = "0.1.0"
```

Import the `gphoto` crate. The starting point for nearly all `gphoto` functionality is to create a
context object. You can then autodetect a camera using the `Camera::autodetect()` function:

```rust
extern crate gphoto;

use std::path::Path;

fn main() {
  let mut context = gphoto::Context::new();

  let mut camera = gphoto::Camera::autodetect(&mut context).unwrap();
  let capture = camera.capture_image().unwrap();
  let mut file = gphoto::FileMedia::create(Path::new(capture.basename())).unwrap();

  camera.download(&capture, &mut file).unwrap()
}
```

### OS X Usage
OS X opens cameras automatically when connected, which prevents other applications from opening the
camera device. When attempting to open a camera that is already opened by the operating system, you
will get an error message like the following:

```
Could not claim the USB device
```

To fix this, you have to kill the `PTPCamera` process after connecting a camera to your system:

```
killall PTPCamera
```

Each camera is opened with a separate instance of the `PTPCamera` application. If you have several
cameras connected, you may want to kill individual `PTPCamera` processes instead of using `killall`.

## License
Copyright © 2015 David Cuddeback

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libgphoto2` C library, which is
licensed under the [LGPL version 2.1](https://github.com/gphoto/libgphoto2/blob/master/COPYING).
