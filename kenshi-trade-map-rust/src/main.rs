use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use image::DynamicImage;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Kenshi Trade Map"),
        ..Default::default()
    };

    eframe::run_native(
        "Kenshi Trade Map",
        options,
        Box::new(|cc| {
            // Темная тема
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(KenshiTradeMap::default())
        }),
    )
}

#[derive(Clone, Serialize, Deserialize)]
struct City {
    id: usize,
    name: String,
    x: f32,
    y: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct TradeItem {
    name: String,
    markup: f32,
}

#[derive(Clone, Serialize, Deserialize)]
struct TradeRoute {
    id: usize,
    name: String,
    #[serde(skip, default)]
    start_point: Pos2,
    #[serde(skip, default)]
    end_point: Pos2,
    start_city_name: String,
    end_city_name: String,
    color: [u8; 3],
    items: Vec<TradeItem>,
}

struct Camera {
    offset: Vec2,
    zoom: f32,
    target_zoom: f32,
}

impl Camera {
    fn world_to_screen(&self, world_pos: Pos2, screen_center: Pos2) -> Pos2 {
        let scaled = (world_pos.to_vec2() * self.zoom) + self.offset;
        screen_center + scaled
    }

    fn screen_to_world(&self, screen_pos: Pos2, screen_center: Pos2) -> Pos2 {
        let relative = screen_pos - screen_center;
        ((relative - self.offset) / self.zoom).to_pos2()
    }

    fn update_smooth_zoom(&mut self) {
        // Более плавная интерполяция (медленнее)
        let lerp_factor = 0.12;
        self.zoom += (self.target_zoom - self.zoom) * lerp_factor;
    }
}

enum AppState {
    Normal,
    PlacingCity,
    CreatingRoute(usize), // Содержит ID начального города
}

struct KenshiTradeMap {
    cities: Vec<City>,
    routes: Vec<TradeRoute>,
    camera: Camera,
    map_texture: Option<egui::TextureHandle>,
    map_size: Vec2,
    
    // State
    state: AppState,
    hovered_city: Option<usize>,
    
    // UI State
    show_route_dialog: bool,
    route_name_input: String,
    route_color: Color32,
    pending_route: Option<(usize, usize)>, // (start_city_id, end_city_id)
    
    // City creation
    show_city_name_dialog: bool,
    city_name_input: String,
    pending_city_pos: Option<Pos2>,
    
    // Trade panel
    show_trade_panel: bool,
    active_route_id: Option<usize>,
}

impl Default for KenshiTradeMap {
    fn default() -> Self {
        Self {
            cities: Vec::new(),
            routes: Vec::new(),
            camera: Camera {
                offset: Vec2::ZERO,
                zoom: 0.3,
                target_zoom: 0.3,
            },
            map_texture: None,
            map_size: Vec2::new(3000.0, 2000.0),
            state: AppState::Normal,
            hovered_city: None,
            show_route_dialog: false,
            route_name_input: String::new(),
            route_color: Color32::from_rgb(76, 175, 80),
            pending_route: None,
            show_city_name_dialog: false,
            city_name_input: String::new(),
            pending_city_pos: None,
            show_trade_panel: false,
            active_route_id: None,
        }
    }
}

impl eframe::App for KenshiTradeMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.camera.update_smooth_zoom();

        // Top panel - темная тема
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("🏴‍☠️ Kenshi Trade Map");
                    ui.separator();
                    
                    if ui.button("📁 Загрузить карту").clicked() {
                        self.load_map_dialog(ctx);
                    }
                    
                    ui.separator();
                    
                    if ui.button("🏙️ Добавить город").clicked() {
                        self.state = AppState::PlacingCity;
                    }
                    
                    ui.separator();
                    
                    if ui.button("💾 Сохранить").clicked() {
                        self.save_data();
                    }
                    
                    if ui.button("📂 Загрузить").clicked() {
                        self.load_data();
                    }
                    
