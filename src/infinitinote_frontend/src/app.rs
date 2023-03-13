
use std::cell::RefCell;

use eframe::{egui};
use egui_extras::RetainedImage;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, PartialEq)]
enum Navigation {
    SignIn,
    NotebooksHome,
    NotebookAdd,
    Notebook,
    Note,
    Preferences
}

thread_local!(
    static NAVIGATION_PATH: RefCell<Navigation> = RefCell::new(Navigation::SignIn);
);

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = backend)]
    async fn greet(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    fn backend_hello();

    #[wasm_bindgen(js_namespace = window)]
    fn get_backend_result() -> String;

    #[wasm_bindgen(js_namespace = window)]
    async fn backend_promise() -> JsValue;

    #[wasm_bindgen(js_namespace=window)]
    async fn backend_principal() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn call_backend_one(funcName: String) -> JsValue;    

    #[wasm_bindgen(js_namespace = window)]
    async fn get_notebooks_for_principal(principalId: String) -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn get_notes_for_notebook(principalId: String, notebook_id: String) -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn add_notebook_for_principal(principal_id: String, notebook_title: String);

    #[wasm_bindgen(js_namespace = window)]
    async fn add_note_to_notebook(principal_id: String, notebook_id: String, note_title: String, note_text: String);

    
}

async fn test_yellow() {
    log(&"Testing Yellow".to_string());

    let result = backend_promise().await;

    log(&result.as_string().unwrap());
}

async fn test_purple() {
    log(&"Testing Purple".to_string());

    let result = backend_principal().await;

    log(&result.as_string().unwrap())
}

async fn test_green() {
    log("TESTING GREEN");
    let my_name = "Hafez";

    //greet(&my_name).await;

    // let backend: Principal = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    // let call_result: Result<(), _> = ic_cdk::call(backend, "test_greet_", ()).await;

    // if call_result.is_ok()
    // {
    //     let call_result_option = call_result.ok();
    //     if call_result_option.is_some()
    //     {
    //         let result  = call_result_option.unwrap();
    //         log("AWESOME CALL DUDE");
    //     }
    // }
    // else
    // {
    //     log("error occured");
    // }
    
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)]
    logo_image: RetainedImage,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

