pub enum PcDevice {
    Nx(Communication),
    Nuc(Communication),
}

enum Communication {
    UsbSerial,
    Usart,
    Can,
}

impl PcDevice {
    pub async fn new() {
        todo!("")
    }
}

use crate::bsp::usart::Uart;

pub struct NucInterface {
    uart: Uart<'static, UART0>,
}

impl NucInterface {
    pub fn new(uart: Uart<'static, UART0>) -> Self {
        NucInterface { uart }
    }

    pub async fn send_telemetry(&mut self, telemetry: Telemetry) -> Result<(), CommunicationError> {
        let data = telemetry.serialize();
        self.uart
            .write(&data)
            .await
            .map_err(CommunicationError::UartError)
    }

    pub async fn receive_command(&mut self) -> Result<Command, CommunicationError> {
        let mut buffer = [0u8; 256]; // Adjust size as needed
        let n = self
            .uart
            .read(&mut buffer)
            .await
            .map_err(CommunicationError::UartError)?;
        Command::deserialize(&buffer[..n]).map_err(CommunicationError::DeserializationError)
    }
}

pub struct NxInterface {
    uart: Uart<'static, UART0>,
}

impl NxInterface {
    pub fn new(uart: Uart<'static, UART0>) -> Self {
        NxInterface { uart }
    }

    pub async fn send_telemetry(&mut self, telemetry: Telemetry) -> Result<(), CommunicationError> {
        let data = telemetry.serialize();
        self.uart
            .write(&data)
            .await
            .map_err(CommunicationError::UartError)
    }

    pub async fn receive_command(&mut self) -> Result<Command, CommunicationError> {
        let mut buffer = [0u8; 256]; // Adjust size as needed
        let n = self
            .uart
            .read(&mut buffer)
            .await
            .map_err(CommunicationError::UartError)?;
        Command::deserialize(&buffer[..n]).map_err(CommunicationError::DeserializationError)
    }
}
