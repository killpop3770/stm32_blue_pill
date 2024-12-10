#![no_main]
#![no_std]

use nb::block;
use panic_semihosting as _;

use stm32f1xx_hal::time::ms;
use stm32f1xx_hal::timer::{Channel, PwmExt, Tim2NoRemap};
use stm32f1xx_hal::{self as _, afio::AfioExt, flash::FlashExt, gpio::GpioExt, pac, rcc::RccExt, timer::Timer};
use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    ////////////////////////////////////////////////////////////////
    // Получаем доступ к периферии
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut afio = dp.AFIO.constrain();
    // Настройка системного тактирования
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    ////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////
    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Конфигурируем системный таймер на запуск обновления каждую секунду.
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(5.Hz()).unwrap();
    ////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////
    // Получаем доступ к порту и пину для подключения ШИМ PA0
    let mut gpioa = dp.GPIOA.split();
     let pwm_pin = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm_timer = dp
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pwm_pin, &mut afio.mapr, 50.Hz(), &clocks);
    ////////////////////////////////////////////////////////////////

    pwm_timer.enable(Channel::C1);
    // let max_duty = pwm_timer.get_max_duty();
    // pwm_timer.set_duty(Channel::C1, max_duty/4);
    // pwm_timer.set_period(ms(2).into_rate());
    // pwm_timer.set_duty(Channel::C1, 0);
    // pwm_timer.set_duty(Channel::C1, 2700);
    // cortex_m::asm::delay(10_000_000);
    // pwm_timer.set_duty(Channel::C1, 0);
    
    //?????
    // dp.TIM2.ccr1.modify(|_, w| unsafe { w.bits(config - 1) });

    loop {
        block!(timer.wait()).unwrap();
        led.set_low();        
        block!(timer.wait()).unwrap();
        led.set_high();
    }
}