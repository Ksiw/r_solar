use stm32f4xx_hal as hal;
use stm32f4xx_hal::{pac, prelude::*, serial::Config};

pub fn init_usart1(
    dp: pac::Peripherals,
    clocks: hal::rcc::Clocks,
) -> (
    stm32f4xx_hal::serial::Tx<pac::USART1>,
    stm32f4xx_hal::serial::Rx<pac::USART1>,
) {
    let gpiob = dp.GPIOB.split();
    let tx_pin = gpiob.pb6.into_alternate();
    let rx_pin = gpiob.pb7.into_alternate();

    let usart_config = Config::default()
        .baudrate(115200.bps())
        .wordlength_8()
        .parity_none();

    let serial = dp
        .USART1
        .serial((tx_pin, rx_pin), usart_config, &clocks)
        .unwrap();
    let (tx, rx) = serial.split();

    (tx, rx)
}
