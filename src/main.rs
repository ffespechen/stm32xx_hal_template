#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Obtener acceso a los periféricos del núcleo de la cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Obtener acceso a los periféricos específicos del dispositivo desde el crate de acceso a periféricos
    let dp = pac::Peripherals::take().unwrap();

    // Tomar posesión sobre los dispositivos raw flash y rcc y convertirlos en los correspondientes
    // estructuras HAL
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Congela la configuración de todos los relojes del sistema y almacena las frecuencias congeladas en
    // `relojes`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Adquirir el periférico GPIOC
    let mut gpioc = dp.GPIOC.split();

    // Configura el pin 13 de gpio C como una salida push-pull. El registro `crh` se pasa a la función
    // para configurar el puerto. Para los pines 0-7, crl debe ser pasado en su lugar.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configurar el temporizador del sistema para que se actualice cada segundo
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();



    // Loop principal de la aplicación
    loop {
        block!(timer.wait()).unwrap();
        led.set_high();
        block!(timer.wait()).unwrap();
        led.set_low();
    }
}