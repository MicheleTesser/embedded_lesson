#![cfg(test)]
use embedded_lesson::{BusUsage, CanBus, CanMessage, Mex, SerialBus, SerialMessage};

#[test]
fn test_can_message_creation() {
  let data = vec![0x01, 0x02, 0x03, 0x04];
  let message = CanMessage::new(0x123, 8, data);

  assert_eq!(message.id, 0x123);
  assert_eq!(message.dlc, 8);
  assert_eq!(message.data, vec![0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn test_can_bus_send_valid_id() {
  let can_bus = CanBus;
  let message = CanMessage::new(0x123, 8, vec![0x01, 0x02, 0x03, 0x04]);

  let result = can_bus.send(message);

  assert!(result.is_ok());
}

#[test]
fn test_can_bus_send_invalid_id() {
  let can_bus = CanBus;
  let message = CanMessage::new(0x800, 8, vec![0x01, 0x02, 0x03, 0x04]);

  let result = can_bus.send(message);

  assert!(result.is_err());
  assert_eq!(result.unwrap_err(), "Invalid CAN ID");
}

#[test]
fn test_can_bus_receive() {
  let can_bus = CanBus;
  let received_message = can_bus.recieve();

  assert_eq!(received_message.id, 0x123);
  assert_eq!(received_message.dlc, 8);
  assert_eq!(received_message.data, vec![0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn test_serial_message_creation() {
  let data = vec![0x01, 0x02, 0x03, 0x04];
  let message = SerialMessage::new(0x01, 4, data);

  assert_eq!(message.id, 0x01);
  assert_eq!(message.dlc, 4);
  assert_eq!(message.data, vec![0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn test_serial_bus_send_valid_message() {
  let serial_bus = SerialBus;
  let message = SerialMessage::new(0x01, 4, vec![0x01, 0x02, 0x03, 0x04]);

  let result = serial_bus.send(message);

  assert!(result.is_ok());
}

#[test]
fn test_serial_bus_send_invalid_message() {
  let serial_bus = SerialBus;
  let message = SerialMessage::new(0x01, 300, vec![0x01, 0x02, 0x03, 0x04]);

  let result = serial_bus.send(message);

  assert!(result.is_err());
  assert_eq!(result.unwrap_err(), "Data too long");
}

#[test]
fn test_serial_bus_receive() {
  let serial_bus = SerialBus;
  let received_message = serial_bus.recieve();

  assert_eq!(received_message.id, 0x01);
  assert_eq!(received_message.dlc, 4);
  assert_eq!(received_message.data, vec![0x01, 0x02, 0x03, 0x04]);
}
