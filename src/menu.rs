use esp_idf_svc::timer::EspTimer;

pub trait Menu {
    fn selected(&self) -> usize;
    fn previous_selected(&self) -> usize;
    fn items(&self) -> &[&str];
    fn menu_scroll_offset(&self) -> usize {
        0
    }
    fn set_menu_scroll_offset(&mut self, _offset: usize) {}
    fn selected_scroll_offset(&self) -> i32 {
        0
    }
    fn set_selected_scroll_offset(&mut self, _offset: i32) {}
    fn selected_scroll_speed(&self) -> i32 {
        4
    }
    fn scroll_delay_ms(&self) -> u32 {
        80
    }
    fn scroll_timer(&self) -> &Option<EspTimer<'static>> {
        &None
    }
    fn set_scroll_timer(&mut self, _timer: Option<EspTimer<'static>>) {}
}
