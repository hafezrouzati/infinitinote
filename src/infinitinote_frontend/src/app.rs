use std::fmt;
use std::cell::RefCell;

use js_sys::Object;

use eframe::{egui};
use egui::text_edit::TextEditState;
use egui_extras::RetainedImage;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use serde::Serialize;
use serde::Deserialize;
use serde_wasm_bindgen;

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
    async fn authenticate();

    #[wasm_bindgen(js_namespace = window)]
    fn getUserPrincipal() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn add_notebook_for_principal(notebook_title: String);

    #[wasm_bindgen(js_namespace = window)]
    async fn get_notebooks_for_principal() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    fn get_notebooks() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn get_notes_for_notebook(principalId: String, notebook_id: String) -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn add_note_to_notebook(notebook_id: String, note_title: String, note_text: String);

    #[wasm_bindgen(js_namespace = window)]
    async fn update_note(notebook_id: String, note_id: String, note_title: String, note_text: String);

}

#[wasm_bindgen]
pub fn test_bind () 
{
    log(&"Success");
}

#[derive(Serialize, Deserialize)]
pub struct UUID(pub String);

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AssetID(pub String);

#[derive(Serialize, Deserialize)]
struct Note
{
    pub id: UUID,
    pub title: String,
    pub content: String,
    pub attachments: Vec<String>,
    pub tags: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Notebook 
{
    pub id: UUID,
    pub title: String,
    pub notes: Vec<Note>,
    pub tags: Vec<String>
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    principal_id: String,

    #[serde(skip)]
    notebooks : Vec<Notebook>,
    
    #[serde(skip)]
    selected_notebook : usize,

    #[serde(skip)]
    selected_note : usize,

    // UI Images

    #[serde(skip)]
    logo_image: RetainedImage,

    //#[serde(skip)]
    //add_notebook_btn_img: RetainedImage,

    #[serde(skip)]
    add_notebook_text : String,
    
    #[serde(skip)]
    add_note_title_text : String,

    #[serde(skip)]
    add_note_text : String,

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
            principal_id: "".to_owned(),
            notebooks: Vec::new(),
            selected_notebook: 0,
            selected_note: 777,
            logo_image: RetainedImage::from_image_bytes(
                "../assets/ui/logo.png",
                include_bytes!("../assets/ui/logo.png"),
            )
            .unwrap(),
            // add_notebook_btn_img: RetainedImage::from_image_bytes(
            //     "add_button.jpg",
            //     include_bytes!("../assets/ui/in_logo.jpg"),
            // )
            // .unwrap(),
            add_notebook_text: "".to_string(),
            add_note_title_text: "".to_string(),
            add_note_text: "".to_string(),
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

    pub fn set_principal_id(&mut self, id: String)
    {
        self.principal_id = id;
        log(&"Set Principal");
    }

    fn navigate(nav: &mut Navigation, to: Navigation)
    {
        *nav = to;
    }

    fn render_header(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let bg_color = egui::Color32::from_rgb(237, 239, 243);
        let bg_frame = egui::Frame::none().fill(bg_color);

        egui::TopBottomPanel::top("header")
        .show_separator_line(false)
        .frame(bg_frame)
        .show(ctx, |ui|
        {
            // ui.add(egui::ImageButton::new(
            //     self.logo_image.texture_id(ctx),
            //     self.logo_image.size_vec2(),
            // ));
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
                spawn_local( async {
                    authenticate().await;
                })
            }

            if ui.button("Confirm Sign In").clicked() {
                let result = getUserPrincipal();
                let principal = result.as_string().unwrap();
                log(&principal);
                self.set_principal_id(principal);
                Self::navigate(nav, Navigation::NotebooksHome);
            }

            if ui.button("Test Sign In").clicked() {
            }
        });
    }

    fn load_notebooks_for_principal(&mut self)
    {
        log(&"load".to_string());
       
        spawn_local( async {
                let result = get_notebooks_for_principal().await;
            }
        );

    }

    fn load_notebooks(&mut self)
    {
        let result = get_notebooks();
        self.notebooks = serde_wasm_bindgen::from_value(result).unwrap();
    }

    fn render_notebooks_home(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let bg_color = egui::Color32::from_rgb(237, 239, 243);
        let bg_frame = egui::Frame::none().fill(bg_color);      

        egui::CentralPanel::default().frame(bg_frame).show(ctx, |ui| {
            if ui.button("Notebooks").clicked() {
                log(&"GREEN".to_string());
                self.load_notebooks_for_principal();
            }

            if ui.button("Get notebooks").clicked() {
                log(&"GREEN".to_string());
                self.load_notebooks();
            }

            //for notebook in &self.notebooks
            for (i, notebook) in self.notebooks.iter().enumerate() 
            {
                if ui.button(&notebook.title).clicked() {
                    self.selected_notebook = i;
                    Self::navigate(nav, Navigation::Notebook);
                }
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
        let bg_color = egui::Color32::from_rgb(237, 239, 243);
        let bg_frame = egui::Frame::none().fill(bg_color);  

        egui::CentralPanel::default().frame(bg_frame).show(ctx, |ui| {
            ui.heading("Add Notebook");
            if ui.button("Notebook Add").clicked() {
                log(&"Notebook Add".to_string());
            }

            ui.horizontal(|ui| {
                ui.label("Notebook Title: ");
                ui.add(egui::TextEdit::singleline(&mut self.add_notebook_text).hint_text("Enter A Notebook Title"));
            });

            if ui.button("Finish Notebook Add").clicked() {
                log(&"Notebook Add".to_string());
                spawn_local(
                    add_notebook_for_principal(self.add_notebook_text.clone())
                );
                Self::navigate(nav, Navigation::NotebooksHome);
            }
        });
    }

    fn render_notebook(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let notebook = &self.notebooks[self.selected_notebook];

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Back").clicked() {
                log(&"Notebook".to_string());
                Self::navigate(nav, Navigation::NotebooksHome);
            }

            ui.label(&notebook.title);

            if ui.button("Add Note").clicked() {
                Self::navigate(nav, Navigation::Note);
                log(&"Notebook".to_string());
            }

            for (i, note) in notebook.notes.iter().enumerate() 
            {
                if ui.button(&note.title).clicked() {
                    self.selected_note = i;
                    Self::navigate(nav, Navigation::Note);
                }
            }


        });
    }

    fn render_note_edit(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, nav: &mut Navigation)
    {
        let notebook = &self.notebooks[self.selected_notebook];

        egui::CentralPanel::default().show(ctx, |ui| {
                
            if ui.button("Save Note").clicked()
            {
                if self.selected_note == 777
                {
                    spawn_local( 
                            add_note_to_notebook(notebook.id.to_string(), self.add_note_title_text.clone(), self.add_note_text.clone())
                    );
                }
                else 
                {
                    let note = &notebook.notes[self.selected_note];
                    spawn_local(
                            update_note(notebook.id.to_string(), note.id.to_string(), self.add_note_title_text.clone(), self.add_note_text.clone())
                    );
                }
            }

            ui.label(&notebook.title);
            ui.label("/");
            ui.text_edit_singleline(&mut self.add_note_title_text);

            ui.text_edit_multiline(&mut self.add_note_text);
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
        let Self { 
            label, 
            principal_id,
            notebooks,
            selected_notebook,
            selected_note,
            logo_image, 
            value, 
            add_notebook_text,
            add_note_title_text,
            add_note_text,
        } = self;

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
            else if *nav_mut == Navigation::Note
            {
                self.render_note_edit(ctx, _frame, &mut nav_mut);
            }

        });

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
