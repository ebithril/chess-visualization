use egui::{Button, Label, Visuals};
use core::fmt;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
enum ChessSquare {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

#[derive(PartialEq, Debug)]
enum SquareColor {
    Light,
    Dark,
}

impl ChessSquare {
    fn color(&self) -> SquareColor {
        match self {
            ChessSquare::A1 => SquareColor::Dark,
            ChessSquare::A2 => SquareColor::Light,
            ChessSquare::A3 => SquareColor::Dark,
            ChessSquare::A4 => SquareColor::Light,
            ChessSquare::A5 => SquareColor::Dark,
            ChessSquare::A6 => SquareColor::Light,
            ChessSquare::A7 => SquareColor::Dark,
            ChessSquare::A8 => SquareColor::Light,
            ChessSquare::B1 => SquareColor::Light,
            ChessSquare::B2 => SquareColor::Dark,
            ChessSquare::B3 => SquareColor::Light,
            ChessSquare::B4 => SquareColor::Dark,
            ChessSquare::B5 => SquareColor::Light,
            ChessSquare::B6 => SquareColor::Dark,
            ChessSquare::B7 => SquareColor::Light,
            ChessSquare::B8 => SquareColor::Dark,
            ChessSquare::C1 => SquareColor::Dark,
            ChessSquare::C2 => SquareColor::Light,
            ChessSquare::C3 => SquareColor::Dark,
            ChessSquare::C4 => SquareColor::Light,
            ChessSquare::C5 => SquareColor::Dark,
            ChessSquare::C6 => SquareColor::Light,
            ChessSquare::C7 => SquareColor::Dark,
            ChessSquare::C8 => SquareColor::Light,
            ChessSquare::D1 => SquareColor::Light,
            ChessSquare::D2 => SquareColor::Dark,
            ChessSquare::D3 => SquareColor::Light,
            ChessSquare::D4 => SquareColor::Dark,
            ChessSquare::D5 => SquareColor::Light,
            ChessSquare::D6 => SquareColor::Dark,
            ChessSquare::D7 => SquareColor::Light,
            ChessSquare::D8 => SquareColor::Dark,
            ChessSquare::E1 => SquareColor::Dark,
            ChessSquare::E2 => SquareColor::Light,
            ChessSquare::E3 => SquareColor::Dark,
            ChessSquare::E4 => SquareColor::Light,
            ChessSquare::E5 => SquareColor::Dark,
            ChessSquare::E6 => SquareColor::Light,
            ChessSquare::E7 => SquareColor::Dark,
            ChessSquare::E8 => SquareColor::Light,
            ChessSquare::F1 => SquareColor::Light,
            ChessSquare::F2 => SquareColor::Dark,
            ChessSquare::F3 => SquareColor::Light,
            ChessSquare::F4 => SquareColor::Dark,
            ChessSquare::F5 => SquareColor::Light,
            ChessSquare::F6 => SquareColor::Dark,
            ChessSquare::F7 => SquareColor::Light,
            ChessSquare::F8 => SquareColor::Dark,
            ChessSquare::G1 => SquareColor::Dark,
            ChessSquare::G2 => SquareColor::Light,
            ChessSquare::G3 => SquareColor::Dark,
            ChessSquare::G4 => SquareColor::Light,
            ChessSquare::G5 => SquareColor::Dark,
            ChessSquare::G6 => SquareColor::Light,
            ChessSquare::G7 => SquareColor::Dark,
            ChessSquare::G8 => SquareColor::Light,
            ChessSquare::H1 => SquareColor::Light,
            ChessSquare::H2 => SquareColor::Dark,
            ChessSquare::H3 => SquareColor::Light,
            ChessSquare::H4 => SquareColor::Dark,
            ChessSquare::H5 => SquareColor::Light,
            ChessSquare::H6 => SquareColor::Dark,
            ChessSquare::H7 => SquareColor::Light,
            ChessSquare::H8 => SquareColor::Dark,
        }
    }

