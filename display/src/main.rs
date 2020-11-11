use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyleBuilder,
};
use ssd1306::{prelude::*, Builder};
use rppal::gpio::{Gpio};
use rppal::hal::{Delay};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() -> ! {
    let gpio = Gpio::new().unwrap();

    let mut rst_pin = gpio.get(24).unwrap().into_output();
    let mut dc_pin = gpio.get(23).unwrap().into_output();

    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0).unwrap();

    let interface = display_interface_spi::SPIInterfaceNoCS::new(spi, dc_pin);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();

    let mut delay = Delay::new();

    disp.reset(&mut rst_pin, &mut delay).unwrap();
    disp.init().unwrap();

    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // square
    Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(96, yoffset + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}
