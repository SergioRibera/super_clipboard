use crate::clipboard::ClipboardItem;

use abomonation::{decode, encode};

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub struct MDnsDevice {
    pub device_id: String,
    pub name: String,
    pub os: String,
}

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub enum MDnsMessage {
    // other devices response `Connecteted` message
    Welcome {
        from: MDnsDevice,
        to: MDnsDevice,
    },
    // When I join into multicast network
    Connected {
        device: MDnsDevice,
    },
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

impl Default for MDnsDevice {
    fn default() -> Self {
        #[cfg(feature = "mobile")]
        let device_id = uuid::Uuid::new_v4().to_string();
        #[cfg(feature = "mobile")]
        let name = "Unknown".to_string();

        #[cfg(feature = "pc")]
        let device_id = machine_uid::get().unwrap_or_default();

        #[cfg(feature = "pc")]
        let name = {
            let binding = gethostname::gethostname();
            binding.into_string().unwrap()
        };
        Self {
            device_id,
            name,
            os: std::env::consts::OS.to_string(),
        }
    }
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

pub fn decode_message(b: &[u8]) -> Option<MDnsMessage> {
    let mut b = b.to_vec();
    unsafe { decode::<MDnsMessage>(&mut b) }.map(|(msg, _)| msg.clone())
}

pub fn encode_message(d: &MDnsMessage) -> Vec<u8> {
    let mut b = Vec::new();
    unsafe {
        encode(d, &mut b).unwrap();
    }
    b
}

