# Mutiny

GTK4 Revolt client

## Installation

First, make sure the following components are installed:

* (The meson build system)[https://mesonbuild.com/Quick-guide.html]
* (The Rust toolchain)[https://www.rust-lang.org/tools/install]
* (The GTK4 library)[https://github.com/ToshioCP/Gtk4-tutorial/blob/main/gfm/sec2.md]
* (The Adwaita library)[https://github.com/GNOME/libadwaita]

After installing the necessary components, set up `builddir`:
```sh
meson setup builddir
```

Once this is done, compile the app:
```sh
cd builddir && meson compile
```

Unfortunely, in some cases it is difficult to utilize `meson install` properly. In this case the user can just specify the prefix to the current build directory:
```sh
# inside builddir
meson configure -D `pwd`
```

When this is done, the resources will be located inside the `share` folder on the build directory.

If the app is not installed with `meson install`, you'll need to tell glib that the schemas reside inside the `data/` folder; therefore, run the client with this command:
```sh
# inside builddir
XDG_DATA_DIRS="$XDG_DATA_DIRS:$PWD/share" ./src/mutiny 
```

## Useful References

https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/
