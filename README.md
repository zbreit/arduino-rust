# arduino-rust

This repo contains simple examples for teaching myself how to use embedded Rust with my Arduino Uno. I'm running on Arch, so all system install commands will use pacman.

## Setup

First, let's install required 3rd-party tools for the AVR cross-compiler:

```bash
sudo pacman -S avr-gcc avr-binutils avr-libc avrdude
```

We'll also have to add a udev rule so that `avrdude` can properly flash the Arduino Uno:

```bash
# See https://support.arduino.cc/hc/en-us/articles/9005041052444-Fix-udev-rules-on-Linux for
# more info.
# Setup a udev rule that grants the user access to the Arduino Uno.
echo 'SUBSYSTEMS=="usb",KERNEL=="ttyACM*",ATTRS{idVendor}=="2341",ATTRS{idProduct}=="0043",MODE="0666",TAG+="uaccess"' | sudo tee /etc/udev/rules.d/55-arduino-uno.rules
sudo chmod 0644 /etc/udev/rules.d/55-arduino-uno.rules
sudo udevadm trigger
sudo udevadm control --reload
```

Next, we have to install the nightly Rust compiler, which gives us access to the `avr-none` target. See [rustc's docs on `avr-none`](https://doc.rust-lang.org/nightly/rustc/platform-support/avr-none.html) for more info on this build target.

```bash
rustup toolchain install nightly
rustup override set nightly
```

Next let's setup [ravedude](https://github.com/Rahix/avr-hal/blob/main/ravedude/README.md): a Rust "runner" that will automatically flash the Arduino using `avrdude` every time we call `cargo run`. Install it with the following command:

``bash
cargo +stable install --locked ravedude
```

Now, you can configure your target architecture, CPU, and runner behavior in `.cargo/config.toml`.

You likely have to build with `cargo run --release` since the ROM size on the atmega328p is fairly small.
