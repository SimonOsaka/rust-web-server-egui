use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use egui::{vec2, Color32, Context, TextStyle, TextureHandle, Ui};

type ImageHashMap = Arc<Mutex<HashMap<String, Option<TextureHandle>>>>;
const FONT_TABLE_TITLE: egui::FontId = egui::FontId {
    size: 32.0,
    family: egui::FontFamily::Proportional,
};

#[derive(Clone)]
struct ImageSize {
    image_width: f32,
    image_height: f32,
}

#[derive(Clone)]
struct Image {
    image_url: String,
    image_size: Option<ImageSize>,
}
#[derive(Clone)]
struct Movie {
    image: Image,
    name: String,
    platform: Vec<String>,
    issue_date: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum LeftMenu {
    Action,
    Comedy,
    Adventure,
    BTV,
    MTV,
    FTV,
    WTV,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    inspection: bool,
    images: ImageHashMap,
    movies: Vec<Movie>,
    my_image: Option<TextureHandle>,
    left_menu: LeftMenu,
}

impl App {
    pub fn new() -> Self {
        Self {
            inspection: false,
            images: Default::default(),
            movies: Vec::default(),
            my_image: Default::default(),
            left_menu: LeftMenu::Action,
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

        self.movies = data_action();

        store_movies(self, _ctx, _frame);
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
                    if ui
                        .selectable_value(&mut self.left_menu, LeftMenu::Action, "Action")
                        .clicked()
                    {
                        self.movies = data_action();
                        store_movies(self, ctx, frame);
                    };
                    if ui
                        .selectable_value(&mut self.left_menu, LeftMenu::Comedy, "Comedy")
                        .clicked()
                    {
                        self.movies = data_comedy();
                        store_movies(self, ctx, frame);
                    }
                    ui.selectable_value(&mut self.left_menu, LeftMenu::Adventure, "Adventure");
                });
            egui::CollapsingHeader::new("TVs")
                .default_open(false)
                .show(ui, |ui| {
                    ui.selectable_value(&mut self.left_menu, LeftMenu::BTV, "BTV");
                    ui.selectable_value(&mut self.left_menu, LeftMenu::MTV, "MTV");
                    ui.selectable_value(&mut self.left_menu, LeftMenu::FTV, "FTV");
                    ui.selectable_value(&mut self.left_menu, LeftMenu::WTV, "WTV");
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

            match self.left_menu {
                LeftMenu::Action => display_action(ui, self),
                LeftMenu::Comedy => display_action(ui, self),
                LeftMenu::Adventure => todo!(),
                LeftMenu::BTV => todo!(),
                LeftMenu::MTV => todo!(),
                LeftMenu::FTV => todo!(),
                LeftMenu::WTV => todo!(),
            };
        });
    }
}

fn display_action(ui: &mut Ui, app: &mut App) {
    egui::ScrollArea::horizontal()
        .id_source("scroll_images")
        .always_show_scroll(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for image_tex_opt in app.images.lock().unwrap().values() {
                    if let Some(image_tex) = image_tex_opt {
                        ui.image(image_tex, vec2(115.0, 162.0));
                    }
                }
            });
        });

    egui::ScrollArea::vertical()
        .id_source("scroll_grid")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("my_grid")
                .num_columns(3)
                .striped(true)
                .spacing([40.0, 4.0])
                // .min_col_width(10.0)
                // .max_col_width(200.0)
                .show(ui, |ui| {
                    for game in &app.movies {
                        let image_tex = app.images.lock().unwrap();
                        if image_tex.contains_key(&game.image.image_url) {
                            let tex = image_tex
                                .get(&game.image.image_url)
                                .unwrap()
                                .as_ref()
                                .unwrap();
                            ui.image(
                                tex,
                                match &game.image.image_size {
                                    Some(size) => egui::vec2(size.image_width, size.image_height),
                                    None => tex.size_vec2(),
                                },
                            );
                        } else {
                            if let Some(image) = &app.my_image {
                                ui.image(image, image.size_vec2());
                            }
                        }
                        ui.label(
                            egui::RichText::new(game.name.clone())
                                .color(Color32::BLUE)
                                .font(FONT_TABLE_TITLE),
                        );
                        ui.horizontal(|ui| {
                            for p in game.platform.clone() {
                                ui.label(
                                    egui::RichText::new(p).background_color(Color32::LIGHT_GRAY),
                                );
                            }
                        });
                        ui.label(game.issue_date.clone());
                        ui.end_row();
                    }
                });
        });
}

fn data_action() -> Vec<Movie> {
    [
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p462657443.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Dark Knight".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2008-07-18".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p2209718348.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Hunger Games".to_string(),
            platform: vec!["amazon prime".to_string()],
            issue_date: "2012-03-23".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p2279945831.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Revenant".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2016-01-08".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p858079649.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "King Kong".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2005-12-14".to_string(),
        },
    ]
    .to_vec()
}

fn data_comedy() -> Vec<Movie> {
    [
        Movie {
            image: Image {
                image_url: String::from("http://localhost:8080/images/p735379215.jpg"),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Devil Wears Prada".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2006-06-30".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p792443418.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "Lock, Stock and Two Smoking Barrels".to_string(),
            platform: vec!["netflix".to_string(), "amazon prime".to_string()],
            issue_date: "1998-08-28".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p854757687.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Terminal".to_string(),
            platform: vec!["amazon prime".to_string()],
            issue_date: "2004-06-18".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p1454261925.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "Intouchables".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2011-11-02".to_string(),
        },
        Movie {
            image: Image {
                image_url: "http://localhost:8080/images/p2160254162.jpg".to_string(),
                image_size: Some(ImageSize {
                    image_width: 115.0,
                    image_height: 164.0,
                }),
            },
            name: "The Wolf of Wall Street".to_string(),
            platform: vec!["netflix".to_string()],
            issue_date: "2013-12-25".to_string(),
        },
    ]
    .to_vec()
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

fn store_movies(app: &mut App, _ctx: &Context, _frame: &epi::Frame) {
    for movie in &app.movies {
        download_image(
            movie.image.image_url.clone(),
            _ctx,
            _frame,
            Arc::clone(&app.images),
        );
    }
}
