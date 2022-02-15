use poll_promise::Promise;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    url: String,
    texture_id: Option<egui::TextureId>,
    // #[cfg_attr(feature = "serde", serde(skip))]
    promise: Option<Promise<ehttp::Result<Resource>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: "https://raw.githubusercontent.com/emilk/egui/master/README.md".to_owned(),
            texture_id: None,
            promise: Default::default(),
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

            // Show the image:
            let t_id;

            if let Some(tt) = self.texture_id {
                t_id = tt;
                println!("id: {:?}", t_id);
                ui.horizontal_wrapped(|ui| {
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                    ui.image(tt, egui::Vec2::new(64.0, 64.0));
                });
            } else {
                let texture: &egui::TextureHandle = &ui
                    .ctx()
                    .load_texture("my-image", egui::ColorImage::example());
                t_id = texture.id();
                ui.horizontal_wrapped(|ui| {
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                    ui.image(texture, texture.size_vec2());
                });
            }

            ui.separator();

            let trigger_fetch = ui_url(ui, frame, &mut self.url);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("HTTP requests made using ");
                ui.hyperlink_to("ehttp", "https://www.github.com/emilk/ehttp");
                ui.label(".");
            });

            if trigger_fetch {
                let ctx = ctx.clone();
                let frame = frame.clone();
                let (sender, promise) = Promise::new();
                let request = ehttp::Request::get(&self.url);
                println!("aaaaaaa");
                ctx.tex_manager().write().free(t_id);
                ehttp::fetch(request, move |response| {
                    frame.request_repaint(); // wake up UI thread
                    println!("bbbbbbb");
                    let resource = response.map(|response| Resource::from_response(&ctx, response));
                    println!("bbbbbbb1");
                    sender.send(resource);
                    println!("bbbbbbb2");
                });
                self.promise = Some(promise);
            }

            ui.separator();

            if let Some(promise) = &self.promise {
                if let Some(result) = promise.ready() {
                    match result {
                        Ok(resource) => {
                            println!("ccccccc");
                            let t = resource.texture.as_ref();
                            println!("ddddddd");
                            self.texture_id = Some(t.unwrap().id());
                            println!("eeeeeee");
                            ui_resource(ui, resource);
                            println!("fffffff");
                        }
                        Err(error) => {
                            // This should only happen if the fetch API isn't available or something similar.
                            ui.colored_label(
                                egui::Color32::RED,
                                if error.is_empty() { "Error" } else { error },
                            );
                        }
                    }
                } else {
                    ui.add(egui::Spinner::new());
                }
            }
            // egui::ScrollArea::vertical()
            //     .auto_shrink([false, true])
            //     .show(ui, |ui| {
            //         egui::Grid::new("my_grid")
            //             .num_columns(6)
            //             .striped(true)
            //             .spacing([40.0, 4.0])
            //             // .min_col_width(10.0)
            //             // .max_col_width(200.0)
            //             .show(ui, |ui| {
            //                 for row in 0..100 {
            //                     for col in 0..6 {
            //                         if col == 0 {
            //                             ui.label(format!("row {}", row));
            //                         } else {
            //                             ui.label(format!("col {}", col));
            //                         }
            //                     }
            //                     ui.end_row();
            //                 }
            //             });
            //     });
        });
    }
}

fn ui_url(ui: &mut egui::Ui, frame: &epi::Frame, url: &mut String) -> bool {
    let mut trigger_fetch = false;

    ui.horizontal(|ui| {
        ui.label("URL:");
        trigger_fetch |= ui
            .add(egui::TextEdit::singleline(url).desired_width(f32::INFINITY))
            .lost_focus();
    });

    if frame.is_web() {
        ui.label("HINT: paste the url of this page into the field above!");
    }

    ui.horizontal(|ui| {
        if ui.button("Source code for this example").clicked() {
            *url = format!(
                "https://raw.githubusercontent.com/emilk/egui/master/{}",
                file!()
            );
            trigger_fetch = true;
        }
        if ui.button("Random image").clicked() {
            let seed = ui.input().time;
            let side = 640;
            *url = format!("https://picsum.photos/seed/{}/{}", seed, side);
            trigger_fetch = true;
        }
    });

    trigger_fetch
}

fn ui_resource(ui: &mut egui::Ui, resource: &Resource) {
    let Resource {
        response,
        text,
        texture,
    } = resource;

    ui.monospace(format!("url:          {}", response.url));
    ui.monospace(format!(
        "status:       {} ({})",
        response.status, response.status_text
    ));
    ui.monospace(format!(
        "content-type: {}",
        response.content_type().unwrap_or_default()
    ));
    ui.monospace(format!(
        "size:         {:.1} kB",
        response.bytes.len() as f32 / 1000.0
    ));

    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            egui::CollapsingHeader::new("Response headers")
                .default_open(false)
                .show(ui, |ui| {
                    egui::Grid::new("response_headers")
                        .spacing(egui::vec2(ui.spacing().item_spacing.x * 2.0, 0.0))
                        .show(ui, |ui| {
                            for header in &response.headers {
                                ui.label(header.0);
                                ui.label(header.1);
                                ui.end_row();
                            }
                        })
                });

            ui.separator();

            if let Some(text) = &text {
                let tooltip = "Click to copy the response body";
                if ui.button("📋").on_hover_text(tooltip).clicked() {
                    ui.output().copied_text = text.clone();
                }
                ui.separator();
            }

            if let Some(texture) = texture {
                let mut size = texture.size_vec2();
                size *= (ui.available_width() / size.x).min(1.0);
                ui.image(texture, size);
            } else if let Some(text) = &text {
                // selectable_text(ui, text);
            } else {
                ui.monospace("[binary]");
            }
        });
}

struct Resource {
    /// HTTP response
    response: ehttp::Response,

    text: Option<String>,

    /// If set, the response was an image.
    texture: Option<egui::TextureHandle>,
}

impl Resource {
    fn from_response(ctx: &egui::Context, response: ehttp::Response) -> Self {
        println!("xxxxxxxx");
        let content_type = response.content_type().unwrap_or_default();
        println!("yyyyyyyy");
        let image = if content_type.starts_with("image/") {
            println!("zzzzzzzz");
            load_image(&response.bytes).ok()
        } else {
            None
        };

        // let texture = image.map(|image| ctx.load_texture(&response.url, image));
        println!("11111111");
        let texture = image.map(|image| {
            // ctx.load_texture("my-image", image.clone());
            ctx.load_texture(&response.url, image)
        });

        println!("load texture");

        println!("write texture");
        let text = response.text();
        let text = text.map(|text| text.to_owned());

        Self {
            response,
            text,
            texture,
        }
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
