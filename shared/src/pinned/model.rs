use crate::clipboard::ClipboardItem;

#[derive(Abomonation, Clone, Default, Eq, PartialEq)]
pub struct PinnedClipboard {
    clipboard: Vec<ClipboardItem>,
    #[unsafe_abomonate_ignore]
    pub is_changed: bool,
}

impl PinnedClipboard {
    pub fn clipboard(&self) -> &[ClipboardItem] {
        self.clipboard.as_ref()
    }

    pub fn add_item(&mut self, item: ClipboardItem) {
        self.clipboard.push(item);
        self.is_changed = true;
    }

    pub fn remove_item(&mut self, pos: usize) {
        self.clipboard.remove(pos);
        self.is_changed = true;
    }
}
