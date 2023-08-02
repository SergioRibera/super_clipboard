use crate::settings::ClipboardItem;


const _MDNS_PORT: u16 = 5353;
const _MDNS_ADDRESS: &str = "224.0.0.251";

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub struct MDnsDevice {
    pub device_id: String,
    pub name: String,
    pub os: String,
}

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub enum MDnsMessage {
    Connected(MDnsDevice),
    LinkRequest {
        from: MDnsDevice,
        to: MDnsDevice,
    },
    LinkAccepted {
        from: MDnsDevice,
        to: MDnsDevice,
    },
    Clipboard {
        device: MDnsDevice,
        item: ClipboardItem,
    },
    #[allow(dead_code)]
    Message {
        device: MDnsDevice,
        content: String,
    },
}

impl MDnsMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        unsafe {
            abomonation::encode(self, &mut bytes).unwrap();
        }
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data.to_vec();
        match unsafe { abomonation::decode::<Self>(&mut data) } {
            Some((item, _)) => Ok(item.clone()),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode data",
            )),
        }
    }
}
