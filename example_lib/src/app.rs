use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use egui::{Color32, TextStyle, TextureHandle};

type ImageHashMap = Arc<Mutex<HashMap<String, Option<TextureHandle>>>>;
const FONT_TABLE_TITLE: egui::FontId = egui::FontId {
    size: 32.0,
    family: egui::FontFamily::Proportional,
};

#[derive(Clone)]
struct Game {
    image_url: String,
    name: String,
    platform: Vec<String>,
    issue_date: String,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    inspection: bool,
    images: ImageHashMap,
    games: Vec<Game>,
    my_image: Option<TextureHandle>,
}

impl App {
    pub fn new() -> Self {
        Self {
            inspection: false,
            images: Default::default(),
            games: Vec::default(),
            my_image: Default::default(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "rust-web-server-egui"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        self.my_image = Some(_ctx.load_texture("my_image", egui::ColorImage::example()));

        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../../font/SourceHanSerifCN-Regular.otf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        // Tell egui to use these fonts:
        _ctx.set_fonts(fonts);

        let games = [
            Game {
                image_url: "https://picsum.photos/seed/picsum/184/69".to_string(),
                name: "异界深渊：觉醒".to_string(),
                platform: vec!["WEB".to_string()],
                issue_date: "2022-02-02".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/185/69".to_string(),
                name: "异界深渊：觉醒".to_string(),
                platform: vec!["WEB".to_string()],
                issue_date: "2022-02-02".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/186/69".to_string(),
                name: "Cyberpunk".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/187/69".to_string(),
                name: "Dysmantle".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/188/69".to_string(),
                name: "DeathTrash".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/189/69".to_string(),
                name: "it-takes-two".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/190/69".to_string(),
                name: "assassins-creed-valhalla".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/191/69".to_string(),
                name: "art-of-rally".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/192/69".to_string(),
                name: "doom-eternal".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/193/69".to_string(),
                name: "fifa-21".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/194/69".to_string(),
                name: "genshin-impact".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/195/69".to_string(),
                name: "nba-2k21".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/196/69".to_string(),
                name: "super-meat-boy-forever".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
            Game {
                image_url: "https://picsum.photos/seed/picsum/197/69".to_string(),
                name: "the-dungeon-of-naheulbeuk-the-amulet-of-chaos".to_string(),
                platform: vec!["XBOX".to_string(), "PS5".to_string()],
                issue_date: "2020-01-01".to_string(),
            },
        ];

        self.games = games.to_vec();

        for game in games {
            download_image(game.image_url, _ctx, _frame, Arc::clone(&self.images));
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                if cfg!(debug_assertions) {
                    ui.menu_button("Debug", |ui| {
                        if ui.button("Inspection UI").clicked() {
                            self.inspection = true
                        }
                    });
                }
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            egui::CollapsingHeader::new("Movies")
                .default_open(false)
                .show(ui, |ui| {
                    if ui.button("Action").clicked() {}
                    if ui.button("Comedy").clicked() {}
                    if ui.button("Adventure").clicked() {}
                });
            egui::CollapsingHeader::new("TVs")
                .default_open(false)
                .show(ui, |ui| {
                    if ui.label("BTV").clicked() {}
                    if ui.button("MTV").clicked() {}
                    if ui.button("FTV").clicked() {}
                    if ui.button("WTV").clicked() {}
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            ui.style_mut().override_text_style = Some(TextStyle::Heading);

            if cfg!(debug_assertions) {
                egui::Window::new("🔍 Inspection")
                    .open(&mut self.inspection)
                    .vscroll(true)
                    .show(ctx, |ui| {
                        ctx.inspection_ui(ui);
                    });
            }

            egui::ScrollArea::horizontal()
                .id_source("scroll_images")
                .always_show_scroll(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for image_tex_opt in self.images.lock().unwrap().values() {
                            if let Some(image_tex) = image_tex_opt {
                                ui.image(image_tex, image_tex.size_vec2());
                            }
                        }
                    });
                });

            egui::ScrollArea::vertical()
                .id_source("scroll_grid")
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        .striped(true)
                        .spacing([40.0, 4.0])
                        // .min_col_width(10.0)
                        // .max_col_width(200.0)
                        .show(ui, |ui| {
                            for game in &self.games {
                                let image_tex = self.images.lock().unwrap();
                                if image_tex.contains_key(&game.image_url) {
                                    let tex =
                                        image_tex.get(&game.image_url).unwrap().as_ref().unwrap();
                                    ui.image(tex, tex.size_vec2());
                                } else {
                                    if let Some(image) = &self.my_image {
                                        ui.image(image, image.size_vec2());
                                    }
                                }
                                ui.label(
                                    egui::RichText::new(game.name.clone())
                                        .color(Color32::BLUE)
                                        .font(FONT_TABLE_TITLE),
                                );
                                ui.label(game.platform.join(", "));
                                ui.label(game.issue_date.clone());
                                ui.end_row();
                            }
                        });
                });
        });
    }
}

fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    // use image::GenericImageView as _;
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn parse_image(ctx: &egui::Context, url: String, data: &[u8]) -> Option<TextureHandle> {
    let image = load_image(data).ok();
    image.map(|image| ctx.load_texture(url, image))
}

fn download_image(
    image_url: String,
    ctx: &egui::Context,
    frame: &epi::Frame,
    images: ImageHashMap,
) {
    let ctx2 = ctx.clone();
    let frame2 = frame.clone();
    ehttp::fetch(ehttp::Request::get(image_url.clone()), move |r| {
        if let Ok(r) = r {
            let data = r.bytes;
            if let Some(handle) = parse_image(&ctx2, image_url.clone(), &data) {
                if images.lock().unwrap().get(&image_url).is_none() {
                    images.lock().unwrap().insert(image_url, Some(handle));
                }
                frame2.request_repaint();
            }
        }
    });
}