                    ui.separator();
                    ui.label(format!("Городов: {}", self.cities.len()));
                    ui.label(format!("Маршрутов: {}", self.routes.len()));
                    ui.label(format!("Зум: {:.1}x", self.camera.zoom));
                });
            });

        // Trade panel
        if self.show_trade_panel {
            egui::SidePanel::right("trade_panel")
                .default_width(400.0)
                .show(ctx, |ui| {
                    self.draw_trade_panel(ui);
                });
        }

        // Main canvas
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(20, 20, 25)))
            .show(ctx, |ui| {
                let (response, painter) = ui.allocate_painter(
                    ui.available_size(),
                    egui::Sense::click_and_drag(),
                );

                let rect = response.rect;
                let center = rect.center();

                // Background - темный
                painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 25));

                // Draw map if loaded
                if let Some(texture) = &self.map_texture {
                    let map_rect = Rect::from_center_size(
                        center + self.camera.offset,
                        self.map_size * self.camera.zoom,
                    );
                    painter.image(
                        texture.id(),
                        map_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    
                    painter.rect_filled(map_rect, 0.0, Color32::from_black_alpha(80));
                } else {
                    painter.text(
                        center,
                        egui::Align2::CENTER_CENTER,
                        "Загрузите карту Kenshi",
                        egui::FontId::proportional(24.0),
                        Color32::from_rgb(150, 150, 150),
                    );
                }

                // Draw routes
                for route in &self.routes {
                    self.draw_route(&painter, route, center);
                }

                // Draw cities
                self.hovered_city = None;
                for (idx, city) in self.cities.iter().enumerate() {
                    let screen_pos = self.camera.world_to_screen(
                        Pos2::new(city.x, city.y),
                        center,
                    );

                    if rect.contains(screen_pos) {
                        let radius = 6.0 * self.camera.zoom;
                        let is_hovered = response.hover_pos()
                            .map(|p| p.distance(screen_pos) < radius + 10.0)
                            .unwrap_or(false);

                        if is_hovered {
                            self.hovered_city = Some(idx);
                            
                            // Glow effect
                            painter.circle_filled(
                                screen_pos,
                                radius + 15.0,
                                Color32::from_rgba_unmultiplied(212, 164, 55, 30),
                            );
                        }

                        // City marker
                        painter.circle_filled(
                            screen_pos,
                            radius,
                            if is_hovered {
                                Color32::WHITE
                            } else {
                                Color32::from_rgb(212, 164, 55)
                            },
                        );
                        painter.circle_stroke(
                            screen_pos,
                            radius,
                            Stroke::new(2.0, Color32::from_rgb(40, 40, 45)),
                        );

                        // City name (только при наведении)
                        if is_hovered {
                            painter.text(
                                screen_pos + Vec2::new(0.0, -radius - 10.0),
                                egui::Align2::CENTER_BOTTOM,
                                &city.name,
                                egui::FontId::proportional(14.0),
                                Color32::WHITE,
                            );
                        }
                    }
                }

                self.handle_input(&response, center, ctx);

                // Hints
                let hint_text = match &self.state {
                    AppState::CreatingRoute(_) => "ЛКМ по городу — выбрать конец маршрута | Esc — отмена",
                    AppState::PlacingCity => "ЛКМ на карте — разместить город | Esc — отмена",
                    _ => "",
                };

                if !hint_text.is_empty() {
                    painter.text(
                        Pos2::new(rect.center().x, rect.max.y - 30.0),
                        egui::Align2::CENTER_CENTER,
                        hint_text,
                        egui::FontId::proportional(16.0),
                        Color32::from_rgb(212, 164, 55),
                    );
                }
            });

        // Route dialog
        if self.show_route_dialog {
            egui::Window::new("Новый маршрут")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Название маршрута:");
                    ui.text_edit_singleline(&mut self.route_name_input);
                    
                    ui.add_space(10.0);
                    
                    ui.label("Цвет маршрута:");
                    ui.horizontal(|ui| {
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut self.route_color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        
                        // Предустановленные цвета
                        let preset_colors = [
                            ("🟢", Color32::from_rgb(76, 175, 80)),   // Зеленый
                            ("🔵", Color32::from_rgb(33, 150, 243)),  // Синий
                            ("🟠", Color32::from_rgb(255, 152, 0)),   // Оранжевый
                            ("🔴", Color32::from_rgb(233, 30, 99)),   // Розовый
                            ("🔷", Color32::from_rgb(0, 188, 212)),   // Голубой
                            ("🟡", Color32::from_rgb(255, 235, 59)),  // Желтый
                            ("🟣", Color32::from_rgb(156, 39, 176)),  // Фиолетовый
                            ("🔺", Color32::from_rgb(244, 67, 54)),   // Красный
                        ];
                        
                        for (emoji, color) in preset_colors {
                            let button = egui::Button::new(emoji)
                                .fill(color)
                                .min_size(Vec2::new(30.0, 30.0));
                            if ui.add(button).clicked() {
                                self.route_color = color;
                            }
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("✓ Создать").clicked() {
                            self.confirm_route_creation();
                        }
                        if ui.button("✗ Отмена").clicked() {
                            self.cancel_route_creation();
                        }
                    });
                });
        }

        // City name dialog
        if self.show_city_name_dialog {
            egui::Window::new("Новый город")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("Название города:");
                    let response = ui.text_edit_singleline(&mut self.city_name_input);
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.confirm_city_creation();
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("✓ Подтвердить").clicked() {
                            self.confirm_city_creation();
                        }
                        if ui.button("✗ Отмена").clicked() {
                            self.cancel_city_creation();
                        }
                    });
                });
        }

        // Context menu для создания маршрута
        if let Some(city_idx) = self.hovered_city {
            if ctx.input(|i| i.pointer.secondary_clicked()) {
                self.start_route_creation(city_idx);
            }
        }

        ctx.request_repaint();
    }
}

