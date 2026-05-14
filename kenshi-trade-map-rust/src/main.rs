use eframe::egui;
use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use image::DynamicImage;
use serde::{Deserialize, Serialize};

// Встроенная ватермарка - изображение встроено в исполняемый файл
// Это изображение нельзя удалить или изменить без перекомпиляции
const WATERMARK_BYTES: &[u8] = include_bytes!("png.png");

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Language {
    Russian,
    English,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Theme {
    Dark,
    Light,
}

struct Texts {
    // Top panel
    title: &'static str,
    load_map: &'static str,
    add_city: &'static str,
    save: &'static str,
    load: &'static str,
    cities: &'static str,
    routes: &'static str,
    zoom: &'static str,
    language: &'static str,
    theme: &'static str,
    
    // Route dialog
    new_route: &'static str,
    route_name: &'static str,
    route_color: &'static str,
    create: &'static str,
    cancel: &'static str,
    
    // City dialog
    new_city: &'static str,
    city_name: &'static str,
    confirm: &'static str,
    
    // Trade panel
    trade_route: &'static str,
    item: &'static str,
    buy_markup: &'static str,
    sell_markup: &'static str,
    hold: &'static str,
    sell: &'static str,
    add_item: &'static str,
    delete_route: &'static str,
    close: &'static str,
    
    // Hints
    hint_route: &'static str,
    hint_city: &'static str,
    
    // Map
    load_map_prompt: &'static str,
}

impl Texts {
    fn get(lang: Language) -> Self {
        match lang {
            Language::Russian => Self {
                title: "Kenshi Trade Map",
                load_map: "Загрузить карту",
                add_city: "Добавить город",
                save: "Сохранить",
                load: "Загрузить",
                cities: "Городов",
                routes: "Маршрутов",
                zoom: "Зум",
                language: "Language",
                theme: "Тема",
                
                new_route: "Новый маршрут",
                route_name: "Название маршрута:",
                route_color: "Цвет маршрута:",
                create: "Создать",
                cancel: "Отмена",
                
                new_city: "Новый город",
                city_name: "Название города:",
                confirm: "Подтвердить",
                
                trade_route: "Торговый маршрут",
                item: "Товар:",
                buy_markup: "Наценка Покупки %:",
                sell_markup: "Наценка Продажи %:",
                hold: "Придержать",
                sell: "Продавать",
                add_item: "+ Добавить товар",
                delete_route: "Удалить маршрут",
                close: "Закрыть",
                
                hint_route: "ЛКМ по городу — выбрать конец маршрута | Esc — отмена",
                hint_city: "ЛКМ на карте — разместить город | Esc — отмена",
                
                load_map_prompt: "Загрузите карту Kenshi",
            },
            Language::English => Self {
                title: "Kenshi Trade Map",
                load_map: "Load Map",
                add_city: "Add City",
                save: "Save",
                load: "Load",
                cities: "Cities",
                routes: "Routes",
                zoom: "Zoom",
                language: "Язык",
                theme: "Theme",
                
                new_route: "New Route",
                route_name: "Route name:",
                route_color: "Route color:",
                create: "Create",
                cancel: "Cancel",
                
                new_city: "New City",
                city_name: "City name:",
                confirm: "Confirm",
                
                trade_route: "Trade Route",
                item: "Item:",
                buy_markup: "Buy Markup %:",
                sell_markup: "Sell Markup %:",
                hold: "Hold",
                sell: "Sell",
                add_item: "+ Add Item",
                delete_route: "Delete Route",
                close: "Close",
                
                hint_route: "LMB on city — select end | Esc — cancel",
                hint_city: "LMB on map — place city | Esc — cancel",
                
                load_map_prompt: "Load Kenshi map",
            },
        }
    }
}

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
    buy_markup: f32,   // Наценка покупки %
    sell_markup: f32,  // Наценка продажи %
    #[serde(default)]
    hold: bool,        // Придержать
    #[serde(default)]
    sell: bool,        // Продавать
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
    #[serde(default)]
    offset_x: f32,  // Смещение по X для дублирующихся маршрутов
    #[serde(default)]
    offset_y: f32,  // Смещение по Y для дублирующихся маршрутов
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
    
    // Language & Theme
    language: Language,
    theme: Theme,
    
    // Watermark
    watermark_texture: Option<egui::TextureHandle>,
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
            language: Language::Russian,
            theme: Theme::Dark,
            watermark_texture: None,
        }
    }
}

