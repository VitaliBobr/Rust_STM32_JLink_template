#![no_main]
#![no_std]

use my_app as _; // global logger + panicking-behavior + memory layout
//use stm32f1xx_hal::;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    let asd : i32 = 5;
    defmt::println!("{}",asd);
    my_app::exit()
}