impl KenshiTradeMap {
    fn draw_route(&self, painter: &egui::Painter, route: &TradeRoute, center: Pos2) {
        let color = Color32::from_rgb(route.color[0], route.color[1], route.color[2]);
        let stroke = Stroke::new(6.0 * self.camera.zoom, color);

        let p1 = self.camera.world_to_screen(route.start_point, center);
        let p2 = self.camera.world_to_screen(route.end_point, center);
        
        // Линия маршрута
        painter.line_segment([p1, p2], stroke);
        
        // Стрелка в конце (указывает направление)
        let direction = (p2 - p1).normalized();
        let arrow_size = 15.0 * self.camera.zoom;
        let arrow_angle = std::f32::consts::PI / 6.0;
        
        let arrow_left = p2 - direction.rot90() * arrow_size * arrow_angle.sin() - direction * arrow_size * arrow_angle.cos();
        let arrow_right = p2 + direction.rot90() * arrow_size * arrow_angle.sin() - direction * arrow_size * arrow_angle.cos();
        
        painter.line_segment([p2, arrow_left], stroke);
        painter.line_segment([p2, arrow_right], stroke);
        
        // Точки начала и конца
        painter.circle_filled(p1, 4.0 * self.camera.zoom, color);
        painter.circle_filled(p2, 4.0 * self.camera.zoom, color);
    }

    fn handle_input(&mut self, response: &egui::Response, center: Pos2, ctx: &egui::Context) {
        // Zoom with normalized and slower speed
        if let Some(hover_pos) = response.hover_pos() {
            let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                // Нормализуем скорость скролла (ограничиваем максимальное значение)
                let normalized_delta = scroll_delta.clamp(-50.0, 50.0);
                
                // Более медленный zoom factor (было 1.15, стало 1.08)
                let zoom_speed = 1.0 + (normalized_delta / 500.0);
                let zoom_factor = zoom_speed.clamp(0.92, 1.08);
                
                let old_zoom = self.camera.zoom;
                self.camera.target_zoom = (self.camera.target_zoom * zoom_factor).clamp(0.1, 3.0);
                
                // Плавная корректировка позиции при зуме
                let world_pos = self.camera.screen_to_world(hover_pos, center);
                let new_screen_pos = self.camera.world_to_screen(world_pos, center);
                let offset_adjustment = (hover_pos - new_screen_pos) * (self.camera.target_zoom / old_zoom - 1.0);
                self.camera.offset += offset_adjustment * 0.12;
            }
        }