    fn is_color_correct(&self, color: &SquareColor) -> bool {
        self.color() == *color
    }
}

impl fmt::Display for ChessSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Distribution<ChessSquare> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ChessSquare {
        match rng.gen_range(0..=63) {
            0 => ChessSquare::A1,
            1 => ChessSquare::A2,
            2 => ChessSquare::A3,
            3 => ChessSquare::A4,
            4 => ChessSquare::A5,
            5 => ChessSquare::A6,
            6 => ChessSquare::A7,
            7 => ChessSquare::A8,
            8 => ChessSquare::B1,
            9 => ChessSquare::B2,
            10 => ChessSquare::B3,
            11 => ChessSquare::B4,
            12 => ChessSquare::B5,
            13 => ChessSquare::B6,
            14 => ChessSquare::B7,
            15 => ChessSquare::B8,
            16 => ChessSquare::C1,
            17 => ChessSquare::C2,
            18 => ChessSquare::C3,
            19 => ChessSquare::C4,
            20 => ChessSquare::C5,
            21 => ChessSquare::C6,
            22 => ChessSquare::C7,
            23 => ChessSquare::C8,
            24 => ChessSquare::D1,
            25 => ChessSquare::D2,
            26 => ChessSquare::D3,
            27 => ChessSquare::D4,
            28 => ChessSquare::D5,
            29 => ChessSquare::D6,
            30 => ChessSquare::D7,
            31 => ChessSquare::D8,
            32 => ChessSquare::E1,
            33 => ChessSquare::E2,
            34 => ChessSquare::E3,
            35 => ChessSquare::E4,
            36 => ChessSquare::E5,
            37 => ChessSquare::E6,
            38 => ChessSquare::E7,
            39 => ChessSquare::E8,
            40 => ChessSquare::F1,
            41 => ChessSquare::F2,
            42 => ChessSquare::F3,
            43 => ChessSquare::F4,
            44 => ChessSquare::F5,
            45 => ChessSquare::F6,
            46 => ChessSquare::F7,
            47 => ChessSquare::F8,
            48 => ChessSquare::G1,
            49 => ChessSquare::G2,
            50 => ChessSquare::G3,
            51 => ChessSquare::G4,
            52 => ChessSquare::G5,
            53 => ChessSquare::G6,
            54 => ChessSquare::G7,
            55 => ChessSquare::G8,
            56 => ChessSquare::H1,
            57 => ChessSquare::H2,
            58 => ChessSquare::H3,
            59 => ChessSquare::H4,
            60 => ChessSquare::H5,
            61 => ChessSquare::H6,
            62 => ChessSquare::H7,
            _ => ChessSquare::H8,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    square: ChessSquare,
    #[serde(skip)]
    result: String,
    current_streak: i32,
    best_streak: i32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            square: rand::random(),
            result: "".to_string(),
            current_streak: 0,
            best_streak: 0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals::dark());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn evaluate_answer(&mut self, answer: &SquareColor) {
        if self.square.is_color_correct(answer) {
            self.result = format!("Correct {} is {:?}", self.square, answer);
            self.current_streak += 1;
        }
        else {
            self.result = format!("Wrong {} is not {:?}", self.square, answer);
            self.current_streak = 0;
        }

        self.best_streak = self.best_streak.max(self.current_streak);

        self.square = rand::random();
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(&self.square.to_string());

            ui.add(Label::new(format!("Current streak is {} best streak is {}", self.current_streak, self.best_streak)));

            if !self.result.is_empty() {
                ui.add(Label::new(&self.result));
            }

            if ui.add(Button::new("Dark")).clicked() {
                self.evaluate_answer(&SquareColor::Dark);
            }

            if ui.add(Button::new("Light")).clicked() {
                self.evaluate_answer(&SquareColor::Light);
            } 

            ui.separator();

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
