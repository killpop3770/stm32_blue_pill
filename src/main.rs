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

    // Сервопривод работает при 50 герц, которые мы поставили на таймере TIM2
    // Частота таймера 50 герц == Период цикла шим 20 милисекунд
    // Если поставить ширину импульса на 1-2 милисекунды это изменит положение
    let servo_period = 20_000;
    let pulse_width_0deg = 1000; // Микросекунды (0 градусов) 2667
    let pulse_width_90deg = 1500; // Микросекунды (90 градусов) 4000
    let pulse_width_180deg = 2000; // Микросекунды (180 градусов) 5333

    // Рассчет значений для set_duty (важно для точности!) 
    // TODO: Использовать Decimal вместо float???
    // TODO: Посмотреть точные значение для пульсации по градусам
    // TODO: Какое точное значение у pwm_timer.get_max_duty(), меняется ли оно 
    // или необходимо все сместить к float???
    let duty_0deg = 2667; //(pulse_width_0deg/servo_period)*pwm_timer.get_max_duty();
    let duty_90deg = 5333; //(pulse_width_90deg/servo_period)*(pwm_timer.get_max_duty() as f16);
    let duty_180deg = 7330; //(pulse_width_180deg/servo_period)*(pwm_timer.get_max_duty() as f16);

    pwm_timer.enable(Channel::C1); // Включаем канал
    pwm_timer.set_period(ms(20).into_rate());
    
    loop {
        block!(timer.wait()).unwrap();
        led.set_low();        
        block!(timer.wait()).unwrap();
        led.set_high();

        pwm_timer.set_duty(Channel::C1, duty_0deg);
        cortex_m::asm::delay(10_000_000);
        pwm_timer.set_duty(Channel::C1, duty_90deg);
        cortex_m::asm::delay(10_000_000);
        pwm_timer.set_duty(Channel::C1, duty_180deg);
        cortex_m::asm::delay(10_000_000);
    }
}