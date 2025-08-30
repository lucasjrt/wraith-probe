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
            display.width(),
            self.title_height(display),
            Some(display.theme().primary()),
        );
        display.text(
            self.title(),
            -1,
            (self.title_height(display) - display.font_height()) / 2,
            Some(display.theme().secondary()),
            Some(Alignment::Center),
        );
    }

    fn render_content(&self, display: &mut EspDisplay) {
        let title_height = self.title_height(display);
        let mut y = title_height + self.title_margin();
        let Some(menu) = self.menu() else {
            display.text(self.content(), 0, y, Some(display.theme().primary()), None);
            return;
        };

        for (i, item) in menu.items().iter().enumerate() {
            let x = self.item_gap();
            if i == menu.previous_selected() {
                display.fill_rect(x, y, display.width() - x, self.item_height(display), None);
            }

            if i == menu.selected() {
                display.fill_rect(
                    x,
                    y,
                    display.width() - x,
                    self.item_height(display),
                    Some(display.theme().secondary()),
                );
                display.text(
                    &format!("> {}", item),
                    x,
                    y + (self.item_gap() / 2),
                    Some(display.theme().primary()),
                    None,
                );
            } else {
                display.text(
                    item,
                    x,
                    y + (self.item_gap() / 2),
                    Some(display.theme().primary()),
                    None,
                );
            }
            y += self.item_height(display);
        }
    }

    #[allow(unused_variables)]
    fn on_event(&mut self, event: &AppEvent, ctx: &mut Context) -> Option<RouterCommand> {
        if event == &AppEvent::BackPressed {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }

    fn title_margin(&self) -> i32 {
        3
    }

    fn title_height(&self, display: &EspDisplay) -> i32 {
        display.font_height() + (self.title_margin() * 2)
    }

    fn title(&self) -> &str {
        "Title"
    }

    fn item_gap(&self) -> i32 {
        2
    }

    fn item_height(&self, display: &EspDisplay) -> i32 {
        display.font_height() + self.item_gap()
    }

    fn render(&self, display: &mut EspDisplay) {
        display.clear();
        self.render_title(display);
        self.render_content(display);
    }

    fn menu(&self) -> Option<&dyn Menu> {
        None
    }

    fn content(&self) -> &str {
        ""
    }

    fn get_title_bounding_box(&self, display: &EspDisplay) -> Rectangle {
        Rectangle::new(
            Point::new(0, 0),
            Size::new(display.width() as u32, self.title_height(display) as u32),
        )
    }

    fn get_content_bounding_box(&self, display: &EspDisplay) -> Rectangle {
        Rectangle::new(
            Point::new(0, self.title_height(display)),
            Size::new(
                display.width() as u32,
                (display.height() - self.title_height(display)) as u32,
            ),
        )
    }
}
