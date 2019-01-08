#![doc = "Peripheral access API for NRF9160 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn SPU();
    fn CLOCK_POWER();
    fn UARTE0_SPIM0_SPIS0_TWIM0_TWIS0();
    fn UARTE1_SPIM1_SPIS1_TWIM1_TWIS1();
    fn UARTE2_SPIM2_SPIS2_TWIM2_TWIS2();
    fn UARTE3_SPIM3_SPIS3_TWIM3_TWIS3();
    fn GPIOTE0();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn RTC1();
    fn WDT();
    fn EGU0();
    fn EGU1();
    fn EGU2();
    fn EGU3();
    fn EGU4();
    fn EGU5();
    fn PWM0();
    fn PWM1();
    fn PWM2();
    fn PWM3();
    fn PDM();
    fn I2S();
    fn IPC();
    fn FPU();
    fn GPIOTE1();
    fn KMU();
    fn CRYPTOCELL();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 65] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPU },
    Vector { _reserved: 0 },
    Vector {
        _handler: CLOCK_POWER,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: UARTE0_SPIM0_SPIS0_TWIM0_TWIS0,
    },
    Vector {
        _handler: UARTE1_SPIM1_SPIS1_TWIM1_TWIS1,
    },
    Vector {
        _handler: UARTE2_SPIM2_SPIS2_TWIM2_TWIS2,
    },
    Vector {
        _handler: UARTE3_SPIM3_SPIS3_TWIM3_TWIS3,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE0 },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: RTC0 },
    Vector { _handler: RTC1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EGU0 },
    Vector { _handler: EGU1 },
    Vector { _handler: EGU2 },
    Vector { _handler: EGU3 },
    Vector { _handler: EGU4 },
    Vector { _handler: EGU5 },
    Vector { _handler: PWM0 },
    Vector { _handler: PWM1 },
    Vector { _handler: PWM2 },
    Vector { _handler: PWM3 },
    Vector { _reserved: 0 },
    Vector { _handler: PDM },
    Vector { _reserved: 0 },
    Vector { _handler: I2S },
    Vector { _reserved: 0 },
    Vector { _handler: IPC },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: KMU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYPTOCELL,
    },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "3 - SPU"]
    SPU,
    #[doc = "5 - CLOCK_POWER"]
    CLOCK_POWER,
    #[doc = "8 - UARTE0_SPIM0_SPIS0_TWIM0_TWIS0"]
    UARTE0_SPIM0_SPIS0_TWIM0_TWIS0,
    #[doc = "9 - UARTE1_SPIM1_SPIS1_TWIM1_TWIS1"]
    UARTE1_SPIM1_SPIS1_TWIM1_TWIS1,
    #[doc = "10 - UARTE2_SPIM2_SPIS2_TWIM2_TWIS2"]
    UARTE2_SPIM2_SPIS2_TWIM2_TWIS2,
    #[doc = "11 - UARTE3_SPIM3_SPIS3_TWIM3_TWIS3"]
    UARTE3_SPIM3_SPIS3_TWIM3_TWIS3,
    #[doc = "13 - GPIOTE0"]
    GPIOTE0,
    #[doc = "14 - SAADC"]
    SAADC,
    #[doc = "15 - TIMER0"]
    TIMER0,
    #[doc = "16 - TIMER1"]
    TIMER1,
    #[doc = "17 - TIMER2"]
    TIMER2,
    #[doc = "20 - RTC0"]
    RTC0,
    #[doc = "21 - RTC1"]
    RTC1,
    #[doc = "24 - WDT"]
    WDT,
    #[doc = "27 - EGU0"]
    EGU0,
    #[doc = "28 - EGU1"]
    EGU1,
    #[doc = "29 - EGU2"]
    EGU2,
    #[doc = "30 - EGU3"]
    EGU3,
    #[doc = "31 - EGU4"]
    EGU4,
    #[doc = "32 - EGU5"]
    EGU5,
    #[doc = "33 - PWM0"]
    PWM0,
    #[doc = "34 - PWM1"]
    PWM1,
    #[doc = "35 - PWM2"]
    PWM2,
    #[doc = "36 - PWM3"]
    PWM3,
    #[doc = "38 - PDM"]
    PDM,
    #[doc = "40 - I2S"]
    I2S,
    #[doc = "42 - IPC"]
    IPC,
    #[doc = "44 - FPU"]
    FPU,
    #[doc = "49 - GPIOTE1"]
    GPIOTE1,
    #[doc = "57 - KMU"]
    KMU,
    #[doc = "64 - CRYPTOCELL"]
    CRYPTOCELL,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SPU => 3,
            Interrupt::CLOCK_POWER => 5,
            Interrupt::UARTE0_SPIM0_SPIS0_TWIM0_TWIS0 => 8,
            Interrupt::UARTE1_SPIM1_SPIS1_TWIM1_TWIS1 => 9,
            Interrupt::UARTE2_SPIM2_SPIS2_TWIM2_TWIS2 => 10,
            Interrupt::UARTE3_SPIM3_SPIS3_TWIM3_TWIS3 => 11,
            Interrupt::GPIOTE0 => 13,
            Interrupt::SAADC => 14,
            Interrupt::TIMER0 => 15,
            Interrupt::TIMER1 => 16,
            Interrupt::TIMER2 => 17,
            Interrupt::RTC0 => 20,
            Interrupt::RTC1 => 21,
            Interrupt::WDT => 24,
            Interrupt::EGU0 => 27,
            Interrupt::EGU1 => 28,
            Interrupt::EGU2 => 29,
            Interrupt::EGU3 => 30,
            Interrupt::EGU4 => 31,
            Interrupt::EGU5 => 32,
            Interrupt::PWM0 => 33,
            Interrupt::PWM1 => 34,
            Interrupt::PWM2 => 35,
            Interrupt::PWM3 => 36,
            Interrupt::PDM => 38,
            Interrupt::I2S => 40,
            Interrupt::IPC => 42,
            Interrupt::FPU => 44,
            Interrupt::GPIOTE1 => 49,
            Interrupt::KMU => 57,
            Interrupt::CRYPTOCELL => 64,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Factory Information Configuration Registers"]
