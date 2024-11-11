#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub trait Mex<DataType, IdType>
  where ():
    {
      fn new(id: IdType, length: u32, data: DataType) -> Self;
      fn id(mex: Self) -> IdType;
      fn dlc(mex: Self) -> u32;
      fn data(mex: Self) -> DataType;
}

pub trait BusUsage<P, D, I >
  where P : Mex<D, I>
    {
      fn new(&self) -> Box<dyn BusUsage<P, D, I>>;
      fn send(&self, p: P) -> Result<(), &str>;
      fn recieve(&self) -> P;

}



pub struct CanMessage {
  pub id: u16,       // 11-bit
  pub dlc: u32,
  pub data: Vec<u8>,
}

impl Mex<Vec<u8>, u16> for CanMessage {
  fn new(id: u16, length: u32, data: Vec<u8>) -> Self {
    Self { id, dlc:length, data }
  }

  fn id(mex: Self) -> u16 {
    mex.id
  }

  fn dlc(mex: Self) -> u32 {
    mex.dlc
  }

  fn data(mex: Self) -> Vec<u8> {
    mex.data
  }
}

pub struct CanBus;

impl BusUsage<CanMessage, Vec<u8>, u16> for CanBus {
  fn new(&self) -> Box<dyn BusUsage<CanMessage, Vec<u8>, u16>> {
    Box::new(CanBus)
  }

  fn send(&self, p: CanMessage) -> Result<(), &str> {
    if p.id > 0x7FF {
      return Err("Invalid CAN ID");
    }
    Ok(())
  }

  fn recieve(&self) -> CanMessage {
    CanMessage::new(0x123, 8, vec![0x01, 0x02, 0x03, 0x04])
  }
}

pub struct SerialMessage {
  pub id: u8,
  pub dlc: u32,
  pub data: Vec<u8>,
}

impl Mex<Vec<u8>, u8> for SerialMessage {
  fn new(id: u8, length: u32, data: Vec<u8>) -> Self {
    Self { id, dlc:length, data }
  }

  fn id(mex: Self) -> u8 {
    mex.id
  }

  fn dlc(mex: Self) -> u32 {
    mex.dlc
  }

  fn data(mex: Self) -> Vec<u8> {
    mex.data
  }
}

pub struct SerialBus;

impl BusUsage<SerialMessage, Vec<u8>, u8> for SerialBus {
  fn new(&self) -> Box<dyn BusUsage<SerialMessage, Vec<u8>, u8>> {
    Box::new(SerialBus)
  }

  fn send(&self, p: SerialMessage) -> Result<(), &str> {
    if p.dlc > 256 {
      return Err("Data too long");
    }
    Ok(())
  }

  fn recieve(&self) -> SerialMessage {
    SerialMessage::new(0x01, 4, vec![0x01, 0x02, 0x03, 0x04])
  }
}