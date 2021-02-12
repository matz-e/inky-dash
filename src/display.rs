use gpio_cdev::{Chip, LineRequestFlags};

use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{CdevPin, Spidev};

use crate::ssd1675::{self, Builder, Dimensions, GraphicDisplay, Rotation};

#[rustfmt::skip]
const LUT: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUTT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    64,   12,   32,   12,    6,   // 0 Flash
    16,   8,    4,    4,     6,   // 1 clear
    4,    8,    8,    16,    16,  // 2 bring in the black
    2,    2,    2,    64,    32,  // 3 time for red
    2,    2,    2,    2,     2,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];

type Interface = ssd1675::interface::Interface<
    linux_embedded_hal::Spidev,
    linux_embedded_hal::CdevPin,
    linux_embedded_hal::CdevPin,
    linux_embedded_hal::CdevPin,
    linux_embedded_hal::CdevPin,
>;

pub type Display<'a> = GraphicDisplay<'a, Interface>;

pub struct Inky<'a> {
    display: Display<'a>,
    delay: Delay,
}

impl<'a> Inky<'a> {
    pub fn new(
        cols: u16,
        rows: u16,
        rotation: Rotation,
        black_buffer: &'a mut [u8],
        red_buffer: &'a mut [u8],
    ) -> Self {
        // Configure SPI
        let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(4_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options).expect("SPI configuration");

        // https://pinout.xyz/pinout/inky_phat
        // Configure Digital I/O Pins
        //
        // TODO This is a bogus pin for cs; Not actually used in the source of ssd1675!
        let mut chip = Chip::new("/dev/gpiochip0").expect("chip");
        let cs = CdevPin::new(
            chip.get_line(6)
                .expect("cs line")
                .request(LineRequestFlags::OUTPUT, 1, "cs export")
                .expect("cs request"),
        )
        .expect("cs pin");
        let busy = CdevPin::new(
            chip.get_line(17)
                .expect("busy line")
                .request(LineRequestFlags::INPUT, 0, "busy export")
                .expect("busy request"),
        )
        .expect("busy pin");
        let dc = CdevPin::new(
            chip.get_line(22)
                .expect("dc line")
                .request(LineRequestFlags::OUTPUT, 1, "dc export")
                .expect("dc request"),
        )
        .expect("dc pin");
        let reset = CdevPin::new(
            chip.get_line(27)
                .expect("reset line")
                .request(LineRequestFlags::OUTPUT, 1, "reset export")
                .expect("reset request"),
        )
        .expect("reset pin");

        let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);
        let config = Builder::new()
            .dimensions(Dimensions { rows, cols })
            .rotation(rotation)
            .lut(&LUT)
            .build()
            .expect("invalid configuration");
        let display = ssd1675::Display::new(controller, config);
        let display = GraphicDisplay::new(display, black_buffer, red_buffer);
        let delay = Delay {};
        Inky { delay, display }
    }

    pub fn display<T>(&mut self, update: T) -> Result<(), std::io::Error>
    where
        T: Fn(&mut GraphicDisplay<Interface>),
    {
        self.display
            .reset(&mut self.delay)
            .expect("error resetting display");
        update(&mut self.display);
        self.display
            .update(&mut self.delay)
            .expect("error updating display");
        self.display.deep_sleep()?;
        Ok(())
    }
}