        // Pan
        if response.dragged_by(egui::PointerButton::Primary) {
            match self.state {
                AppState::CreatingRoute(_) | AppState::PlacingCity => {}
                _ => {
                    if self.hovered_city.is_none() {
                        self.camera.offset += response.drag_delta();
                    }
                }
            }
        }

        // Click handling
        if response.clicked() {
            match &self.state {
                AppState::PlacingCity => {
                    if let Some(pos) = response.hover_pos() {
                        let world_pos = self.camera.screen_to_world(pos, center);
                        self.pending_city_pos = Some(world_pos);
                        self.show_city_name_dialog = true;
                    }
                }
                AppState::CreatingRoute(start_city_id) => {
                    if let Some(end_city_idx) = self.hovered_city {
                        let end_city_id = self.cities[end_city_idx].id;
                        if *start_city_id != end_city_id {
                            self.pending_route = Some((*start_city_id, end_city_id));
                            self.show_route_dialog = true;
                        }
                    }
                }
                AppState::Normal => {
                    if let Some(pos) = response.hover_pos() {
                        let world_pos = self.camera.screen_to_world(pos, center);
                        if let Some(route_id) = self.find_route_at(world_pos) {
                            self.active_route_id = Some(route_id);
                            self.show_trade_panel = true;
                        }
                    }
                }
            }
        }

