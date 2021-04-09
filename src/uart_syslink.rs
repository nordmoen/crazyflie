//! Communication support between the `STM32F405` and `nRF51`
use crate::hal::gpio::{
    gpioc::{PC6, PC7},
    Alternate, Floating, Input, AF8,
};
use crate::hal::nb;
use crate::hal::pac::USART6;
use crate::hal::prelude::*;
use crate::hal::rcc::Clocks;
use crate::hal::serial::{self, config, Serial};
use heapless::{consts::U70, Vec};
use syslink;

pub type TxPin = PC6<Alternate<AF8>>;
pub type RxPin = PC7<Alternate<AF8>>;
/// Underlying UART connection
pub type SerialConn = Serial<USART6, (TxPin, RxPin)>;

const BAUDRATE: u32 = 1_000_000;

/// Potential errors that could occur when sending [`syslink`] packets
pub enum SendError {
    /// Error related to writing a [`syslink`] packet to a byte stream
    Syslink(syslink::WriteError),
    /// Problem sending or receiving data on the underlying UART connection
    Uart(serial::Error),
}

/// Potential errors that could occur when receiving a [`syslink`] packet
pub enum RecvError {
    /// Error related to parsing the raw stream of bytes into a valid packet
    ///
    /// For the errors that signal truncation needed this will be handled by this module and is not
    /// needed on the user side. The error is simply returned fully in case it is of interest to
    /// the user.
    Syslink(syslink::ParseError),
    /// Problem receiving data on the underlying UART connection
    Uart(serial::Error),
    /// The receive buffer has filled up
    ReceiveBufferFull,
}

/// Internal communication channel between `STM32F405` and `nRF51` based on reading the
/// corresponding UART channel
pub struct UartComm {
    conn: SerialConn,
    recv_buff: Vec<u8, U70>,
}

impl UartComm {
    /// Create a new communication channel to the `nRF51`
    pub fn new(
        usart: USART6,
        tx_pin: PC6<Input<Floating>>,
        rx_pin: PC7<Input<Floating>>,
        clocks: Clocks,
    ) -> Self {
        let tx_pin = tx_pin.into_alternate_af8();
        let rx_pin = rx_pin.into_alternate_af8();
        let uart_cfg = config::Config::default()
            .baudrate(BAUDRATE.bps())
            .parity_none()
            .wordlength_8()
            .stopbits(config::StopBits::STOP1);
        Self {
            // Unwrap safety: Since the configuration is controlled internally it should always be
            // correct, if this fails then the method will have to be reconfigured
            conn: Serial::usart6(usart, (tx_pin, rx_pin), uart_cfg, clocks).unwrap(),
            recv_buff: Vec::new(),
        }
    }

    /// Send a [`Packet`](syslink::Packet) over UART to the `nRF51`, blocking to write whole packet
    pub fn send(&mut self, packet: syslink::Packet) -> Result<(), SendError> {
        let mut buffer = [0u8; 72];
        let bytes = packet.write(&mut buffer).map_err(SendError::Syslink)?;
        self.conn
            .bwrite_all(&buffer[..bytes])
            .map_err(SendError::Uart)?;
        Ok(())
    }

    /// Receive a new [`Packet`](syslink::Packet) over UART from the `nRF51`
    pub fn receive(&mut self) -> nb::Result<syslink::Packet, RecvError> {
        // Try to read from UART connection
        let data = self.conn.read().map_err(|e| e.map(RecvError::Uart))?;
        // Try to add to our receive buffer, overflow here should "never" happen, but if it does we
        // signal it so that calling users can clear the buffer
        self.recv_buff.push(data).map_err(|_| RecvError::ReceiveBufferFull)?;
        // Try to parse packet from data buffer
        match syslink::Packet::from(&self.recv_buff) {
            // If we parsed a complete packet, clear the internal buffer and return the packet
            Ok((slice, packet)) => {
                // Unwrap safety: Since the slice already comes from 'recv_buff' this can never
                // fail
                self.recv_buff = Vec::from_slice(&slice).unwrap();
                Ok(packet)
            }
            // Not yet done parsing, do nothing with the internal buffer and signal blockage
            Err(syslink::ParseError::Incomplete(_)) => Err(nb::Error::WouldBlock),
            // The other parsing errors require us to truncate the underlying receive buffer before
            // returning the error
            Err(e @ syslink::ParseError::WrongTag) => {
                self.truncate_first(1);
                Err(nb::Error::Other(RecvError::Syslink(e)))
            }
            Err(syslink::ParseError::TooMuchData(tr)) => {
                self.truncate_first(tr);
                Err(nb::Error::Other(RecvError::Syslink(
                    syslink::ParseError::TooMuchData(tr),
                )))
            }
            Err(syslink::ParseError::WrongChecksum(tr)) => {
                self.truncate_first(tr);
                Err(nb::Error::Other(RecvError::Syslink(
                    syslink::ParseError::WrongChecksum(tr),
                )))
            }
        }
    }

    /// Remove the first `bytes` from the internal receiver buffer
    ///
    /// This method is used when a parse error occurs and we need to discard a given number of
    /// bytes from the start of the buffer.
    fn truncate_first(&mut self, bytes: usize) {
        for _ in 0..bytes {
            // Exit early if we are asked to remove more values than are stored in the buffer
            if let None = self.recv_buff.pop() {
                break;
            }
        }
    }

    /// Empty receiver buffer
    ///
    /// This might be necessary if the receiver buffer fills up to max capacity.
    pub fn empty_buffer(&mut self) {
        self.recv_buff.clear();
    }

    /// Enable interrupt ([`Event`](serial::Event)) for the UART connection
    pub fn enable_interrupt(&mut self, event: serial::Event) {
        self.conn.listen(event);
    }

    /// Disable interrupt ([`Event`](serial::Event)) for the UART connection
    pub fn disable_interrupt(&mut self, event: serial::Event) {
        self.conn.unlisten(event);
    }
}
