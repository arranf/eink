use embedded_graphics::{
    fonts::{Font12x16, Font6x8, Text},
    prelude::*,
    primitives::{Circle, Line},
    style::PrimitiveStyle,
    text_style,
};
use embedded_hal::prelude::*;
use epd_waveshare::{
    color::*,
    epd7in5_v2::{Display7in5, EPD7in5},
    graphics::{Display, DisplayRotation},
    prelude::*,
};

use rppal::gpio::Gpio;
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

// activate spi, gpio in raspi-config
// needs to be run with sudo because of some sysfs_gpio permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

fn main() -> Result<(), std::io::Error> {
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

    println!("Setting up EPD7in5");
    let mut epd7in5 =
        EPD7in5::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");

    println!("Test all the rotations");
    let mut display = Display7in5::default();

    // display.set_rotation(DisplayRotation::Rotate0);
    draw_text(&mut display, "Welcome to Valinde", 5, 50);
    // println!("Rotate 0");

    display.set_rotation(DisplayRotation::Rotate90);
    // draw_text(&mut display, "Rotate 90!", 5, 50);
    // println!("Rotate 90");

    // display.set_rotation(DisplayRotation::Rotate180);
    // draw_text(&mut display, "Rotate 180!", 5, 50);
    // println!("Rotate 180");

    // display.set_rotation(DisplayRotation::Rotate270);
    // draw_text(&mut display, "Rotate 270!", 5, 50);

    epd7in5
        .update_frame(&mut spi, &display.buffer())
        .expect("Expect frame update");
    epd7in5
        .display_frame(&mut spi)
        .expect("display frame new graphics");
    // delay.delay_ms(5000u16);

    //println!("Now test new graphics with default rotation and some special stuff:");
    // display.clear_buffer(Color::White);

    // draw a analog clock
    // let _ = Circle::new(Point::new(64, 64), 64)
    //     .into_styled(PrimitiveStyle::with_stroke(Black, 1))
    //     .draw(&mut display);
    // let _ = Line::new(Point::new(64, 64), Point::new(0, 64))
    //     .into_styled(PrimitiveStyle::with_stroke(Black, 1))
    //     .draw(&mut display);
    // let _ = Line::new(Point::new(64, 64), Point::new(80, 80))
    //     .into_styled(PrimitiveStyle::with_stroke(Black, 1))
    //     .draw(&mut display);

    // draw white on black background
    let _ = Text::new("It's working-WoB!", Point::new(175, 250))
        .into_styled(text_style!(
            font = Font6x8,
            text_color = White,
            background_color = Black
        ))
        .draw(&mut display);

    // use bigger/different font
    // let _ = Text::new("It's working-WoB!", Point::new(50, 200))
    //     .into_styled(text_style!(
    //         font = Font12x16,
    //         text_color = White,
    //         background_color = Black
    //     ))
    //     .draw(&mut display);

    // a moving `Hello World!`
    // let limit = 10;
    // epd7in5.clear_frame(&mut spi).unwrap();
    // for i in 0..limit {
    //     //println!("Moving Hello World. Loop {} from {}", (i + 1), limit);

    //     draw_text(&mut display, "  Hello World! ", 5 + i * 12, 50);

    //     epd7in5.update_frame(&mut spi, &display.buffer()).unwrap();
    //     epd7in5
    //         .display_frame(&mut spi)
    //         .expect("display frame new graphics");

    //     delay.delay_ms(1_000u16);
    // }

    println!("Finished tests - going to sleep");
    epd7in5.sleep(&mut spi).expect("Expect sleep to succeed");
    Ok(())
}

fn draw_text(display: &mut Display7in5, text: &str, x: i32, y: i32) {
    let _ = Text::new(text, Point::new(x, y))
        .into_styled(text_style!(
            font = Font6x8,
            text_color = Black,
            background_color = White
        ))
        .draw(display);
}