        // Keyboard
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Escape) {
                match self.state {
                    AppState::CreatingRoute(_) => {
                        self.state = AppState::Normal;
                    }
                    AppState::PlacingCity => {
                        self.cancel_city_creation();
                    }
                    _ => {
                        self.show_trade_panel = false;
                    }
                }
            }
        });
    }

    fn start_route_creation(&mut self, city_idx: usize) {
        let city_id = self.cities[city_idx].id;
        self.state = AppState::CreatingRoute(city_id);
    }

    fn confirm_route_creation(&mut self) {
        if let Some((start_id, end_id)) = self.pending_route {
            let start_city = self.cities.iter().find(|c| c.id == start_id).unwrap();
            let end_city = self.cities.iter().find(|c| c.id == end_id).unwrap();
            
            let name = if self.route_name_input.is_empty() {
                format!("{} → {}", start_city.name, end_city.name)
            } else {
                self.route_name_input.clone()
            };

            let route = TradeRoute {
                id: self.routes.len(),
                name,
                start_point: Pos2::new(start_city.x, start_city.y),
                end_point: Pos2::new(end_city.x, end_city.y),
                start_city_name: start_city.name.clone(),
                end_city_name: end_city.name.clone(),
                color: [self.route_color.r(), self.route_color.g(), self.route_color.b()],
                items: Vec::new(),
            };

            self.routes.push(route);
        }
        self.cancel_route_creation();
    }

    fn cancel_route_creation(&mut self) {
        self.state = AppState::Normal;
        self.show_route_dialog = false;
        self.route_name_input.clear();
        self.pending_route = None;
        self.route_color = Color32::from_rgb(76, 175, 80);
    }

    fn confirm_city_creation(&mut self) {
        if let Some(pos) = self.pending_city_pos {
            let name = if self.city_name_input.is_empty() {
                format!("Город {}", self.cities.len() + 1)
            } else {
                self.city_name_input.clone()
            };

            let city = City {
                id: self.cities.len(),
                name,
                x: pos.x,
                y: pos.y,
            };

            self.cities.push(city);
        }
        self.cancel_city_creation();
    }

    fn cancel_city_creation(&mut self) {
        self.state = AppState::Normal;
        self.show_city_name_dialog = false;
        self.city_name_input.clear();
        self.pending_city_pos = None;
    }

    fn find_route_at(&self, world_pos: Pos2) -> Option<usize> {
        let threshold = 20.0 / self.camera.zoom;
        
        for route in &self.routes {
            let dist = Self::distance_to_segment(world_pos, route.start_point, route.end_point);
            if dist < threshold {
                return Some(route.id);
            }
        }
        
        None
    }

    fn distance_to_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
        let ab = b - a;
        let ap = p - a;
        let t = (ap.dot(ab) / ab.length_sq()).clamp(0.0, 1.0);
        let closest = a + ab * t;
        p.distance(closest)
    }

    fn draw_trade_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Торговый маршрут");
        
        if let Some(route_id) = self.active_route_id {
            if let Some(route) = self.routes.iter_mut().find(|r| r.id == route_id) {
                ui.label(format!("📍 {}", route.name));
                ui.label(format!("🏙️ {} → {}", route.start_city_name, route.end_city_name));
                
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut to_remove = None;
                    
                    for (idx, item) in route.items.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("Товар:");
                                ui.text_edit_singleline(&mut item.name);
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label("Наценка %:");
                                ui.add(egui::DragValue::new(&mut item.markup).speed(0.5));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() - 30.0);
                                if ui.button("✖").clicked() {
                                    to_remove = Some(idx);
                                }
                            });
                        });
                    }
                    
                    if let Some(idx) = to_remove {
                        route.items.remove(idx);
                    }
                });
                
                ui.separator();
                
                if ui.button("+ Добавить товар").clicked() {
                    route.items.push(TradeItem {
                        name: String::new(),
                        markup: 0.0,
                    });
                }
                
                ui.separator();
                
                if ui.button("🗑️ Удалить маршрут").clicked() {
                    self.routes.retain(|r| r.id != route_id);
                    self.show_trade_panel = false;
                    self.active_route_id = None;
                    return;
                }
            }
        }
        
        ui.separator();
        
        if ui.button("✗ Закрыть").clicked() {
            self.show_trade_panel = false;
        }
    }

    fn load_map_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .pick_file()
        {
            if let Ok(img) = image::open(&path) {
                self.load_map_texture(ctx, img);
            }
        }
    }

    fn load_map_texture(&mut self, ctx: &egui::Context, img: DynamicImage) {
        let size = [img.width() as usize, img.height() as usize];
        let img_buffer = img.to_rgba8();
        let pixels = img_buffer.as_flat_samples();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );
        
        self.map_texture = Some(ctx.load_texture("map", color_image, Default::default()));
        self.map_size = Vec2::new(size[0] as f32, size[1] as f32);
        
        self.camera.zoom = 0.3;
        self.camera.target_zoom = 0.3;
        self.camera.offset = Vec2::ZERO;
    }

    fn save_data(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .set_file_name("kenshi_data.json")
            .save_file()
        {
            #[derive(Serialize)]
            struct SaveData {
                cities: Vec<City>,
                routes: Vec<TradeRoute>,
            }

            let data = SaveData {
                cities: self.cities.clone(),
                routes: self.routes.clone(),
            };

            if let Ok(json) = serde_json::to_string_pretty(&data) {
                let _ = std::fs::write(path, json);
            }
        }
    }

    fn load_data(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(path) {
                #[derive(Deserialize)]
                struct SaveData {
                    cities: Vec<City>,
                    routes: Vec<TradeRoute>,
                }

                if let Ok(mut data) = serde_json::from_str::<SaveData>(&content) {
                    self.cities = data.cities;
                    
                    // Reconstruct points from city names
                    for route in &mut data.routes {
                        if let Some(start_city) = self.cities.iter().find(|c| c.name == route.start_city_name) {
                            route.start_point = Pos2::new(start_city.x, start_city.y);
                        }
                        if let Some(end_city) = self.cities.iter().find(|c| c.name == route.end_city_name) {
                            route.end_point = Pos2::new(end_city.x, end_city.y);
                        }
                    }
                    self.routes = data.routes;
                }
            }
        }
    }
}
