use eframe::egui;

#[derive(Default)]
struct MyApp {
    clicked: bool,
    da_input: String,
    cursor: usize,
}

pub fn run_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Testing",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |_| {
        egui::Window::new("test").resizable(true).show(ctx, |ui| {
            ui.label("show da window");
            let _  = ui.add(egui::TextEdit::multiline(&mut self.da_input));
            if ui.button("show").clicked() {
                self.clicked = !self.clicked;
            }

        });

        if !self.clicked {
            egui::Window::new("test2").show(ctx, |ui| {
                let mut lines: Vec<String> = self.da_input
                    .lines()
                    .map(|s| s.to_string())
                    .collect();

                if ui.button(">").clicked() {
                    self.cursor += 1;
                    if self.cursor >= lines.len() {
                        self.cursor = 0;
                    }
                }

                if lines.len() > self.cursor {
                    let current = lines[self.cursor].to_string();
                    lines[self.cursor] = format!("> {current}");
                }

                let mut textbox_string = lines.join("\n");
                ui.code_editor(&mut textbox_string);
                ui.label("crash");
                if ui.button("123").clicked() {
                    panic!()
                }
            });
        }
    }
}
