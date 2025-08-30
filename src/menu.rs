pub trait Menu {
    fn selected(&self) -> usize;
    fn previous_selected(&self) -> usize;
    fn items(&self) -> &[&str];
    fn selected_scroll_offset(&self) -> usize {
        0
    }
    fn menu_scroll_offset(&self) -> usize {
        0
    }
    fn set_selected_scroll_offset(&mut self, _offset: usize) {}
    fn set_menu_scroll_offset(&mut self, _offset: usize) {}
}
