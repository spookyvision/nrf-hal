use uarte::{self, Uarte};

use heapless;
use heapless::RingBuffer;
use heapless::consts::*;
use heapless::ring_buffer::{Producer, Consumer};

use target::UARTE0;
use core::mem::transmute;

// TODO, make generic for RingBuffer sizes
// TODO, make generic for separate RX and TX RB sizes
struct UarteStreamer<'buf, T> {
    periph: T,

    tx_ring: &'buf mut RingBuffer<u8, U1024, u16>,
    tx_buf: &'buf mut [u8],

    rx_ring: &'buf mut RingBuffer<u8, U1024, u16>,
    rx_buf: &'buf mut [u8],
}

struct TxHandle<'buf> {
    sender: Producer<'buf, u8, U1024, u16>,
    // TODO: give a handle to check errors? Maybe AtomicBool/Int?
}

struct RxHandle<'buf> {
    receiver: Consumer<'buf, u8, U1024, u16>
    // TODO: give a handle to check errors? Maybe AtomicBool/Int?
}

struct Context<T> {
    tx_recv: Consumer<'static, u8, U1024, u16>,
    tx_buf: &'static mut [u8],

    rx_send: Producer<'static, u8, U1024, u16>,
    rx_buf: &'static mut [u8],

    periph: T,
}

// TODO: Hm, this can't be generic, but to be fair, there is only one.
// if we had more than one, we would want one context per
static mut CONTEXT: Option<Context<UARTE0>> = None;

impl<'buf> UarteStreamer<'buf, UARTE0> {
    pub fn rip_and_grip(self) -> (TxHandle<'buf>, RxHandle<'buf>) {
        // Break up RBs
        let (tx_send, tx_recv) = self.tx_ring.split();
        let (rx_send, rx_recv) = self.rx_ring.split();

        // TODO: Ensure safety of this cast. At the least, we should probably
        // abort any transfers in progress on drop of the Handles, and disable
        // the interrupts, as the non-ring buffers will no longer be valid.
        // At the moment, the buffers are all locked to the same lifetime
        let tx_recv = unsafe { transmute::<Consumer<'buf, _, _, _>, Consumer<'static, _, _, _>>(tx_recv) };
        let rx_send = unsafe { transmute::<Producer<'buf, _, _, _>, Producer<'static, _, _, _>>(rx_send) };
        let tx_buf = unsafe { transmute::<&'buf mut _, &'static mut _>(self.tx_buf) };
        let rx_buf = unsafe { transmute::<&'buf mut _, &'static mut _>(self.rx_buf) };

        // Move 1/2 of RB, ptrs to buffers, and periph to a static location
        let context = Context {
            tx_recv,
            rx_send,
            tx_buf,
            rx_buf,
            periph: self.periph,
        };
        unsafe {
            // TODO assert CONTEXT is None?
            CONTEXT = Some(context);
            let context = CONTEXT.as_ref();
        }

        // Set interrupt handler
        unsafe { uarte::set_interrupt_handler(&stream_handler as *const _ as usize) };

        // enable interrupts

        // Maybe check for contents in the buffer already, trigger an interrupt?
        unimplemented!()
    }
}

fn stream_handler() -> () {
    // unimplemented!()
}