trait RenderMethods {
    fn render_sign_in(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
    fn render_notebooks_home(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
    fn render_notebook_add(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
    fn render_notebook(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            logo_image: RetainedImage::from_image_bytes(
                "../assets/ui/logo.png",
                include_bytes!("../assets/ui/logo.png"),
            )
            .unwrap(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn navigate(nav: &mut Navigation, to: Navigation)
    {
        *nav = to;
    }

    fn render_header(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        egui::TopBottomPanel::top("header")
        .show_separator_line(false)
        .frame(bg_frame)
        .show(ctx, |ui|
        {
            ui.add(egui::ImageButton::new(
                self.logo_image.texture_id(ctx),
                self.logo_image.size_vec2(),
            ));
        });
    }

    fn render_sign_in(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let bg_color = egui::Color32::from_rgb(237, 239, 243);
        let bg_frame = egui::Frame::none().fill(bg_color);

        egui::CentralPanel::default().frame(bg_frame)
        .show(ctx, |ui| {


            if ui.button("Sign In").clicked() {
                log(&"PURPLE".to_string());
                
                Self::navigate(nav, Navigation::NotebooksHome);
            }
        });
    }

    fn render_notebooks_home(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let bg_color = egui::Color32::from_rgb(237, 239, 243);
        let bg_frame = egui::Frame::none().fill(bg_color);      

        egui::CentralPanel::default().frame(bg_frame).show(ctx, |ui| {
            if ui.button("Notebooks").clicked() {
                log(&"GREEN".to_string());
            }

            if ui.button("Add Notebook").clicked() {
                log(&"GREEN".to_string());
                Self::navigate(nav, Navigation::NotebookAdd);
            }

            if ui.button("Yellow Notebook").clicked() {
                log(&"GREEN".to_string());
                Self::navigate(nav, Navigation::Notebook);
            }
        });
    }

    fn render_notebook_add(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Notebook Add").clicked() {
                log(&"Notebook Add".to_string());
            }

            if ui.button("Finish Notebook Add").clicked() {
                log(&"Notebook Add".to_string());
                Self::navigate(nav, Navigation::NotebooksHome);
            }
        });
    }

    fn render_notebook(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Back").clicked() {
                log(&"Notebook".to_string());
                Self::navigate(nav, Navigation::NotebooksHome);
            }

            if ui.button("Notebook").clicked() {
                log(&"Notebook".to_string());
            }
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, logo_image, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        NAVIGATION_PATH.with(|navigation| {
            let mut nav_mut = navigation.borrow_mut();

            self.render_header(ctx, _frame, &mut nav_mut);

            if *nav_mut == Navigation::SignIn
            {
                self.render_sign_in(ctx, _frame, &mut nav_mut)
            }
            else if *nav_mut == Navigation::NotebooksHome
            {
                self.render_notebooks_home(ctx, _frame, &mut nav_mut);
            }
            else if *nav_mut == Navigation::NotebookAdd
            {
                self.render_notebook_add(ctx, _frame, &mut nav_mut);
            }
            else if *nav_mut == Navigation::Notebook
            {
                self.render_notebook(ctx, _frame, &mut nav_mut);
            }
        });


        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
        //     ui.heading("Side Panel");

        //     // ui.horizontal(|ui| {
        //     //     ui.label("Write something: ");
        //     //     ui.text_edit_singleline(label);
        //     // });

        //     // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //     // if ui.button("Increment").clicked() {
        //     //     *value += 1.0;
        //     // }

        //     if ui.button("Greet").clicked() {
        //         let g = greet("Hafez".to_string());
        //         ui.label("greet");
        //     }

        //     ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        //         ui.horizontal(|ui| {
        //             ui.spacing_mut().item_spacing.x = 0.0;
        //             ui.label("powered by ");
        //             ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        //             ui.label(" and ");
        //             ui.hyperlink_to(
        //                 "eframe",
        //                 "https://github.com/emilk/egui/tree/master/crates/eframe",
        //             );
        //             ui.label(".");
        //         });
        //     });
        // });

        // egui::CentralPanel::default().show(ctx, |ui| {
        //     // The central panel the region left after adding TopPanel's and SidePanel's
        //     // ui.heading("TEST TEST TEST");
        //     // ui.heading("infinitinote");
        //     // ui.hyperlink("https://github.com/hafezrouzati/infinitinote");
        //     // ui.add(egui::github_link_file!(
        //     //     "https://github.com/hafezrouzati/infinitinote/",
        //     //     "Source code."
        //     // ));

        //     if ui.button("Greet Me").clicked() {
        //         //let g = greet("Hafez".to_string());
        //         //ui.label("greet");
        //         log(&"GREEN".to_string());
        //         backend_hello();
        //     }

        //     if ui.button("Get Result").clicked() {
        //         //let g = greet("Hafez".to_string());
        //         //ui.label("greet");
        //         log(&"YELLOW".to_string());
        //         let r = get_backend_result();
        //         log(&r);
        //     }

        //     if ui.button("Yellow").clicked() {
        //         log(&"MELLOW YELLOW2".to_string());
        //         //let g = greet("Hafez".to_string());
        //         //ui.label("greet");
        //         spawn_local(
        //             test_yellow()
        //         );
                
        //     }

        //     if ui.button("Purple").clicked() {
        //         log(&"Purple".to_string());
        //         //let g = greet("Hafez".to_string());
        //         //ui.label("greet");
        //         spawn_local(
        //             test_purple()
        //         );
                
        //     }

        //     if ui.button("Banana").clicked() {
        //         log(&"Banana".to_string());
        //         //let g = greet("Hafez".to_string());
        //         //ui.label("greet");
        //         spawn_local( async {
        //                 let result = call_backend_one("greet".to_string()).await;
        //                 log(&result.as_string().unwrap());
        //             }
        //         );
                
        //     }
            
        //     // ui.horizontal(|ui| {
        //     //     ui.label("Write something: ");
        //     //     ui.text_edit_multiline(label);
        //     // });

        //     egui::warn_if_debug_build(ui);
        // });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
