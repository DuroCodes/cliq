use eframe::egui;
use global_hotkey::hotkey::{HotKey, Modifiers};

use crate::{
    app::CliqApp,
    hotkey::{egui_key_to_code, format_hotkey},
};

impl eframe::App for CliqApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_hotkey();

        render_status_panel(self, ctx);
        render_central_panel(self, ctx);
        render_hotkey_modal(self, ctx);

        self.handle_click();
        ctx.request_repaint();
    }
}

fn render_status_panel(app: &mut CliqApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("status_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.strong("Status:");
            let status = if app.is_enabled { "ON" } else { "OFF" };
            ui.add(egui::Label::new(egui::RichText::new(status).color(
                if app.is_enabled {
                    egui::Color32::GREEN
                } else {
                    egui::Color32::GRAY
                },
            )))
            .on_hover_text("Press hotkey to toggle");

            ui.separator();

            if ui.checkbox(&mut app.is_sticky, "Sticky").changed() {
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(match app.is_sticky {
                    true => egui::WindowLevel::AlwaysOnTop,
                    false => egui::WindowLevel::Normal,
                }));
            }

            ui.separator();

            ui.checkbox(&mut app.is_right_click, "Right").changed();
        });
    });
}

fn render_central_panel(app: &mut CliqApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 4.0;
        ui.style_mut().spacing.window_margin = egui::Margin::ZERO;

        ui.horizontal(|ui| {
            ui.strong("Hotkey:");
            if ui
                .button(format_hotkey(&app.hotkey))
                .on_hover_text("Click to change hotkey")
                .clicked()
            {
                app.is_setting_hotkey = true;
            }
        });

        ui.add(egui::Separator::default().spacing(2.0));

        ui.horizontal(|ui| {
            ui.set_min_width(60.0);
            ui.strong("Delay:");
            ui.add(
                egui::DragValue::new(&mut app.delay_ms)
                    .speed(1)
                    .range(1..=1000)
                    .suffix(" ms"),
            );
        });
    });
}

fn render_hotkey_modal(app: &mut CliqApp, ctx: &egui::Context) {
    if app.is_setting_hotkey {
        egui::Window::new("Set Hotkey")
            .collapsible(false)
            .resizable(false)
            .fixed_size(egui::vec2(200.0, 80.0))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.label("Press any key combination...");

                let mut modifiers = Vec::new();
                if ctx.input(|i| i.modifiers.ctrl) {
                    modifiers.push("Ctrl");
                }
                if ctx.input(|i| i.modifiers.shift) {
                    modifiers.push("Shift");
                }
                if ctx.input(|i| i.modifiers.alt) {
                    modifiers.push("Alt");
                }
                if ctx.input(|i| i.modifiers.mac_cmd) {
                    modifiers.push("Cmd");
                }

                if let Some(key) = ctx.input(|i| i.keys_down.iter().next().copied()) {
                    let key_name = if modifiers.is_empty() {
                        format!("{key:?}")
                    } else {
                        format!("{} + {key:?}", modifiers.join(" + "))
                    };
                    ui.label(&key_name);

                    let mut mods = None;
                    if !modifiers.is_empty() {
                        let modifier_map = [
                            ("Ctrl", Modifiers::CONTROL),
                            ("Shift", Modifiers::SHIFT),
                            ("Alt", Modifiers::ALT),
                            ("Cmd", Modifiers::SUPER),
                        ];

                        let m = modifier_map
                            .iter()
                            .filter(|(name, _)| modifiers.contains(name))
                            .fold(Modifiers::empty(), |acc, (_, m)| acc | *m);

                        mods = Some(m);
                    }

                    if let Some(code) = egui_key_to_code(key) {
                        let new_hotkey = HotKey::new(mods, code);
                        if let Ok(()) = app.manager.register(new_hotkey) {
                            let _ = app.manager.unregister(app.hotkey);
                            app.hotkey = new_hotkey;
                            app.is_setting_hotkey = false;
                        }
                    }
                }
            });
    }
}
