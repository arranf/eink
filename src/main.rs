use embedded_graphics::{
    image::ImageRaw,
    pixelcolor::{raw::BigEndian, BinaryColor},
    prelude::*,
};
use epd_waveshare::{
    epd7in5_v2::{Display7in5, EPD7in5},
    graphics::Display,
    prelude::*,
};
use rand::seq::SliceRandom;
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use embedded_graphics::image::Image;

use rppal::gpio::Gpio;
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let (mut spi, mut epd7in5) = setup_waveshare();

    let mut display = Display7in5::default();

    let data = get_random_image().with_context(|| "Failed to get random image")?;
    let raw_image = ImageRaw::<BinaryColor, BigEndian>::new(&data, 800, 480);

    let image = Image::new(&raw_image, Point::zero());

    image
        .draw(&mut display)
        .with_context(|| "Failed to draw to screen")?;

    epd7in5
        .update_frame(&mut spi, &display.buffer())
        .with_context(|| "Failed to update frame")?;
    epd7in5
        .display_frame(&mut spi)
        .with_context(|| "Failed to display frame")?;

    println!("Finished rendering - going to sleep");
    epd7in5.sleep(&mut spi).with_context(|| "Failed to sleep")?;
    Ok(())
}

fn get_random_image() -> Result<Vec<u8>> {
    let entries: Vec<PathBuf> = fs::read_dir(".")
        .with_context(|| "Failed to read directory")?
        .filter_map(|file| file.ok())
        .filter(|entry| {
            let path = entry.path();
            let extension = path.extension().and_then(OsStr::to_str);
            match extension {
                Some(extension) => extension == "txt",
                None => false,
            }
        })
        .map(|entry| entry.path())
        .collect();
    let chosen = entries
        .choose(&mut rand::thread_rng())
        .with_context(|| "Failed choose image file as there are none available")?;
    let mut data = Vec::new();
    File::open(chosen)
        .with_context(|| format!("Failed to open file {}", chosen.display()))?
        .read_to_end(&mut data)
        .with_context(|| format!("Failed to read file {} to end", chosen.display()))?;
    Ok(data)
}

fn setup_waveshare() -> (
    Spi,
    EPD7in5<
        Spi,
        rppal::gpio::OutputPin,
        rppal::gpio::OutputPin,
        rppal::gpio::OutputPin,
        rppal::gpio::OutputPin,
    >,
) {
    // activate spi, gpio in raspi-config
    // needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
    // see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

    // This code matches the pins described in https://www.waveshare.com/wiki/7.5inch_e-Paper_HAT
    // It also matches the code from https://github.com/waveshare/e-Paper
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 4_000_000, Mode::Mode0)
        .expect("Unable to configure SPI");
    spi.set_bits_per_word(8).expect("Set bits per word");

    let mut rst = Gpio::new()
        .expect("Failed to get GPIO")
        .get(17) // Board 11 BCM 17
        .expect("Failed to get BCM Pin 17 for RST")
        .into_output();
    rst.set_low();
    rst.set_high();

    let mut dc = Gpio::new()
        .expect("Failed to get GPIO")
        .get(25) //Board 22, BCM 25
        .expect("Failed to get BCM Pin 25 for RST")
        .into_output();
    dc.set_low();
    dc.set_high();

    let mut cs = Gpio::new()
        .expect("Failed to get GPIO")
        .get(8) //Board 24, BCM 8
        .expect("Failed to get BCM Pin 8 for RST")
        .into_output();
    cs.set_high();

    let busy = Gpio::new()
        .expect("Failed to get GPIO")
        .get(24) // Board 18, BCM 24
        .expect("Failed to get BCM Pin 24 for RST")
        .into_output();
    let mut delay = Delay {};
    let epd7in5 =
        EPD7in5::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");
    (spi, epd7in5)
}
