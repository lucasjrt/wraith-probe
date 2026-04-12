use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
    text::Alignment,
};

use crate::{
    context::{Context, Dirty},
    display::EspDisplay,
    events::AppEvent,
    menu::Menu,
    ui::router::RouterCommand,
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

    fn render_content(&mut self, display: &mut EspDisplay, ctx: Arc<RwLock<Context>>) {
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

        if menu_mut.selected() >= menu_mut.menu_scroll_offset() + (visible_item_count) {
            let new_offset = menu_mut.selected() + 1 - visible_item_count;
            menu_mut.set_menu_scroll_offset(new_offset);
        } else if menu_mut.selected() < menu_mut.menu_scroll_offset() {
            menu_mut.set_menu_scroll_offset(menu_mut.selected());
        }

        let menu_offset = menu_mut.menu_scroll_offset();
        let x = gap as i32;
        for i in 0..visible_item_count {
            let i = i + menu_offset;
            let Some(item) = menu_items.get(i) else {
                break;
            };
            if i == menu_mut.previous_selected() {
                display.fill_rect(x, y, display.width() as i32 - x, item_height, None);
            }

            if i == menu_mut.selected() {
                let mut x_offset: i32 = -menu_mut.selected_scroll_offset();
                let font_width = display.font_width();
                let text_width = item.len() * font_width;
                let display_width = display.width() - (gap * 2);

                if text_width > display_width {
                    let max_offset = (text_width - display_width) as i32;
                    let new_offset = x_offset - menu_mut.selected_scroll_speed();
                    if new_offset.abs() >= max_offset {
                        menu_mut.set_selected_scroll_offset(-max_offset);
                    } else {
                        menu_mut.set_selected_scroll_offset(new_offset);
                    }

                    // Only create a new timer if one doesn't already exist
                    if menu_mut.scroll_timer().is_none() {
                        log::info!("Creating horizontal scroll timer");
                        let callback_ctx = ctx.clone();
                        let timer = {
                            ctx.read()
                                .expect("Failed to lock context")
                                .services()
                                .timer()
                                .clone()
                        };
                        let callback_timer = timer
                            .timer(move || {
                                if let Ok(mut ctx) = callback_ctx.write() {
                                    ctx.set_dirty(Dirty::Full);
                                    let _ = ctx.services().app().send(AppEvent::Redraw);
                                }
                            })
                            .expect("Failed to create timer");
                        let delay = menu_mut.scroll_delay_ms() as u64;
                        callback_timer
                            .after(Duration::from_millis(delay))
                            .expect("Failed to set timer");
                        menu_mut.set_scroll_timer(Some(callback_timer));
                    } else {
                        // Reuse existing timer — just reschedule it
                        let delay = menu_mut.scroll_delay_ms() as u64;
                        if let Some(timer) = menu_mut.scroll_timer() {
                            let _ = timer.after(Duration::from_millis(delay));
                        }
                    }
                } else {
                    x_offset = 0;
                    menu_mut.set_selected_scroll_offset(0);
                    // Clear the timer when text fits on screen
                    menu_mut.set_scroll_timer(None);
                }

                let text_offset = 2 * display.font_width() as i32 + x_offset;

                display.fill_rect(
                    x,
                    y,
                    display.width() as i32 - x,
                    item_height,
                    Some(display.theme().secondary()),
                );
                display.text(
                    item,
                    text_offset,
                    y + (gap / 2) as i32,
                    Some(display.theme().primary()),
                    None,
                );

                display.fill_rect(
                    x,
                    y + (gap / 2) as i32,
                    display.font_width() as i32,
                    display.font_height() as i32,
                    Some(display.theme().secondary()),
                );
                display.text(
                    ">",
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

    fn on_event(&mut self, event: &AppEvent, _ctx: Arc<RwLock<Context>>) -> Option<RouterCommand> {
        if event == &AppEvent::BackPressed {
            return Some(RouterCommand::NavigateBack);
        }
        None
    }

    fn title_margin(&self) -> usize {
        3
    }

    fn title_height(&self, display: &EspDisplay) -> usize {
        display.font_height() + (self.title_margin() * 2)
    }

    fn title(&self) -> &str {
        "Title"
    }

    fn item_gap(&self) -> usize {
        5
    }

    fn item_height(&self, display: &EspDisplay) -> usize {
        display.font_height() + self.item_gap()
    }

    fn render(&mut self, display: &mut EspDisplay, ctx: Arc<RwLock<Context>>) {
        display.clear();
        self.render_title(display);
        self.render_content(display, ctx);
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

    fn get_content_bounding_box(&self, display: &EspDisplay) -> Rectangle {
        Rectangle::new(
            Point::new(0, self.title_height(display) as i32),
            Size::new(
                display.width() as u32,
                (display.height() - self.title_height(display)) as u32,
            ),
        )
    }

    fn visible_items_count(&self, display: &EspDisplay) -> usize {
        let content_height = self.get_content_bounding_box(display).size.height as usize;
        content_height / self.item_height(display)
    }
}
