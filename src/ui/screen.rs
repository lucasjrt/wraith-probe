use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
    text::Alignment,
};

use crate::{
    context::Context, display::EspDisplay, events::AppEvent, menu::Menu, ui::router::RouterCommand,
};

pub trait Screen {
    fn render_title(&self, display: &mut EspDisplay) {
        display.fill_rect(
            0,
            0,
            display.width() as i32,
            self.title_height(display) as i32,
            Some(display.theme().primary()),
        );
        display.text(
            self.title(),
            -1,
            (self.title_height(display) - display.font_height()) as i32 / 2,
            Some(display.theme().secondary()),
            Some(Alignment::Center),
        );
    }

    fn render_content(&mut self, display: &mut EspDisplay) {
        let title_height = self.title_height(display);
        let visible_item_count = self.visible_items_count(display);
        let item_height = self.item_height(display) as i32;
        let gap = self.item_gap();
        let mut y = (title_height + self.title_margin()) as i32;
        let menu_items: Vec<String> = self
            .menu()
            .map(|m| m.items().iter().map(|s| s.to_string()).collect())
            .unwrap_or_default();

        let Some(menu_mut) = self.menu_mut() else {
            display.text(
                self.content(),
                self.item_gap() as i32,
                y,
                Some(display.theme().primary()),
                None,
            );
            return;
        };

        if menu_mut.selected() >= menu_mut.menu_scroll_offset() + (visible_item_count as usize) {
            let new_offset = menu_mut.selected() + 1 - (visible_item_count as usize);
            menu_mut.set_menu_scroll_offset(new_offset);
        } else if menu_mut.selected() < menu_mut.menu_scroll_offset() {
            menu_mut.set_menu_scroll_offset(menu_mut.selected());
        }

        let menu_offset = menu_mut.menu_scroll_offset();
        let x = gap as i32;
        for i in 0..visible_item_count as usize {
            let i = i + menu_offset;
            let Some(item) = menu_items.get(i) else {
                break;
            };
            if i == menu_mut.previous_selected() {
                display.fill_rect(x, y, display.width() as i32 - x, item_height, None);
            }

            if i == menu_mut.selected() {
                display.fill_rect(
                    x,
                    y,
                    display.width() as i32 - x,
                    item_height,
                    Some(display.theme().secondary()),
                );
                display.text(
                    &format!("> {}", item),
                    x,
                    y + (gap / 2) as i32,
                    Some(display.theme().primary()),
                    None,
                );
            } else {
                display.text(
                    item,
                    x,
                    y + (gap / 2) as i32,
                    Some(display.theme().primary()),
                    None,
                );
            }
            y += item_height;
        }
    }

    #[allow(unused_variables)]
    fn on_event(&mut self, event: &AppEvent, ctx: &mut Context) -> Option<RouterCommand> {
        if event == &AppEvent::BackPressed {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }

    fn title_margin(&self) -> u32 {
        3
    }

    fn title_height(&self, display: &EspDisplay) -> u32 {
        display.font_height() + (self.title_margin() * 2)
    }

    fn title(&self) -> &str {
        "Title"
    }

    fn item_gap(&self) -> u32 {
        5
    }

    fn item_height(&self, display: &EspDisplay) -> u32 {
        display.font_height() + self.item_gap()
    }

    fn render(&mut self, display: &mut EspDisplay) {
        display.clear();
        self.render_title(display);
        self.render_content(display);
    }

    fn menu(&self) -> Option<&dyn Menu> {
        None
    }

    fn menu_mut(&mut self) -> Option<&mut dyn Menu> {
        None
    }

    fn content(&self) -> &str {
        ""
    }

    fn get_title_bounding_box(&self, display: &EspDisplay) -> Rectangle {
        Rectangle::new(
            Point::new(0, 0),
            Size::new(display.width(), self.title_height(display)),
        )
    }

    fn get_content_bounding_box(&self, display: &EspDisplay) -> Rectangle {
        Rectangle::new(
            Point::new(0, self.title_height(display) as i32),
            Size::new(
                display.width(),
                display.height() - self.title_height(display),
            ),
        )
    }

    fn visible_items_count(&self, display: &EspDisplay) -> u32 {
        let content_height = self.get_content_bounding_box(display).size.height;
        content_height / self.item_height(display)
    }
}