pub struct FICR_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR_S {}
impl FICR_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ficr_s::RegisterBlock {
        16711680 as *const _
    }
}
impl Deref for FICR_S {
    type Target = ficr_s::RegisterBlock;
    fn deref(&self) -> &ficr_s::RegisterBlock {
        unsafe { &*FICR_S::ptr() }
    }
}
#[doc = "Factory Information Configuration Registers"]
pub mod ficr_s;
#[doc = "User information configuration registers User information configuration registers"]
pub struct UICR_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR_S {}
impl UICR_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uicr_s::RegisterBlock {
        16744448 as *const _
    }
}
impl Deref for UICR_S {
    type Target = uicr_s::RegisterBlock;
    fn deref(&self) -> &uicr_s::RegisterBlock {
        unsafe { &*UICR_S::ptr() }
    }
}
#[doc = "User information configuration registers User information configuration registers"]
pub mod uicr_s;
#[doc = "Trace and debug control"]
pub struct TAD_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAD_S {}
impl TAD_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tad_s::RegisterBlock {
        3758620672 as *const _
    }
}
impl Deref for TAD_S {
    type Target = tad_s::RegisterBlock;
    fn deref(&self) -> &tad_s::RegisterBlock {
        unsafe { &*TAD_S::ptr() }
    }
}
#[doc = "Trace and debug control"]
pub mod tad_s;
#[doc = "System protection unit"]
pub struct SPU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPU_S {}
impl SPU_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spu_s::RegisterBlock {
        1342189568 as *const _
    }
}
impl Deref for SPU_S {
    type Target = spu_s::RegisterBlock;
    fn deref(&self) -> &spu_s::RegisterBlock {
        unsafe { &*SPU_S::ptr() }
    }
}
#[doc = "System protection unit"]
pub mod spu_s;
#[doc = "Voltage regulators control 0"]
pub struct REGULATORS_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REGULATORS_NS {}
impl REGULATORS_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const regulators_ns::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for REGULATORS_NS {
    type Target = regulators_ns::RegisterBlock;
    fn deref(&self) -> &regulators_ns::RegisterBlock {
        unsafe { &*REGULATORS_NS::ptr() }
    }
}
#[doc = "Voltage regulators control 0"]
pub mod regulators_ns;
#[doc = "Voltage regulators control 1"]
pub struct REGULATORS_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REGULATORS_S {}
impl REGULATORS_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const regulators_ns::RegisterBlock {
        1342193664 as *const _
    }
}
impl Deref for REGULATORS_S {
    type Target = regulators_ns::RegisterBlock;
    fn deref(&self) -> &regulators_ns::RegisterBlock {
        unsafe { &*REGULATORS_S::ptr() }
    }
}
#[doc = "Clock management 0"]
pub struct CLOCK_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK_NS {}
impl CLOCK_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clock_ns::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for CLOCK_NS {
    type Target = clock_ns::RegisterBlock;
    fn deref(&self) -> &clock_ns::RegisterBlock {
        unsafe { &*CLOCK_NS::ptr() }
    }
}
#[doc = "Clock management 0"]
pub mod clock_ns;
#[doc = "Power control 0"]
pub struct POWER_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER_NS {}
impl POWER_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const power_ns::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for POWER_NS {
    type Target = power_ns::RegisterBlock;
    fn deref(&self) -> &power_ns::RegisterBlock {
        unsafe { &*POWER_NS::ptr() }
    }
}
#[doc = "Power control 0"]
pub mod power_ns;
#[doc = "Clock management 1"]
pub struct CLOCK_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK_S {}
impl CLOCK_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clock_ns::RegisterBlock {
        1342197760 as *const _
    }
}
impl Deref for CLOCK_S {
    type Target = clock_ns::RegisterBlock;
    fn deref(&self) -> &clock_ns::RegisterBlock {
        unsafe { &*CLOCK_S::ptr() }
    }
}
#[doc = "Power control 1"]
pub struct POWER_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER_S {}
impl POWER_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const power_ns::RegisterBlock {
        1342197760 as *const _
    }
}
impl Deref for POWER_S {
    type Target = power_ns::RegisterBlock;
    fn deref(&self) -> &power_ns::RegisterBlock {
        unsafe { &*POWER_S::ptr() }
    }
}
#[doc = "Control access port"]
pub struct CTRL_AP_PERI_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRL_AP_PERI_S {}
impl CTRL_AP_PERI_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ctrl_ap_peri_s::RegisterBlock {
        1342201856 as *const _
    }
}
impl Deref for CTRL_AP_PERI_S {
    type Target = ctrl_ap_peri_s::RegisterBlock;
    fn deref(&self) -> &ctrl_ap_peri_s::RegisterBlock {
        unsafe { &*CTRL_AP_PERI_S::ptr() }
    }
}
#[doc = "Control access port"]
pub mod ctrl_ap_peri_s;
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub struct SPIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0_NS {}
impl SPIM0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SPIM0_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM0_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 0"]
pub mod spim0_ns;
#[doc = "SPI Slave 0"]
pub struct SPIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0_NS {}
impl SPIS0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SPIS0_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS0_NS::ptr() }
    }
}
#[doc = "SPI Slave 0"]
pub mod spis0_ns;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub struct TWIM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0_NS {}
impl TWIM0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TWIM0_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 0"]
pub mod twim0_ns;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub struct TWIS0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0_NS {}
impl TWIS0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TWIS0_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS0_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 0"]
pub mod twis0_ns;
#[doc = "UART with EasyDMA 0"]
pub struct UARTE0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0_NS {}
impl UARTE0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for UARTE0_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE0_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 0"]
pub mod uarte0_ns;
#[doc = "Serial Peripheral Interface Master with EasyDMA 1"]
pub struct SPIM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0_S {}
impl SPIM0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for SPIM0_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM0_S::ptr() }
    }
}
#[doc = "SPI Slave 1"]
pub struct SPIS0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0_S {}
impl SPIS0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for SPIS0_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS0_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 1"]
pub struct TWIM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0_S {}
impl TWIM0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for TWIM0_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM0_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 1"]
pub struct TWIS0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0_S {}
impl TWIS0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for TWIS0_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS0_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 1"]
pub struct UARTE0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0_S {}
impl UARTE0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1342210048 as *const _
    }
}
impl Deref for UARTE0_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE0_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 2"]
pub struct SPIM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1_NS {}
impl SPIM1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for SPIM1_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM1_NS::ptr() }
    }
}
#[doc = "SPI Slave 2"]
pub struct SPIS1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1_NS {}
impl SPIS1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for SPIS1_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS1_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 2"]
pub struct TWIM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1_NS {}
impl TWIM1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for TWIM1_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM1_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 2"]
pub struct TWIS1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1_NS {}
impl TWIS1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for TWIS1_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS1_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 2"]
pub struct UARTE1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE1_NS {}
impl UARTE1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for UARTE1_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE1_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 3"]
pub struct SPIM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1_S {}
impl SPIM1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for SPIM1_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM1_S::ptr() }
    }
}
#[doc = "SPI Slave 3"]
pub struct SPIS1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1_S {}
impl SPIS1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for SPIS1_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS1_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 3"]
pub struct TWIM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM1_S {}
impl TWIM1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for TWIM1_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM1_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 3"]
pub struct TWIS1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS1_S {}
impl TWIS1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for TWIS1_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS1_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 3"]
pub struct UARTE1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE1_S {}
impl UARTE1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1342214144 as *const _
    }
}
impl Deref for UARTE1_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE1_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 4"]
pub struct SPIM2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2_NS {}
impl SPIM2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for SPIM2_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM2_NS::ptr() }
    }
}
#[doc = "SPI Slave 4"]
pub struct SPIS2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS2_NS {}
impl SPIS2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for SPIS2_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS2_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 4"]
pub struct TWIM2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM2_NS {}
impl TWIM2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for TWIM2_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM2_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 4"]
pub struct TWIS2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS2_NS {}
impl TWIS2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for TWIS2_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS2_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 4"]
pub struct UARTE2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE2_NS {}
impl UARTE2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for UARTE2_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE2_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 5"]
pub struct SPIM2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2_S {}
impl SPIM2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1342218240 as *const _
    }
}
impl Deref for SPIM2_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM2_S::ptr() }
    }
}
#[doc = "SPI Slave 5"]
pub struct SPIS2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS2_S {}
impl SPIS2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1342218240 as *const _
    }
}
impl Deref for SPIS2_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS2_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 5"]
pub struct TWIM2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM2_S {}
impl TWIM2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1342218240 as *const _
    }
}
impl Deref for TWIM2_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM2_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 5"]
pub struct TWIS2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS2_S {}
impl TWIS2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1342218240 as *const _
    }
}
impl Deref for TWIS2_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS2_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 5"]
pub struct UARTE2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE2_S {}
impl UARTE2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1342218240 as *const _
    }
}
impl Deref for UARTE2_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE2_S::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 6"]
pub struct SPIM3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM3_NS {}
impl SPIM3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for SPIM3_NS {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM3_NS::ptr() }
    }
}
#[doc = "SPI Slave 6"]
pub struct SPIS3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS3_NS {}
impl SPIS3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for SPIS3_NS {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS3_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 6"]
pub struct TWIM3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM3_NS {}
impl TWIM3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for TWIM3_NS {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM3_NS::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 6"]
pub struct TWIS3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS3_NS {}
impl TWIS3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for TWIS3_NS {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS3_NS::ptr() }
    }
}
#[doc = "UART with EasyDMA 6"]
pub struct UARTE3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE3_NS {}
impl UARTE3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for UARTE3_NS {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE3_NS::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA 7"]
pub struct SPIM3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM3_S {}
impl SPIM3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0_ns::RegisterBlock {
        1342222336 as *const _
    }
}
impl Deref for SPIM3_S {
    type Target = spim0_ns::RegisterBlock;
    fn deref(&self) -> &spim0_ns::RegisterBlock {
        unsafe { &*SPIM3_S::ptr() }
    }
}
#[doc = "SPI Slave 7"]
pub struct SPIS3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS3_S {}
impl SPIS3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0_ns::RegisterBlock {
        1342222336 as *const _
    }
}
impl Deref for SPIS3_S {
    type Target = spis0_ns::RegisterBlock;
    fn deref(&self) -> &spis0_ns::RegisterBlock {
        unsafe { &*SPIS3_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA 7"]
pub struct TWIM3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM3_S {}
impl TWIM3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0_ns::RegisterBlock {
        1342222336 as *const _
    }
}
impl Deref for TWIM3_S {
    type Target = twim0_ns::RegisterBlock;
    fn deref(&self) -> &twim0_ns::RegisterBlock {
        unsafe { &*TWIM3_S::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA 7"]
pub struct TWIS3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS3_S {}
impl TWIS3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0_ns::RegisterBlock {
        1342222336 as *const _
    }
}
impl Deref for TWIS3_S {
    type Target = twis0_ns::RegisterBlock;
    fn deref(&self) -> &twis0_ns::RegisterBlock {
        unsafe { &*TWIS3_S::ptr() }
    }
}
#[doc = "UART with EasyDMA 7"]
pub struct UARTE3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE3_S {}
impl UARTE3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0_ns::RegisterBlock {
        1342222336 as *const _
    }
}
impl Deref for UARTE3_S {
    type Target = uarte0_ns::RegisterBlock;
    fn deref(&self) -> &uarte0_ns::RegisterBlock {
        unsafe { &*UARTE3_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 0"]
pub struct GPIOTE0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE0_S {}
impl GPIOTE0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiote0_s::RegisterBlock {
        1342230528 as *const _
    }
}
impl Deref for GPIOTE0_S {
    type Target = gpiote0_s::RegisterBlock;
    fn deref(&self) -> &gpiote0_s::RegisterBlock {
        unsafe { &*GPIOTE0_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 0"]
pub mod gpiote0_s;
#[doc = "Analog to Digital Converter 0"]
pub struct SAADC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC_NS {}
impl SAADC_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const saadc_ns::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for SAADC_NS {
    type Target = saadc_ns::RegisterBlock;
    fn deref(&self) -> &saadc_ns::RegisterBlock {
        unsafe { &*SAADC_NS::ptr() }
    }
}
#[doc = "Analog to Digital Converter 0"]
pub mod saadc_ns;
#[doc = "Analog to Digital Converter 1"]
pub struct SAADC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC_S {}
impl SAADC_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const saadc_ns::RegisterBlock {
        1342234624 as *const _
    }
}
impl Deref for SAADC_S {
    type Target = saadc_ns::RegisterBlock;
    fn deref(&self) -> &saadc_ns::RegisterBlock {
        unsafe { &*SAADC_S::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub struct TIMER0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_NS {}
impl TIMER0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for TIMER0_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER0_NS::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0_ns;
#[doc = "Timer/Counter 1"]
pub struct TIMER0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0_S {}
impl TIMER0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1342238720 as *const _
    }
}
impl Deref for TIMER0_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER0_S::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_NS {}
impl TIMER1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TIMER1_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER1_NS::ptr() }
    }
}
#[doc = "Timer/Counter 3"]
pub struct TIMER1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1_S {}
impl TIMER1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1342242816 as *const _
    }
}
impl Deref for TIMER1_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER1_S::ptr() }
    }
}
#[doc = "Timer/Counter 4"]
pub struct TIMER2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2_NS {}
impl TIMER2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for TIMER2_NS {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER2_NS::ptr() }
    }
}
#[doc = "Timer/Counter 5"]
pub struct TIMER2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2_S {}
impl TIMER2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0_ns::RegisterBlock {
        1342246912 as *const _
    }
}
impl Deref for TIMER2_S {
    type Target = timer0_ns::RegisterBlock;
    fn deref(&self) -> &timer0_ns::RegisterBlock {
        unsafe { &*TIMER2_S::ptr() }
    }
}
#[doc = "Real-time counter 0"]
pub struct RTC0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0_NS {}
impl RTC0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0_ns::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for RTC0_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &rtc0_ns::RegisterBlock {
        unsafe { &*RTC0_NS::ptr() }
    }
}
#[doc = "Real-time counter 0"]
pub mod rtc0_ns;
#[doc = "Real-time counter 1"]
pub struct RTC0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0_S {}
impl RTC0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0_ns::RegisterBlock {
        1342259200 as *const _
    }
}
impl Deref for RTC0_S {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &rtc0_ns::RegisterBlock {
        unsafe { &*RTC0_S::ptr() }
    }
}
#[doc = "Real-time counter 2"]
pub struct RTC1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1_NS {}
impl RTC1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0_ns::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for RTC1_NS {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &rtc0_ns::RegisterBlock {
        unsafe { &*RTC1_NS::ptr() }
    }
}
#[doc = "Real-time counter 3"]
pub struct RTC1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1_S {}
impl RTC1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0_ns::RegisterBlock {
        1342263296 as *const _
    }
}
impl Deref for RTC1_S {
    type Target = rtc0_ns::RegisterBlock;
    fn deref(&self) -> &rtc0_ns::RegisterBlock {
        unsafe { &*RTC1_S::ptr() }
    }
}
#[doc = "Distributed Programmable Peripheral Interconnect Controller 0"]
pub struct DPPIC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPPIC_NS {}
impl DPPIC_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dppic_ns::RegisterBlock {
        1073836032 as *const _
    }
}
impl Deref for DPPIC_NS {
    type Target = dppic_ns::RegisterBlock;
    fn deref(&self) -> &dppic_ns::RegisterBlock {
        unsafe { &*DPPIC_NS::ptr() }
    }
}
#[doc = "Distributed Programmable Peripheral Interconnect Controller 0"]
pub mod dppic_ns;
#[doc = "Distributed Programmable Peripheral Interconnect Controller 1"]
pub struct DPPIC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPPIC_S {}
impl DPPIC_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dppic_ns::RegisterBlock {
        1342271488 as *const _
    }
}
impl Deref for DPPIC_S {
    type Target = dppic_ns::RegisterBlock;
    fn deref(&self) -> &dppic_ns::RegisterBlock {
        unsafe { &*DPPIC_S::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub struct WDT_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_NS {}
impl WDT_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt_ns::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for WDT_NS {
    type Target = wdt_ns::RegisterBlock;
    fn deref(&self) -> &wdt_ns::RegisterBlock {
        unsafe { &*WDT_NS::ptr() }
    }
}
#[doc = "Watchdog Timer 0"]
pub mod wdt_ns;
#[doc = "Watchdog Timer 1"]
pub struct WDT_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT_S {}
impl WDT_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt_ns::RegisterBlock {
        1342275584 as *const _
    }
}
impl Deref for WDT_S {
    type Target = wdt_ns::RegisterBlock;
    fn deref(&self) -> &wdt_ns::RegisterBlock {
        unsafe { &*WDT_S::ptr() }
    }
}
#[doc = "Event Generator Unit 0"]
pub struct EGU0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0_NS {}
impl EGU0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073852416 as *const _
    }
}
impl Deref for EGU0_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU0_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 0"]
pub mod egu0_ns;
#[doc = "Event Generator Unit 1"]
pub struct EGU0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0_S {}
impl EGU0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342287872 as *const _
    }
}
impl Deref for EGU0_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU0_S::ptr() }
    }
}
#[doc = "Event Generator Unit 2"]
pub struct EGU1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1_NS {}
impl EGU1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for EGU1_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU1_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 3"]
pub struct EGU1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1_S {}
impl EGU1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342291968 as *const _
    }
}
impl Deref for EGU1_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU1_S::ptr() }
    }
}
#[doc = "Event Generator Unit 4"]
pub struct EGU2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2_NS {}
impl EGU2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073860608 as *const _
    }
}
impl Deref for EGU2_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU2_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 5"]
pub struct EGU2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU2_S {}
impl EGU2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342296064 as *const _
    }
}
impl Deref for EGU2_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU2_S::ptr() }
    }
}
#[doc = "Event Generator Unit 6"]
pub struct EGU3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3_NS {}
impl EGU3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for EGU3_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU3_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 7"]
pub struct EGU3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU3_S {}
impl EGU3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342300160 as *const _
    }
}
impl Deref for EGU3_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU3_S::ptr() }
    }
}
#[doc = "Event Generator Unit 8"]
pub struct EGU4_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4_NS {}
impl EGU4_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for EGU4_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU4_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 9"]
pub struct EGU4_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU4_S {}
impl EGU4_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342304256 as *const _
    }
}
impl Deref for EGU4_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU4_S::ptr() }
    }
}
#[doc = "Event Generator Unit 10"]
pub struct EGU5_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5_NS {}
impl EGU5_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for EGU5_NS {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU5_NS::ptr() }
    }
}
#[doc = "Event Generator Unit 11"]
pub struct EGU5_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU5_S {}
impl EGU5_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0_ns::RegisterBlock {
        1342308352 as *const _
    }
}
impl Deref for EGU5_S {
    type Target = egu0_ns::RegisterBlock;
    fn deref(&self) -> &egu0_ns::RegisterBlock {
        unsafe { &*EGU5_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 0"]
pub struct PWM0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0_NS {}
impl PWM0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for PWM0_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM0_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 0"]
pub mod pwm0_ns;
#[doc = "Pulse width modulation unit 1"]
pub struct PWM0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0_S {}
impl PWM0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1342312448 as *const _
    }
}
impl Deref for PWM0_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM0_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 2"]
pub struct PWM1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1_NS {}
impl PWM1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for PWM1_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM1_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 3"]
pub struct PWM1_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1_S {}
impl PWM1_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1342316544 as *const _
    }
}
impl Deref for PWM1_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM1_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 4"]
pub struct PWM2_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2_NS {}
impl PWM2_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for PWM2_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM2_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 5"]
pub struct PWM2_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM2_S {}
impl PWM2_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1342320640 as *const _
    }
}
impl Deref for PWM2_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM2_S::ptr() }
    }
}
#[doc = "Pulse width modulation unit 6"]
pub struct PWM3_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM3_NS {}
impl PWM3_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1073889280 as *const _
    }
}
impl Deref for PWM3_NS {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM3_NS::ptr() }
    }
}
#[doc = "Pulse width modulation unit 7"]
pub struct PWM3_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM3_S {}
impl PWM3_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0_ns::RegisterBlock {
        1342324736 as *const _
    }
}
impl Deref for PWM3_S {
    type Target = pwm0_ns::RegisterBlock;
    fn deref(&self) -> &pwm0_ns::RegisterBlock {
        unsafe { &*PWM3_S::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub struct PDM_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM_NS {}
impl PDM_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm_ns::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for PDM_NS {
    type Target = pdm_ns::RegisterBlock;
    fn deref(&self) -> &pdm_ns::RegisterBlock {
        unsafe { &*PDM_NS::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 0"]
pub mod pdm_ns;
#[doc = "Pulse Density Modulation (Digital Microphone) Interface 1"]
pub struct PDM_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM_S {}
impl PDM_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm_ns::RegisterBlock {
        1342332928 as *const _
    }
}
impl Deref for PDM_S {
    type Target = pdm_ns::RegisterBlock;
    fn deref(&self) -> &pdm_ns::RegisterBlock {
        unsafe { &*PDM_S::ptr() }
    }
}
#[doc = "Inter-IC Sound 0"]
pub struct I2S_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S_NS {}
impl I2S_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s_ns::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for I2S_NS {
    type Target = i2s_ns::RegisterBlock;
    fn deref(&self) -> &i2s_ns::RegisterBlock {
        unsafe { &*I2S_NS::ptr() }
    }
}
#[doc = "Inter-IC Sound 0"]
pub mod i2s_ns;
#[doc = "Inter-IC Sound 1"]
pub struct I2S_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S_S {}
impl I2S_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s_ns::RegisterBlock {
        1342341120 as *const _
    }
}
impl Deref for I2S_S {
    type Target = i2s_ns::RegisterBlock;
    fn deref(&self) -> &i2s_ns::RegisterBlock {
        unsafe { &*I2S_S::ptr() }
    }
}
#[doc = "Inter Processor Communication 0"]
pub struct IPC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC_NS {}
impl IPC_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ipc_ns::RegisterBlock {
        1073913856 as *const _
    }
}
impl Deref for IPC_NS {
    type Target = ipc_ns::RegisterBlock;
    fn deref(&self) -> &ipc_ns::RegisterBlock {
        unsafe { &*IPC_NS::ptr() }
    }
}
#[doc = "Inter Processor Communication 0"]
pub mod ipc_ns;
#[doc = "Inter Processor Communication 1"]
pub struct IPC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPC_S {}
impl IPC_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ipc_ns::RegisterBlock {
        1342349312 as *const _
    }
}
impl Deref for IPC_S {
    type Target = ipc_ns::RegisterBlock;
    fn deref(&self) -> &ipc_ns::RegisterBlock {
        unsafe { &*IPC_S::ptr() }
    }
}
#[doc = "FPU 0"]
pub struct FPU_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_NS {}
impl FPU_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu_ns::RegisterBlock {
        1073922048 as *const _
    }
}
impl Deref for FPU_NS {
    type Target = fpu_ns::RegisterBlock;
    fn deref(&self) -> &fpu_ns::RegisterBlock {
        unsafe { &*FPU_NS::ptr() }
    }
}
#[doc = "FPU 0"]
pub mod fpu_ns;
#[doc = "FPU 1"]
pub struct FPU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_S {}
impl FPU_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu_ns::RegisterBlock {
        1342357504 as *const _
    }
}
impl Deref for FPU_S {
    type Target = fpu_ns::RegisterBlock;
    fn deref(&self) -> &fpu_ns::RegisterBlock {
        unsafe { &*FPU_S::ptr() }
    }
}
#[doc = "GPIO Tasks and Events 1"]
pub struct GPIOTE1_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE1_NS {}
impl GPIOTE1_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiote0_s::RegisterBlock {
        1073942528 as *const _
    }
}
impl Deref for GPIOTE1_NS {
    type Target = gpiote0_s::RegisterBlock;
    fn deref(&self) -> &gpiote0_s::RegisterBlock {
        unsafe { &*GPIOTE1_NS::ptr() }
    }
}
#[doc = "Key management unit 0"]
pub struct KMU_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KMU_NS {}
impl KMU_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const kmu_ns::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for KMU_NS {
    type Target = kmu_ns::RegisterBlock;
    fn deref(&self) -> &kmu_ns::RegisterBlock {
        unsafe { &*KMU_NS::ptr() }
    }
}
#[doc = "Key management unit 0"]
pub mod kmu_ns;
#[doc = "Non-volatile memory controller 0"]
pub struct NVMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC_NS {}
impl NVMC_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmc_ns::RegisterBlock {
        1073975296 as *const _
    }
}
impl Deref for NVMC_NS {
    type Target = nvmc_ns::RegisterBlock;
    fn deref(&self) -> &nvmc_ns::RegisterBlock {
        unsafe { &*NVMC_NS::ptr() }
    }
}
#[doc = "Non-volatile memory controller 0"]
pub mod nvmc_ns;
#[doc = "Key management unit 1"]
pub struct KMU_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for KMU_S {}
impl KMU_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const kmu_ns::RegisterBlock {
        1342410752 as *const _
    }
}
impl Deref for KMU_S {
    type Target = kmu_ns::RegisterBlock;
    fn deref(&self) -> &kmu_ns::RegisterBlock {
        unsafe { &*KMU_S::ptr() }
    }
}
#[doc = "Non-volatile memory controller 1"]
pub struct NVMC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC_S {}
impl NVMC_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmc_ns::RegisterBlock {
        1342410752 as *const _
    }
}
impl Deref for NVMC_S {
    type Target = nvmc_ns::RegisterBlock;
    fn deref(&self) -> &nvmc_ns::RegisterBlock {
        unsafe { &*NVMC_S::ptr() }
    }
}
#[doc = "Volatile Memory controller 0"]
pub struct VMC_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VMC_NS {}
impl VMC_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vmc_ns::RegisterBlock {
        1073979392 as *const _
    }
}
impl Deref for VMC_NS {
    type Target = vmc_ns::RegisterBlock;
    fn deref(&self) -> &vmc_ns::RegisterBlock {
        unsafe { &*VMC_NS::ptr() }
    }
}
#[doc = "Volatile Memory controller 0"]
pub mod vmc_ns;
#[doc = "Volatile Memory controller 1"]
pub struct VMC_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VMC_S {}
impl VMC_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const vmc_ns::RegisterBlock {
        1342414848 as *const _
    }
}
impl Deref for VMC_S {
    type Target = vmc_ns::RegisterBlock;
    fn deref(&self) -> &vmc_ns::RegisterBlock {
        unsafe { &*VMC_S::ptr() }
    }
}
#[doc = "ARM TrustZone CryptoCell register interface"]
pub struct CRYPTOCELL_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTOCELL_S {}
impl CRYPTOCELL_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cryptocell_s::RegisterBlock {
        1350828032 as *const _
    }
}
impl Deref for CRYPTOCELL_S {
    type Target = cryptocell_s::RegisterBlock;
    fn deref(&self) -> &cryptocell_s::RegisterBlock {
        unsafe { &*CRYPTOCELL_S::ptr() }
    }
}
#[doc = "ARM TrustZone CryptoCell register interface"]
pub mod cryptocell_s;
#[doc = "GPIO Port 0"]
pub struct P0_NS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0_NS {}
impl P0_NS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const p0_ns::RegisterBlock {
        1082402048 as *const _
    }
}
impl Deref for P0_NS {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &p0_ns::RegisterBlock {
        unsafe { &*P0_NS::ptr() }
    }
}
#[doc = "GPIO Port 0"]
pub mod p0_ns;
#[doc = "GPIO Port 1"]
pub struct P0_S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0_S {}
impl P0_S {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const p0_ns::RegisterBlock {
        1350837504 as *const _
    }
}
impl Deref for P0_S {
    type Target = p0_ns::RegisterBlock;
    fn deref(&self) -> &p0_ns::RegisterBlock {
        unsafe { &*P0_S::ptr() }
    }
}
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FICR_S"]
    pub FICR_S: FICR_S,
    #[doc = "UICR_S"]
    pub UICR_S: UICR_S,
    #[doc = "TAD_S"]
    pub TAD_S: TAD_S,
    #[doc = "SPU_S"]
    pub SPU_S: SPU_S,
    #[doc = "REGULATORS_NS"]
    pub REGULATORS_NS: REGULATORS_NS,
    #[doc = "REGULATORS_S"]
    pub REGULATORS_S: REGULATORS_S,
    #[doc = "CLOCK_NS"]
    pub CLOCK_NS: CLOCK_NS,
    #[doc = "POWER_NS"]
    pub POWER_NS: POWER_NS,
    #[doc = "CLOCK_S"]
    pub CLOCK_S: CLOCK_S,
    #[doc = "POWER_S"]
    pub POWER_S: POWER_S,
    #[doc = "CTRL_AP_PERI_S"]
    pub CTRL_AP_PERI_S: CTRL_AP_PERI_S,
    #[doc = "SPIM0_NS"]
    pub SPIM0_NS: SPIM0_NS,
    #[doc = "SPIS0_NS"]
    pub SPIS0_NS: SPIS0_NS,
    #[doc = "TWIM0_NS"]
    pub TWIM0_NS: TWIM0_NS,
    #[doc = "TWIS0_NS"]
    pub TWIS0_NS: TWIS0_NS,
    #[doc = "UARTE0_NS"]
    pub UARTE0_NS: UARTE0_NS,
    #[doc = "SPIM0_S"]
    pub SPIM0_S: SPIM0_S,
    #[doc = "SPIS0_S"]
    pub SPIS0_S: SPIS0_S,
    #[doc = "TWIM0_S"]
    pub TWIM0_S: TWIM0_S,
    #[doc = "TWIS0_S"]
    pub TWIS0_S: TWIS0_S,
    #[doc = "UARTE0_S"]
    pub UARTE0_S: UARTE0_S,
    #[doc = "SPIM1_NS"]
    pub SPIM1_NS: SPIM1_NS,
    #[doc = "SPIS1_NS"]
    pub SPIS1_NS: SPIS1_NS,
    #[doc = "TWIM1_NS"]
    pub TWIM1_NS: TWIM1_NS,
    #[doc = "TWIS1_NS"]
    pub TWIS1_NS: TWIS1_NS,
    #[doc = "UARTE1_NS"]
    pub UARTE1_NS: UARTE1_NS,
    #[doc = "SPIM1_S"]
    pub SPIM1_S: SPIM1_S,
    #[doc = "SPIS1_S"]
    pub SPIS1_S: SPIS1_S,
    #[doc = "TWIM1_S"]
    pub TWIM1_S: TWIM1_S,
    #[doc = "TWIS1_S"]
    pub TWIS1_S: TWIS1_S,
    #[doc = "UARTE1_S"]
    pub UARTE1_S: UARTE1_S,
    #[doc = "SPIM2_NS"]
    pub SPIM2_NS: SPIM2_NS,
    #[doc = "SPIS2_NS"]
    pub SPIS2_NS: SPIS2_NS,
    #[doc = "TWIM2_NS"]
    pub TWIM2_NS: TWIM2_NS,
    #[doc = "TWIS2_NS"]
    pub TWIS2_NS: TWIS2_NS,
    #[doc = "UARTE2_NS"]
    pub UARTE2_NS: UARTE2_NS,
    #[doc = "SPIM2_S"]
    pub SPIM2_S: SPIM2_S,
    #[doc = "SPIS2_S"]
    pub SPIS2_S: SPIS2_S,
    #[doc = "TWIM2_S"]
    pub TWIM2_S: TWIM2_S,
    #[doc = "TWIS2_S"]
    pub TWIS2_S: TWIS2_S,
    #[doc = "UARTE2_S"]
    pub UARTE2_S: UARTE2_S,
    #[doc = "SPIM3_NS"]
    pub SPIM3_NS: SPIM3_NS,
    #[doc = "SPIS3_NS"]
    pub SPIS3_NS: SPIS3_NS,
    #[doc = "TWIM3_NS"]
    pub TWIM3_NS: TWIM3_NS,
    #[doc = "TWIS3_NS"]
    pub TWIS3_NS: TWIS3_NS,
    #[doc = "UARTE3_NS"]
    pub UARTE3_NS: UARTE3_NS,
    #[doc = "SPIM3_S"]
    pub SPIM3_S: SPIM3_S,
    #[doc = "SPIS3_S"]
    pub SPIS3_S: SPIS3_S,
    #[doc = "TWIM3_S"]
    pub TWIM3_S: TWIM3_S,
    #[doc = "TWIS3_S"]
    pub TWIS3_S: TWIS3_S,
    #[doc = "UARTE3_S"]
    pub UARTE3_S: UARTE3_S,
    #[doc = "GPIOTE0_S"]
    pub GPIOTE0_S: GPIOTE0_S,
    #[doc = "SAADC_NS"]
    pub SAADC_NS: SAADC_NS,
    #[doc = "SAADC_S"]
    pub SAADC_S: SAADC_S,
    #[doc = "TIMER0_NS"]
    pub TIMER0_NS: TIMER0_NS,
    #[doc = "TIMER0_S"]
    pub TIMER0_S: TIMER0_S,
    #[doc = "TIMER1_NS"]
    pub TIMER1_NS: TIMER1_NS,
    #[doc = "TIMER1_S"]
    pub TIMER1_S: TIMER1_S,
    #[doc = "TIMER2_NS"]
    pub TIMER2_NS: TIMER2_NS,
    #[doc = "TIMER2_S"]
    pub TIMER2_S: TIMER2_S,
    #[doc = "RTC0_NS"]
    pub RTC0_NS: RTC0_NS,
    #[doc = "RTC0_S"]
    pub RTC0_S: RTC0_S,
    #[doc = "RTC1_NS"]
    pub RTC1_NS: RTC1_NS,
    #[doc = "RTC1_S"]
    pub RTC1_S: RTC1_S,
    #[doc = "DPPIC_NS"]
    pub DPPIC_NS: DPPIC_NS,
    #[doc = "DPPIC_S"]
    pub DPPIC_S: DPPIC_S,
    #[doc = "WDT_NS"]
    pub WDT_NS: WDT_NS,
    #[doc = "WDT_S"]
    pub WDT_S: WDT_S,
    #[doc = "EGU0_NS"]
    pub EGU0_NS: EGU0_NS,
    #[doc = "EGU0_S"]
    pub EGU0_S: EGU0_S,
    #[doc = "EGU1_NS"]
    pub EGU1_NS: EGU1_NS,
    #[doc = "EGU1_S"]
    pub EGU1_S: EGU1_S,
    #[doc = "EGU2_NS"]
    pub EGU2_NS: EGU2_NS,
    #[doc = "EGU2_S"]
    pub EGU2_S: EGU2_S,
    #[doc = "EGU3_NS"]
    pub EGU3_NS: EGU3_NS,
    #[doc = "EGU3_S"]
    pub EGU3_S: EGU3_S,
    #[doc = "EGU4_NS"]
    pub EGU4_NS: EGU4_NS,
    #[doc = "EGU4_S"]
    pub EGU4_S: EGU4_S,
    #[doc = "EGU5_NS"]
    pub EGU5_NS: EGU5_NS,
    #[doc = "EGU5_S"]
    pub EGU5_S: EGU5_S,
    #[doc = "PWM0_NS"]
    pub PWM0_NS: PWM0_NS,
    #[doc = "PWM0_S"]
    pub PWM0_S: PWM0_S,
    #[doc = "PWM1_NS"]
    pub PWM1_NS: PWM1_NS,
    #[doc = "PWM1_S"]
    pub PWM1_S: PWM1_S,
    #[doc = "PWM2_NS"]
    pub PWM2_NS: PWM2_NS,
    #[doc = "PWM2_S"]
    pub PWM2_S: PWM2_S,
    #[doc = "PWM3_NS"]
    pub PWM3_NS: PWM3_NS,
    #[doc = "PWM3_S"]
    pub PWM3_S: PWM3_S,
    #[doc = "PDM_NS"]
    pub PDM_NS: PDM_NS,
    #[doc = "PDM_S"]
    pub PDM_S: PDM_S,
    #[doc = "I2S_NS"]
    pub I2S_NS: I2S_NS,
    #[doc = "I2S_S"]
    pub I2S_S: I2S_S,
    #[doc = "IPC_NS"]
    pub IPC_NS: IPC_NS,
    #[doc = "IPC_S"]
    pub IPC_S: IPC_S,
    #[doc = "FPU_NS"]
    pub FPU_NS: FPU_NS,
    #[doc = "FPU_S"]
    pub FPU_S: FPU_S,
    #[doc = "GPIOTE1_NS"]
    pub GPIOTE1_NS: GPIOTE1_NS,
    #[doc = "KMU_NS"]
    pub KMU_NS: KMU_NS,
    #[doc = "NVMC_NS"]
    pub NVMC_NS: NVMC_NS,
    #[doc = "KMU_S"]
    pub KMU_S: KMU_S,
    #[doc = "NVMC_S"]
    pub NVMC_S: NVMC_S,
    #[doc = "VMC_NS"]
    pub VMC_NS: VMC_NS,
    #[doc = "VMC_S"]
    pub VMC_S: VMC_S,
    #[doc = "CRYPTOCELL_S"]
    pub CRYPTOCELL_S: CRYPTOCELL_S,
    #[doc = "P0_NS"]
    pub P0_NS: P0_NS,
    #[doc = "P0_S"]
    pub P0_S: P0_S,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FICR_S: FICR_S {
                _marker: PhantomData,
            },
            UICR_S: UICR_S {
                _marker: PhantomData,
            },
            TAD_S: TAD_S {
                _marker: PhantomData,
            },
            SPU_S: SPU_S {
                _marker: PhantomData,
            },
            REGULATORS_NS: REGULATORS_NS {
                _marker: PhantomData,
            },
            REGULATORS_S: REGULATORS_S {
                _marker: PhantomData,
            },
            CLOCK_NS: CLOCK_NS {
                _marker: PhantomData,
            },
            POWER_NS: POWER_NS {
                _marker: PhantomData,
            },
            CLOCK_S: CLOCK_S {
                _marker: PhantomData,
            },
            POWER_S: POWER_S {
                _marker: PhantomData,
            },
            CTRL_AP_PERI_S: CTRL_AP_PERI_S {
                _marker: PhantomData,
            },
            SPIM0_NS: SPIM0_NS {
                _marker: PhantomData,
            },
            SPIS0_NS: SPIS0_NS {
                _marker: PhantomData,
            },
            TWIM0_NS: TWIM0_NS {
                _marker: PhantomData,
            },
            TWIS0_NS: TWIS0_NS {
                _marker: PhantomData,
            },
            UARTE0_NS: UARTE0_NS {
                _marker: PhantomData,
            },
            SPIM0_S: SPIM0_S {
                _marker: PhantomData,
            },
            SPIS0_S: SPIS0_S {
                _marker: PhantomData,
            },
            TWIM0_S: TWIM0_S {
                _marker: PhantomData,
            },
            TWIS0_S: TWIS0_S {
                _marker: PhantomData,
            },
            UARTE0_S: UARTE0_S {
                _marker: PhantomData,
            },
            SPIM1_NS: SPIM1_NS {
                _marker: PhantomData,
            },
            SPIS1_NS: SPIS1_NS {
                _marker: PhantomData,
            },
            TWIM1_NS: TWIM1_NS {
                _marker: PhantomData,
            },
            TWIS1_NS: TWIS1_NS {
                _marker: PhantomData,
            },
            UARTE1_NS: UARTE1_NS {
                _marker: PhantomData,
            },
            SPIM1_S: SPIM1_S {
                _marker: PhantomData,
            },
            SPIS1_S: SPIS1_S {
                _marker: PhantomData,
            },
            TWIM1_S: TWIM1_S {
                _marker: PhantomData,
            },
            TWIS1_S: TWIS1_S {
                _marker: PhantomData,
            },
            UARTE1_S: UARTE1_S {
                _marker: PhantomData,
            },
            SPIM2_NS: SPIM2_NS {
                _marker: PhantomData,
            },
            SPIS2_NS: SPIS2_NS {
                _marker: PhantomData,
            },
            TWIM2_NS: TWIM2_NS {
                _marker: PhantomData,
            },
            TWIS2_NS: TWIS2_NS {
                _marker: PhantomData,
            },
            UARTE2_NS: UARTE2_NS {
                _marker: PhantomData,
            },
            SPIM2_S: SPIM2_S {
                _marker: PhantomData,
            },
            SPIS2_S: SPIS2_S {
                _marker: PhantomData,
            },
            TWIM2_S: TWIM2_S {
                _marker: PhantomData,
            },
            TWIS2_S: TWIS2_S {
                _marker: PhantomData,
            },
            UARTE2_S: UARTE2_S {
                _marker: PhantomData,
            },
            SPIM3_NS: SPIM3_NS {
                _marker: PhantomData,
            },
            SPIS3_NS: SPIS3_NS {
                _marker: PhantomData,
            },
            TWIM3_NS: TWIM3_NS {
                _marker: PhantomData,
            },
            TWIS3_NS: TWIS3_NS {
                _marker: PhantomData,
            },
            UARTE3_NS: UARTE3_NS {
                _marker: PhantomData,
            },
            SPIM3_S: SPIM3_S {
                _marker: PhantomData,
            },
            SPIS3_S: SPIS3_S {
                _marker: PhantomData,
            },
            TWIM3_S: TWIM3_S {
                _marker: PhantomData,
            },
            TWIS3_S: TWIS3_S {
                _marker: PhantomData,
            },
            UARTE3_S: UARTE3_S {
                _marker: PhantomData,
            },
            GPIOTE0_S: GPIOTE0_S {
                _marker: PhantomData,
            },
            SAADC_NS: SAADC_NS {
                _marker: PhantomData,
            },
            SAADC_S: SAADC_S {
                _marker: PhantomData,
            },
            TIMER0_NS: TIMER0_NS {
                _marker: PhantomData,
            },
            TIMER0_S: TIMER0_S {
                _marker: PhantomData,
            },
            TIMER1_NS: TIMER1_NS {
                _marker: PhantomData,
            },
            TIMER1_S: TIMER1_S {
                _marker: PhantomData,
            },
            TIMER2_NS: TIMER2_NS {
                _marker: PhantomData,
            },
            TIMER2_S: TIMER2_S {
                _marker: PhantomData,
            },
            RTC0_NS: RTC0_NS {
                _marker: PhantomData,
            },
            RTC0_S: RTC0_S {
                _marker: PhantomData,
            },
            RTC1_NS: RTC1_NS {
                _marker: PhantomData,
            },
            RTC1_S: RTC1_S {
                _marker: PhantomData,
            },
            DPPIC_NS: DPPIC_NS {
                _marker: PhantomData,
            },
            DPPIC_S: DPPIC_S {
                _marker: PhantomData,
            },
            WDT_NS: WDT_NS {
                _marker: PhantomData,
            },
            WDT_S: WDT_S {
                _marker: PhantomData,
            },
            EGU0_NS: EGU0_NS {
                _marker: PhantomData,
            },
            EGU0_S: EGU0_S {
                _marker: PhantomData,
            },
            EGU1_NS: EGU1_NS {
                _marker: PhantomData,
            },
            EGU1_S: EGU1_S {
                _marker: PhantomData,
            },
            EGU2_NS: EGU2_NS {
                _marker: PhantomData,
            },
            EGU2_S: EGU2_S {
                _marker: PhantomData,
            },
            EGU3_NS: EGU3_NS {
                _marker: PhantomData,
            },
            EGU3_S: EGU3_S {
                _marker: PhantomData,
            },
            EGU4_NS: EGU4_NS {
                _marker: PhantomData,
            },
            EGU4_S: EGU4_S {
                _marker: PhantomData,
            },
            EGU5_NS: EGU5_NS {
                _marker: PhantomData,
            },
            EGU5_S: EGU5_S {
                _marker: PhantomData,
            },
            PWM0_NS: PWM0_NS {
                _marker: PhantomData,
            },
            PWM0_S: PWM0_S {
                _marker: PhantomData,
            },
            PWM1_NS: PWM1_NS {
                _marker: PhantomData,
            },
            PWM1_S: PWM1_S {
                _marker: PhantomData,
            },
            PWM2_NS: PWM2_NS {
                _marker: PhantomData,
            },
            PWM2_S: PWM2_S {
                _marker: PhantomData,
            },
            PWM3_NS: PWM3_NS {
                _marker: PhantomData,
            },
            PWM3_S: PWM3_S {
                _marker: PhantomData,
            },
            PDM_NS: PDM_NS {
                _marker: PhantomData,
            },
            PDM_S: PDM_S {
                _marker: PhantomData,
            },
            I2S_NS: I2S_NS {
                _marker: PhantomData,
            },
            I2S_S: I2S_S {
                _marker: PhantomData,
            },
            IPC_NS: IPC_NS {
                _marker: PhantomData,
            },
            IPC_S: IPC_S {
                _marker: PhantomData,
            },
            FPU_NS: FPU_NS {
                _marker: PhantomData,
            },
            FPU_S: FPU_S {
                _marker: PhantomData,
            },
            GPIOTE1_NS: GPIOTE1_NS {
                _marker: PhantomData,
            },
            KMU_NS: KMU_NS {
                _marker: PhantomData,
            },
            NVMC_NS: NVMC_NS {
                _marker: PhantomData,
            },
            KMU_S: KMU_S {
                _marker: PhantomData,
            },
            NVMC_S: NVMC_S {
                _marker: PhantomData,
            },
            VMC_NS: VMC_NS {
                _marker: PhantomData,
            },
            VMC_S: VMC_S {
                _marker: PhantomData,
            },
            CRYPTOCELL_S: CRYPTOCELL_S {
                _marker: PhantomData,
            },
            P0_NS: P0_NS {
                _marker: PhantomData,
            },
            P0_S: P0_S {
                _marker: PhantomData,
            },
        }
    }
}