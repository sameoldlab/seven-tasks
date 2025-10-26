use state::time;
use state::{FlightBooker, Tabs, TripType, from_fahrenheit, to_fahrenheit};

pub struct App {
    count: i32,
    page: Tabs,
    /// temperature in celsius
    temp: f32,
    elapsed_time: std::time::Instant,
    total_time: f64,
    fb: FlightBooker,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        App {
            count: 1,
            page: Tabs::TempConverter,
            temp: 27.,
            elapsed_time: std::time::Instant::now(),
            total_time: 30.0,
            fb: FlightBooker::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                for (i, t) in [
                    ("Counter", Tabs::Counter),
                    ("Temp Converter", Tabs::TempConverter),
                    ("Flight Booker", Tabs::FlightBooker),
                    ("Timer", Tabs::Timer),
                    ("CRUD", Tabs::Crud),
                    ("Circles", Tabs::Circles),
                    ("Cells", Tabs::Cells),
                ] {
                    if ui.button(i).clicked() {
                        self.page = t;
                        if t == Tabs::Timer {
                            self.elapsed_time = std::time::Instant::now();
                        }
                    }
                    ui.add_space(16.0);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");
            match self.page {
                Tabs::Counter => {
                    ui.horizontal(|ui| {
                        if ui.button("+").clicked() {
                            self.count += 1;
                        }
                        ui.label(&self.count.to_string());
                        if ui.button("-").clicked() {
                            self.count -= 1;
                        }
                    });
                }
                Tabs::TempConverter => {
                    let mut c = format!("{:.2}", self.temp);
                    let mut f = format!("{:.2}", to_fahrenheit(self.temp));
                    // two extra sting variables in state are much better than cloning this 60 times per second
                    let c_tmp = c.clone();
                    let f_tmp = f.clone();
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut c);
                        ui.label("C = ");
                        ui.text_edit_singleline(&mut f);
                        ui.label("F");
                    });
                    if c != c_tmp {
                        match c.parse::<f32>() {
                            Ok(c) => self.temp = c,
                            Err(_) => {}
                        }
                    }
                    if f != f_tmp {
                        match f.parse::<f32>() {
                            Ok(f) => self.temp = from_fahrenheit(f),
                            Err(_) => {}
                        }
                    }
                }
                // Possibly as a result of imgui architecture I can already see a lot more places where a
                // string and data representation of the same element need to be tracked. Part of this is
                // again be my laziness, using a text input instead of a proper date picker. Because IM
                // components have no internal state, the work of storing an invalid number (from
                // temp) or date (in flightbooker) moves into my code and they MUST be stored when using text
                // inputs because the process of typing '90.02' includes '90.' and '90.0' which will both get
                // thrown away by a validator.
                Tabs::FlightBooker => {
                    let now = time::OffsetDateTime::now_utc().date();
                    // only saw this in examples, but not docs.
                    // rules for ui.* vs ui.add(egui::*) vs egui::*.show_ui are not clear
                    egui::ComboBox::new("trip_type", "")
                        .selected_text(self.fb.trip_type.to_string())
                        .show_ui(ui, |ui| {
                            for trip in [TripType::OneWay, TripType::Return] {
                                ui.selectable_value(
                                    &mut self.fb.trip_type,
                                    trip.clone(),
                                    trip.to_string(),
                                );
                            }
                        });

                    // somehow the above comment led to the design for best flight booker implementation
                    // I've done so far. Illegal states are unrepresentable, so there is no longer
                    // a need to track errors. Seven GUI says the challenge is "simplified" to textfields,
                    // but input validation is a harder problem than comparing numbers!
                    // let msg = &self.fb.validate();

                    ui.horizontal(|ui| {
                        ui.label("Depart ");
                        self.fb.departure = date_picker(ui, &mut self.fb.departure).max(now);
                    });
                    ui.add_enabled_ui(self.fb.trip_type == TripType::Return, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Arrival  ");
                            self.fb.arrival =
                                date_picker(ui, &mut self.fb.arrival).max(self.fb.departure);
                        });
                    });
                    if ui.button("Book").clicked() {
                        self.fb.reset()
                    }
                }
                Tabs::Timer => {
                    // Different model but objectively simpler than all
                    // the frameworks which need an Animation System
                    let elapsed = (self.elapsed_time.elapsed().as_millis() as f64).min(self.total_time * 1000.);
                    ui.horizontal(|ui| {
                        ui.label("Elapsed Time: ");
                        ui.add(egui::ProgressBar::new(
                            (elapsed / (self.total_time * 1000.0)) as f32,
                        ));
                    });
                    ui.label(format!("{:.2}s", elapsed / 1000.0));
                    ui.horizontal(|ui| {
                        ui.label("Duration: ");
                        ui.add(egui::Slider::new(&mut self.total_time, 0. ..=60.0).suffix("s"));
                    });
                    if elapsed <= self.total_time * 1000. {
                        ctx.request_repaint();
                    }
                    if ui.button("Reset").clicked() {
                        self.elapsed_time = std::time::Instant::now();
                    }
                }
                Tabs::Crud => {
                    
                }
                Tabs::Circles => todo!(),
                Tabs::Cells => todo!(),
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                counter(ui);
            });
        });
    }
}

fn counter(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.label(".");
    });
}

// So turns out dates (and other numerical inputs) are extremely
// easy if you use a DragValue instead of text input
fn date_picker(ui: &mut egui::Ui, date: &mut time::Date) -> time::Date {
    let (mut year, month, mut day) = date.to_calendar_date();
    let mut month_num: u8 = month.into();
    ui.horizontal(|ui| {
        ui.add(egui::DragValue::new(&mut day).range(1..=month.length(year)));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut month_num).range(1..=12));
        ui.label("/");
        ui.add(egui::DragValue::new(&mut year).range(-9999..=9999));
    });
    let month = time::Month::try_from(month_num).unwrap();
    let date = date.replace_year(year).unwrap();
    let date = date.replace_day(day.min(month.length(year))).unwrap();
    let date = date.replace_month(month).unwrap();
    date
}
