use std::{sync::mpsc::Sender, time::Instant};

use button_driver::{Button, ButtonConfig, PinWrapper};
use esp_idf_svc::hal::delay::Delay;

use crate::AppEvent;

pub struct Input<S: PinWrapper, B: PinWrapper> {
    delay: Delay,
    select_button: Button<S, Instant>,
    back_button: Button<B, Instant>,
}

impl<S: PinWrapper, B: PinWrapper> Input<S, B> {
    pub fn new(select_pin: S, back_pin: B) -> Self {
        Input {
            delay: Delay::new(1),
            select_button: Button::<_, Instant>::new(select_pin, ButtonConfig::default()),
            back_button: Button::<_, Instant>::new(back_pin, ButtonConfig::default()),
        }
    }

    pub fn run(&mut self, app_tx: Sender<AppEvent>) {
        let mut select_hold_handled = false;
        let mut back_hold_handled = false;
        loop {
            self.tick();
            if self.select_button.is_clicked() {
                if let Err(e) = app_tx.send(AppEvent::DownPressed) {
                    log::error!("Failed to send SelectPressed event: {}", e);
                }
            }
            if self.back_button.is_clicked() {
                if let Err(e) = app_tx.send(AppEvent::UpPressed) {
                    log::error!("Failed to send BackPressed event: {}", e);
                }
            }

            if self
                .select_button
                .current_holding_time()
                .unwrap_or_default()
                .as_millis()
                > 300
                && !select_hold_handled
            {
                select_hold_handled = true;
                self.select_button.reset();
                if let Err(e) = app_tx.send(AppEvent::SelectPressed) {
                    log::error!("Failed to send SelectHeld event: {}", e);
                }
            }
            if self
                .back_button
                .current_holding_time()
                .unwrap_or_default()
                .as_millis()
                > 300
                && !back_hold_handled
            {
                back_hold_handled = true;
                self.back_button.reset();
                if let Err(e) = app_tx.send(AppEvent::BackPressed) {
                    log::error!("Failed to send BackHeld event: {}", e);
                }
            }

            if self.select_button.is_double_clicked() {
                log::info!("Select button double clicked but no action assigned.");
            }
            if self.back_button.is_double_clicked() {
                log::info!("Back button double clicked but no action assigned.");
            }

            if self
                .select_button
                .held_time()
                .unwrap_or_default()
                .as_millis()
                > 300
            {
                select_hold_handled = false;
            }

            if self.back_button.held_time().unwrap_or_default().as_millis() > 300 {
                back_hold_handled = false;
            }

            self.back_button.reset();
            self.select_button.reset();
            self.delay.delay_ms(5);
        }
    }

    fn tick(&mut self) {
        self.select_button.tick();
        self.back_button.tick();
    }
}
