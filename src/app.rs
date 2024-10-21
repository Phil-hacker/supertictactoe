use crate::{game::*, game_widget::GameField};

#[derive(Debug, Default)]
enum AppState {
    OfflineGame(Vec<Game>),
    #[default]
    Home,
}

#[derive(Debug, Default)]
pub struct SuperTicTacToe {
    state: AppState,
}

impl SuperTicTacToe {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for SuperTicTacToe {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                    #[cfg(debug_assertions)]
                    ui.label(format!(
                        "CPU: {:.2}ms",
                        _frame.info().cpu_usage.unwrap_or(0.0) * 1000.0
                    ));
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.vertical_centered(|ui| {
                ui.heading("Super TicTacToe");
                match &mut self.state {
                    AppState::OfflineGame(game) => {
                        ui.add(GameField::new(game, None));
                        let winner = game.last().unwrap().get_winner();
                        if match winner {
                            Winner::Draw => ui.label("draw"),
                            Winner::Cross => ui.label("X wins"),
                            Winner::Circle => ui.label("O wins"),
                            Winner::None => ui.button("undo"),
                        }.clicked() && game.len() > 1 { let _ = game.pop(); };
                        if !matches!(winner, Winner::None) && ui.button("Play again").clicked() {
                            self.state = AppState::OfflineGame(vec![Game::default()])
                        }
                        if !matches!(winner, Winner::None) && ui.button("Home").clicked() {
                            self.state = AppState::Home
                        }
                    }
                    AppState::Home => {
                        if ui.button("Offline Game").clicked() {
                            self.state = AppState::OfflineGame(vec![Game::default()]);
                        }
                    }
                }
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