impl eframe::App for KenshiTradeMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.camera.update_smooth_zoom();

        // Apply theme
        match self.theme {
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
        }

        let texts = Texts::get(self.language);

        // Top panel
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(texts.title);
                    ui.separator();
                    
                    if ui.button(texts.load_map).clicked() {
                        self.load_map_dialog(ctx);
                    }
                    
                    ui.separator();
                    
                    if ui.button(texts.add_city).clicked() {
                        self.state = AppState::PlacingCity;
                    }
                    
                    ui.separator();
                    
                    if ui.button(texts.save).clicked() {
                        self.save_data();
                    }
                    
                    if ui.button(texts.load).clicked() {
                        self.load_data();
                    }
                    
                    ui.separator();
                    ui.label(format!("{}: {}", texts.cities, self.cities.len()));
                    ui.label(format!("{}: {}", texts.routes, self.routes.len()));
                    ui.label(format!("{}: {:.1}x", texts.zoom, self.camera.zoom));
                    
                    ui.separator();
                    if ui.button(texts.language).clicked() {
                        self.language = match self.language {
                            Language::Russian => Language::English,
                            Language::English => Language::Russian,
                        };
                    }
                    
                    if ui.button(texts.theme).clicked() {
                        self.theme = match self.theme {
                            Theme::Dark => Theme::Light,
                            Theme::Light => Theme::Dark,
                        };
                    }
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
        let bg_color = match self.theme {
            Theme::Dark => Color32::from_rgb(20, 20, 25),
            Theme::Light => Color32::from_rgb(240, 240, 245),
        };
        
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg_color))
            .show(ctx, |ui| {
                let (response, painter) = ui.allocate_painter(
                    ui.available_size(),
                    egui::Sense::click_and_drag(),
                );

                let rect = response.rect;
                let center = rect.center();

                // Background
                painter.rect_filled(rect, 0.0, bg_color);

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
                        texts.load_map_prompt,
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
                    AppState::CreatingRoute(_) => texts.hint_route,
                    AppState::PlacingCity => texts.hint_city,
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

                // Load watermark on first frame if not loaded
                if self.watermark_texture.is_none() {
                    self.load_watermark(ctx);
                }

                // Draw watermark in bottom-right corner
                if let Some(watermark) = &self.watermark_texture {
                    let watermark_size = Vec2::new(100.0, 100.0);
                    let watermark_pos = Pos2::new(
                        rect.max.x - watermark_size.x - 10.0,
                        rect.max.y - watermark_size.y - 10.0,
                    );
                    let watermark_rect = Rect::from_min_size(watermark_pos, watermark_size);
                    
                    // Draw watermark image
                    painter.image(
                        watermark.id(),
                        watermark_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    
                    // Check if mouse is hovering over watermark
                    if let Some(hover_pos) = response.hover_pos() {
                        if watermark_rect.contains(hover_pos) {
                            // Show tooltip
                            egui::show_tooltip_at_pointer(ctx, egui::Id::new("watermark_tooltip"), |ui| {
                                ui.label("mushoku tensei | Реинкарнация безработного");
                            });
                        }
                    }
                }
            });

        // Route dialog
        if self.show_route_dialog {
            egui::Window::new(texts.new_route)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(texts.route_name);
                    ui.text_edit_singleline(&mut self.route_name_input);
                    
                    ui.add_space(10.0);
                    
                    ui.label(texts.route_color);
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
                        if ui.button(texts.create).clicked() {
                            self.confirm_route_creation();
                        }
                        if ui.button(texts.cancel).clicked() {
                            self.cancel_route_creation();
                        }
                    });
                });
        }

        // City name dialog
        if self.show_city_name_dialog {
            egui::Window::new(texts.new_city)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(texts.city_name);
                    let response = ui.text_edit_singleline(&mut self.city_name_input);
                    
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.confirm_city_creation();
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button(texts.confirm).clicked() {
                            self.confirm_city_creation();
                        }
                        if ui.button(texts.cancel).clicked() {
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
        // Zoom with normalized and slower speed, centered on cursor
        if let Some(hover_pos) = response.hover_pos() {
            let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
            if scroll_delta != 0.0 {
                // Нормализуем скорость скролла (ограничиваем максимальное значение)
                let normalized_delta = scroll_delta.clamp(-50.0, 50.0);
                
                // Более медленный zoom factor (было 1.15, стало 1.08)
                let zoom_speed = 1.0 + (normalized_delta / 500.0);
                let zoom_factor = zoom_speed.clamp(0.92, 1.08);
                
                // Сохраняем мировую позицию под курсором
                let world_pos_before = self.camera.screen_to_world(hover_pos, center);
                
                // Применяем новый зум
                self.camera.target_zoom = (self.camera.target_zoom * zoom_factor).clamp(0.1, 3.0);
                
                // Корректируем offset так, чтобы мировая позиция под курсором осталась на месте
                // Вычисляем где будет эта точка после зума
                let world_pos_after_screen = self.camera.world_to_screen(world_pos_before, center);
                
                // Смещаем offset чтобы компенсировать разницу
                self.camera.offset += hover_pos - world_pos_after_screen;
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

            // Проверяем, есть ли уже маршруты между этими городами
            let existing_routes_count = self.routes.iter()
                .filter(|r| {
                    (r.start_city_name == start_city.name && r.end_city_name == end_city.name) ||
                    (r.start_city_name == end_city.name && r.end_city_name == start_city.name)
                })
                .count();

            // Вычисляем смещение для маршрута
            let base_start = Pos2::new(start_city.x, start_city.y);
            let base_end = Pos2::new(end_city.x, end_city.y);
            let mut start_point = base_start;
            let mut end_point = base_end;
            
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            
            if existing_routes_count > 0 {
                // Смещаем маршрут перпендикулярно линии между городами
                let direction = (base_end - base_start).normalized();
                let perpendicular = Vec2::new(-direction.y, direction.x);
                let offset_distance = 15.0 * (existing_routes_count as f32);
                let offset = perpendicular * offset_distance;
                
                offset_x = offset.x;
                offset_y = offset.y;
                
                start_point += offset;
                end_point += offset;
            }

            let route = TradeRoute {
                id: self.routes.len(),
                name,
                start_point,
                end_point,
                start_city_name: start_city.name.clone(),
                end_city_name: end_city.name.clone(),
                color: [self.route_color.r(), self.route_color.g(), self.route_color.b()],
                items: Vec::new(),
                offset_x,
                offset_y,
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
        let texts = Texts::get(self.language);
        
        ui.heading(texts.trade_route);
        
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
                                ui.label(texts.item);
                                ui.text_edit_singleline(&mut item.name);
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(texts.buy_markup);
                                ui.add(egui::DragValue::new(&mut item.buy_markup).speed(0.5));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.label(texts.sell_markup);
                                ui.add(egui::DragValue::new(&mut item.sell_markup).speed(0.5));
                            });
                            
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut item.hold, texts.hold);
                                ui.add_space(10.0);
                                ui.checkbox(&mut item.sell, texts.sell);
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
                
                if ui.button(texts.add_item).clicked() {
                    route.items.push(TradeItem {
                        name: String::new(),
                        buy_markup: 0.0,
                        sell_markup: 0.0,
                        hold: false,
                        sell: false,
                    });
                }
                
                ui.separator();
                
                if ui.button(texts.delete_route).clicked() {
                    self.routes.retain(|r| r.id != route_id);
                    self.show_trade_panel = false;
                    self.active_route_id = None;
                    return;
                }
            }
        }
        
        ui.separator();
        
        if ui.button(texts.close).clicked() {
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

    fn load_watermark(&mut self, ctx: &egui::Context) {
        // Загружаем встроенную ватермарку из байтов (встроена в исполняемый файл)
        // Это изображение нельзя удалить или изменить без перекомпиляции приложения
        if let Ok(img) = image::load_from_memory(WATERMARK_BYTES) {
            let size = [img.width() as usize, img.height() as usize];
            let img_buffer = img.to_rgba8();
            let pixels = img_buffer.as_flat_samples();
            
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            );
            
            self.watermark_texture = Some(ctx.load_texture("watermark", color_image, Default::default()));
        }
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
                    
                    // Reconstruct points from city names with saved offset
                    for route in &mut data.routes {
                        if let Some(start_city) = self.cities.iter().find(|c| c.name == route.start_city_name) {
                            route.start_point = Pos2::new(
                                start_city.x + route.offset_x,
                                start_city.y + route.offset_y
                            );
                        }
                        if let Some(end_city) = self.cities.iter().find(|c| c.name == route.end_city_name) {
                            route.end_point = Pos2::new(
                                end_city.x + route.offset_x,
                                end_city.y + route.offset_y
                            );
                        }
                    }
                    self.routes = data.routes;
                }
            }
        }
    }
}
