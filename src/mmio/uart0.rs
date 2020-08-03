use crate::driver::common::DeviceDriver;
use crate::mmio::common::MMIOWrapper;
use crate::util::mutex::{Mutex, SyncMutex};
use core::fmt;
use cortex_a::asm;
use register::{mmio::*, register_bitfields, register_structs};

type Registers = MMIOWrapper<RegisterBlock>;

// Documentation taken from BCM2835 ARM Peripherals
register_bitfields! [
    u32,

    /*
        [UART Flag Register]
        See BCM2835 ARM Peripherals p. 182
    */
    FR [
        TXFE OFFSET(7) NUMBITS(1) [],   // Transmit FIFO Empty
        RXFF OFFSET(6) NUMBITS(1) [],   // Recieve FIFO Full
        TXFF OFFSET(5) NUMBITS(1) [],   // Transmit FIFO Full
        RXFE OFFSET(4) NUMBITS(1) [],   // Recieve FIFO Empty
        BUSY OFFSET(3) NUMBITS(1) [],   // UART is busy
        CTS OFFSET(0) NUMBITS(1) []     // UART Clear to Send
    ],

    /*
        [UART Baud Rate Divisor Register]
        See BCM2835 ARM Peripherals p. 183
    */
    IBRD [
        IBRD OFFSET(0) NUMBITS(16) []   // Integer Baud Rate Divisor
    ],

    /*
        [UART Fractional Baud Rate Divisor Register]
        See BCM2835 ARM Peripherals p. 183
    */
    FBRD [
        FBRD OFFSET(0) NUMBITS(6) []    // Fractional Baud Rate Divisor
    ],

    /*
        [UART Line Control Register]
        See BCM2835 ARM Peripherals p. 184
    */
    LCRH [
        SPS OFFSET(7) NUMBITS(1) [      // Stick Partiy Select
            DISABLED = 0,
            EPS = 1
        ],
        WLEN OFFSET(5) NUMBITS(2) [     // Word Length of Transmitted Data
            FIVE_BITS = 0b00,
            SIX_BITS = 0b01,
            SEVEN_BITS = 0b10,
            EIGHT_BITS = 0b11
        ],
        FEN OFFSET(4) NUMBITS(1) [      // Enable FIFO
            DISABLED = 0,
            ENABLED = 1
        ],
        STP2 OFFSET(3) NUMBITS(1) [],   // Two Stop Bits Select
        EPS OFFSET(2) NUMBITS(1) [      // Even Parity Select
            ODD_PARITY = 0,
            EVEN_PARITY = 1
        ],
        PEN OFFSET(1) NUMBITS(1) [      // Parity Enable
            DISABLED = 0,
            ENABLED = 1
        ],
        BRK OFFSET(0) NUMBITS(1) []     // Send Break
    ],

    /*
        [UART Control Register]
        See BCM2835 ARM Peripherals p. 185
    */
    CR [
        RXE OFFSET(9) NUMBITS(1) [      // UART Recieve Enable
            DISABLED = 0,
            ENABLED = 1
        ],

        TXE OFFSET(8) NUMBITS(1) [      // UART Transmit Enable
            DISABLED = 0,
            ENABLED = 1
        ],

        UARTEN OFFSET(0) NUMBITS(1) [   // UART Enable
            DISABLED = 0,
            ENABLED = 1
        ]
    ],

    /*
        [UART Interrupt Clear Register]
        See BCM2835 ARM Peripherals p. 192
    */
    ICR [
        ALL OFFSET(0) NUMBITS(11) []    // Metadata for Pending Interrupts
    ]
];

register_structs! {
    /*
        [UART Register Map]
        See BCM2835 ARM Peripherals p. 177
    */
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCRH: WriteOnly<u32, LCRH::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct UART0 {
    inner: SyncMutex<_UART0>,
}

impl UART0 {
    pub const unsafe fn new(base_addr: usize) -> Self {
        UART0 {
            inner: SyncMutex::new(_UART0::new(base_addr)),
        }
    }

    // See that nasty github repo, maybe turn this into a Writer trait?
    pub fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        let mut r = &self.inner;
        r.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
}

impl DeviceDriver for UART0 {
    fn display_name(&self) -> &str {
        "PL011 UART"
    }

    fn init(&self) -> Result<(), ()> {
        let mut r = &self.inner;
        r.lock(|inner| inner.init());
        Ok(())
    }
    
}

struct _UART0 {
    registers: Registers,
}

impl _UART0 {
    pub const unsafe fn new(base_addr: usize) -> Self {
        _UART0 {
            registers: Registers::new(base_addr),
        }
    }

    pub fn init(&mut self) {
        self.registers.CR.set(0);

        self.registers.ICR.write(ICR::ALL::CLEAR);
        self.registers.IBRD.write(IBRD::IBRD.val(13));
        self.registers.FBRD.write(FBRD::FBRD.val(1));

        self.registers
            .LCRH
            .write(LCRH::WLEN::EIGHT_BITS + LCRH::FEN::ENABLED);
        self.registers
            .CR
            .write(CR::UARTEN::ENABLED + CR::TXE::ENABLED + CR::RXE::ENABLED);
    }

    fn write_char(&mut self, c: char) {
        while self.registers.FR.matches_all(FR::TXFF::SET) {
            asm::nop();
        }

        self.registers.DR.set(c as u32);
    }
}

impl fmt::Write for _UART0 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
