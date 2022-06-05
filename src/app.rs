use poll_promise::Promise;

use crate::cheater;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    green_input: String,
    orange_input: String,
    tried_letters: String,
    solns: String,
    solns_by_resulting_entropy: String,
    all_by_resulting_entropy: String,
    #[serde(skip)]
    promise: Option<Promise<cheater::CheaterResult>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            green_input: ".....".to_owned(),
            orange_input: ". . . . .".to_owned(),
            tried_letters: "raise".to_owned(),
            solns: "".to_owned(),
            solns_by_resulting_entropy: "".to_owned(),
            all_by_resulting_entropy: "".to_owned(),
            promise: Option::<Promise<cheater::CheaterResult>>::None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        cc.egui_ctx.set_pixels_per_point(35.0);

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { green_input, orange_input, tried_letters, solns,
            solns_by_resulting_entropy, all_by_resulting_entropy,promise: _} = self;

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
            ui.heading("Input");

            ui.horizontal(|ui| {
                ui.label("Green String");
                ui.text_edit_singleline(green_input);
            });


            ui.horizontal(|ui| {
                ui.label("Orange String");
                ui.text_edit_singleline(orange_input);
            });

            ui.horizontal(|ui| {
                ui.label("Tried Letters");
                ui.text_edit_multiline(tried_letters);
            });

            

            if ui.button("Calculate").clicked() {
                     let s1:String = (*green_input).clone().to_string();
                    let s2 = (*orange_input).clone().to_string();
                    let s3 = (*tried_letters).clone().to_string();
                     //*promise = Some(Promise::spawn_thread("looking at options",move ||  
                    // {
                    ui.label("Calculating ...");
                    let res = cheater::cheat(s1,s2,s3);
                    *solns = res.possiblewords.clone();
                    *solns_by_resulting_entropy = res.possibleByEntropy.clone();
                    *all_by_resulting_entropy = res.allByEntropy.clone();
                    println!("{:?}",solns)
            }


            // match promise {
            //     None => if ui.button("Calculate").clicked() {
            //         println!("ouch");
            //         let s1:String = (*green_input).clone().to_string();
            //         let s2 = (*orange_input).clone().to_string();
            //         let s3 = (*tried_letters).clone().to_string();
            //         *solns = String::from("Calculating");
            //         *solns_by_resulting_entropy = String::from("Calculating");
            //         *all_by_resulting_entropy = String::from("Calculating");
            //         *promise = Some(Promise::spawn_thread("looking at options",move ||  
            //         {
            //             cheater::cheat(s1,s2,s3)
            //         }
            //     ))
            //     },
            //     Some(p) => match p.ready() {
            //         None => {ui.label("Calculating");},
            //         Some(r) => {
            //             let res: &cheater::CheaterResult= r;
            //             *solns = res.possiblewords.clone();
            //             *solns_by_resulting_entropy = res.possibleByEntropy.clone();
            //             *all_by_resulting_entropy = res.allByEntropy.clone();

            //             *promise = None;
            //             //println!("{:?}",r.possiblewords)},
            //         },
            //     },                
            // }            
             


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Output");
            ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Possible Solutions");
                        ui.text_edit_multiline(solns);
                    });

                    ui.vertical(|ui| {
                        ui.label("Possible Solutions by Resulting Entropy");
                        ui.text_edit_multiline(solns_by_resulting_entropy);
                    });

                    ui.vertical(|ui| {
                        ui.label("All words by resulting entropy");
                        ui.text_edit_multiline(all_by_resulting_entropy);
                    });
                });
            });
            
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
