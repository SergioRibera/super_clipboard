#[cfg(feature = "pc")]
use arboard::ImageData;
use chrono::{DateTime, Utc};

#[derive(Abomonation, Clone, Debug, Eq, PartialEq)]
pub enum ClipboardItem {
    Text {
        date: String,
        value: String,
    },
    Image {
        date: String,
        w: u64,
        h: u64,
        b: Vec<u8>,
    },
}

impl ClipboardItem {
    pub fn format(&self, fmt: &str) -> String {
        match self {
            ClipboardItem::Text { date, value: _ } => DateTime::parse_from_rfc3339(date)
                .unwrap()
                .format(fmt)
                .to_string(),
            ClipboardItem::Image {
                date,
                w: _,
                h: _,
                b: _,
            } => DateTime::parse_from_rfc3339(date)
                .unwrap()
                .format(fmt)
                .to_string(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        unsafe {
            abomonation::encode(self, &mut bytes).unwrap();
        }
        bytes
    }

    pub fn from_bytes(data: &[u8]) -> Result<ClipboardItem, std::io::Error> {
        let mut data = data.to_vec();
        match unsafe { abomonation::decode::<ClipboardItem>(&mut data) } {
            Some((item, _)) => Ok(item.clone()),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to decode data",
            )),
        }
    }
}

impl ToString for ClipboardItem {
    fn to_string(&self) -> String {
        match self {
            ClipboardItem::Text { date: _, value: v } => v.clone(),
            // @TODO:Convert to base64
            ClipboardItem::Image {
                date: _,
                w,
                h,
                b: _,
            } => format!("{w} - {h}"),
        }
    }
}

impl From<String> for ClipboardItem {
    fn from(value: String) -> Self {
        Self::Text {
            date: Utc::now().to_rfc3339(),
            value,
        }
    }
}

#[cfg(feature = "pc")]
impl From<ImageData<'_>> for ClipboardItem {
    fn from(value: ImageData) -> Self {
        let ImageData {
            width: w,
            height: h,
            bytes,
        } = value;
        Self::Image {
            date: Utc::now().to_rfc3339(),
            w: w as u64,
            h: h as u64,
            b: bytes.to_vec(),
        }
    }
}
