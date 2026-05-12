// ============================================
// KENSHI TRADE MAP - Интерактивная карта торговых маршрутов
// ============================================

class KenshiTradeMap {
    constructor() {
        this.canvas = document.getElementById('map-canvas');
        this.ctx = this.canvas.getContext('2d');
        
        // Камера
        this.camera = {
            x: 0,
            y: 0,
            zoom: 1,
            minZoom: 0.3,
            maxZoom: 3
        };

        // Состояние
        this.isDragging = false;
        this.dragStart = { x: 0, y: 0 };
        this.mousePos = { x: 0, y: 0 };
        this.hoveredCity = null;
        this.selectedCity = null;

        // Режим создания маршрута
        this.isCreatingRoute = false;
        this.routePoints = [];
        this.tempRoutePoint = null;

        // Данные
        this.cities = this.generateCities();
        this.routes = [];
        this.hoveredRoute = null;

        // Настройки отображения
        this.showGrid = false;

        this.init();
    }

    init() {
        this.resizeCanvas();
        this.setupEventListeners();
        this.centerView();
        this.animate();
    }

    resizeCanvas() {
        const container = document.getElementById('map-container');
        this.canvas.width = container.clientWidth;
        this.canvas.height = container.clientHeight;
    }

    setupEventListeners() {
        // Изменение размера окна
        window.addEventListener('resize', () => this.resizeCanvas());

        // Мышь
        this.canvas.addEventListener('mousedown', (e) => this.onMouseDown(e));
        this.canvas.addEventListener('mousemove', (e) => this.onMouseMove(e));
        this.canvas.addEventListener('mouseup', (e) => this.onMouseUp(e));
        this.canvas.addEventListener('wheel', (e) => this.onWheel(e));
        this.canvas.addEventListener('click', (e) => this.onClick(e));

        // Клавиатура
        window.addEventListener('keydown', (e) => this.onKeyDown(e));

        // Кнопки управления
        document.getElementById('reset-view').addEventListener('click', () => this.centerView());
        document.getElementById('toggle-grid').addEventListener('click', () => {
            this.showGrid = !this.showGrid;
        });
        document.getElementById('create-route').addEventListener('click', () => this.startCreatingRoute());
        document.getElementById('cancel-route').addEventListener('click', () => this.cancelRoute());
        document.getElementById('save-routes').addEventListener('click', () => this.saveRoutes());
        document.getElementById('load-routes').addEventListener('click', () => {
            document.getElementById('file-input').click();
        });
        document.getElementById('file-input').addEventListener('change', (e) => this.loadRoutes(e));

        // Контекстное меню маршрута
        document.getElementById('confirm-route').addEventListener('click', () => this.confirmRoute());
        document.getElementById('cancel-route-menu').addEventListener('click', () => this.hideRouteMenu());

        // Удаление маршрута
        document.getElementById('delete-route').addEventListener('click', () => this.deleteHoveredRoute());
    }

    // ============================================
    // ГЕНЕРАЦИЯ ГОРОДОВ
    // ============================================
    generateCities() {
        const cityNames = [
            'Хаб', 'Стоат', 'Скваин', 'Блистер Хилл', 'Хенг',
            'Шо-Баттай', 'Адмаг', 'Черная Пустошь', 'Мир\'с Энд',
            'Монгрел', 'Флэтс Лагун', 'Хефт', 'Стек', 'Барк',
            'Клаунстеад', 'Шарк', 'Дрифтер\'с Ласт', 'Вейстейшн',
            'Катун', 'Хо', 'Спринг', 'Рейвер\'с Пост'
        ];

        return cityNames.map((name, i) => ({
            id: i,
            name: name,
            x: Math.random() * 3000 - 1500,
            y: Math.random() * 3000 - 1500,
            size: 8 + Math.random() * 4
        }));
    }

    // ============================================
    // УПРАВЛЕНИЕ КАМЕРОЙ
    // ============================================
    centerView() {
        this.camera.x = 0;
        this.camera.y = 0;
        this.camera.zoom = 1;
    }

    screenToWorld(screenX, screenY) {
        return {
            x: (screenX - this.canvas.width / 2) / this.camera.zoom - this.camera.x,
            y: (screenY - this.canvas.height / 2) / this.camera.zoom - this.camera.y
        };
    }

    worldToScreen(worldX, worldY) {
        return {
            x: (worldX + this.camera.x) * this.camera.zoom + this.canvas.width / 2,
            y: (worldY + this.camera.y) * this.camera.zoom + this.canvas.height / 2
        };
    }

    // ============================================
    // ОБРАБОТКА СОБЫТИЙ
    // ============================================
    onMouseDown(e) {
        if (this.isCreatingRoute) return;
        
        this.isDragging = true;
        this.dragStart = {
            x: e.clientX - this.camera.x * this.camera.zoom,
            y: e.clientY - this.camera.y * this.camera.zoom
        };
    }

    onMouseMove(e) {
        const rect = this.canvas.getBoundingClientRect();
        this.mousePos = {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top
        };

        if (this.isDragging && !this.isCreatingRoute) {
            this.camera.x = (e.clientX - this.dragStart.x) / this.camera.zoom;
            this.camera.y = (e.clientY - this.dragStart.y) / this.camera.zoom;
        }

        // Проверка наведения на город
        const worldPos = this.screenToWorld(this.mousePos.x, this.mousePos.y);
        this.hoveredCity = this.findCityAt(worldPos.x, worldPos.y);

        // Проверка наведения на маршрут
        if (!this.isCreatingRoute) {
            this.hoveredRoute = this.findRouteAt(worldPos.x, worldPos.y);
            if (this.hoveredRoute) {
                this.showRouteTooltip(this.hoveredRoute);
            } else {
                this.hideRouteTooltip();
            }
        }

        // Временная точка при создании маршрута
        if (this.isCreatingRoute && this.routePoints.length > 0) {
            this.tempRoutePoint = worldPos;
        }
    }

    onMouseUp(e) {
        this.isDragging = false;
    }

    onWheel(e) {
        e.preventDefault();
        
        const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
        const newZoom = Math.max(this.camera.minZoom, Math.min(this.camera.maxZoom, this.camera.zoom * zoomFactor));
        
        // Масштабирование относительно курсора
        const worldPosBefore = this.screenToWorld(this.mousePos.x, this.mousePos.y);
        this.camera.zoom = newZoom;
        const worldPosAfter = this.screenToWorld(this.mousePos.x, this.mousePos.y);
        
        this.camera.x += worldPosAfter.x - worldPosBefore.x;
        this.camera.y += worldPosAfter.y - worldPosBefore.y;
    }

    onClick(e) {
        if (!this.isCreatingRoute) return;

        const rect = this.canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;
        const worldPos = this.screenToWorld(mouseX, mouseY);

        const city = this.findCityAt(worldPos.x, worldPos.y);
        if (city) {
            this.routePoints.push({ x: city.x, y: city.y, cityId: city.id });
        } else {
            this.routePoints.push({ x: worldPos.x, y: worldPos.y, cityId: null });
        }
    }

    onKeyDown(e) {
        if (e.key === 'Enter' && this.isCreatingRoute && this.routePoints.length >= 2) {
            this.finishRoute();
        } else if (e.key === 'Escape' && this.isCreatingRoute) {
            this.cancelRoute();
        }
    }

    // ============================================
    // УПРАВЛЕНИЕ МАРШРУТАМИ
    // ============================================
    startCreatingRoute() {
        this.isCreatingRoute = true;
        this.routePoints = [];
        this.tempRoutePoint = null;
        document.getElementById('create-route').style.display = 'none';
        document.getElementById('cancel-route').style.display = 'block';
    }

    cancelRoute() {
        this.isCreatingRoute = false;
        this.routePoints = [];
        this.tempRoutePoint = null;
        document.getElementById('create-route').style.display = 'block';
        document.getElementById('cancel-route').style.display = 'none';
        this.hideRouteMenu();
    }

    finishRoute() {
        if (this.routePoints.length < 2) return;

        // Показать меню создания маршрута
        const menu = document.getElementById('route-context-menu');
        menu.style.display = 'block';
        menu.style.left = '50%';
        menu.style.top = '50%';
        menu.style.transform = 'translate(-50%, -50%)';

        // Предзаполнить название
        const startCity = this.cities.find(c => c.id === this.routePoints[0].cityId);
        const endCity = this.cities.find(c => c.id === this.routePoints[this.routePoints.length - 1].cityId);
        if (startCity && endCity) {
            document.getElementById('route-name-input').value = `${startCity.name} → ${endCity.name}`;
        }
    }

    confirmRoute() {
        const name = document.getElementById('route-name-input').value.trim();
        const goods = document.getElementById('goods-select').value;
        const markup = parseFloat(document.getElementById('markup-input').value) || 0;

        if (!name) {
            alert('Введите название маршрута!');
            return;
        }

        const route = {
            id: Date.now(),
            name: name,
            points: [...this.routePoints],
            goods: goods,
            markup: markup,
            color: this.getRandomColor()
        };

        this.routes.push(route);
        this.updateRouteList();
        this.cancelRoute();
    }

    deleteHoveredRoute() {
        if (!this.hoveredRoute) return;
        
        this.routes = this.routes.filter(r => r.id !== this.hoveredRoute.id);
        this.hoveredRoute = null;
        this.hideRouteTooltip();
        this.updateRouteList();
    }

    hideRouteMenu() {
        document.getElementById('route-context-menu').style.display = 'none';
        document.getElementById('route-name-input').value = '';
        document.getElementById('goods-select').value = '';
        document.getElementById('markup-input').value = '0';
    }

    updateRouteList() {
        const list = document.getElementById('route-list');
        list.innerHTML = '';

        if (this.routes.length === 0) {
            list.innerHTML = '<p style="color: #999; font-size: 12px; padding: 10px;">Нет созданных маршрутов</p>';
            return;
        }

        this.routes.forEach(route => {
            const item = document.createElement('div');
            item.className = 'route-item';
            item.innerHTML = `
                <strong>${route.name}</strong>
                <small>${route.goods ? this.getGoodsName(route.goods) : 'Без товара'} 
                ${route.markup > 0 ? `(+${route.markup}%)` : route.markup < 0 ? `(${route.markup}%)` : ''}</small>
            `;
            item.addEventListener('click', () => this.focusOnRoute(route));
            list.appendChild(item);
        });
    }

    focusOnRoute(route) {
        if (route.points.length === 0) return;

        // Вычислить центр маршрута
        let sumX = 0, sumY = 0;
        route.points.forEach(p => {
            sumX += p.x;
            sumY += p.y;
        });

        this.camera.x = -sumX / route.points.length;
        this.camera.y = -sumY / route.points.length;
        this.camera.zoom = 1.5;
    }

    // ============================================
    // ПОИСК ОБЪЕКТОВ
    // ============================================
    findCityAt(x, y) {
        for (const city of this.cities) {
            const dx = city.x - x;
            const dy = city.y - y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            if (distance < city.size + 5) {
                return city;
            }
        }
        return null;
    }

    findRouteAt(x, y) {
        const threshold = 10 / this.camera.zoom;

        for (const route of this.routes) {
            for (let i = 0; i < route.points.length - 1; i++) {
                const p1 = route.points[i];
                const p2 = route.points[i + 1];

                const dist = this.pointToLineDistance(x, y, p1.x, p1.y, p2.x, p2.y);
                if (dist < threshold) {
                    return route;
                }
            }
        }
        return null;
    }

    pointToLineDistance(px, py, x1, y1, x2, y2) {
        const A = px - x1;
        const B = py - y1;
        const C = x2 - x1;
        const D = y2 - y1;

        const dot = A * C + B * D;
        const lenSq = C * C + D * D;
        let param = -1;

        if (lenSq !== 0) param = dot / lenSq;

        let xx, yy;

        if (param < 0) {
            xx = x1;
            yy = y1;
        } else if (param > 1) {
            xx = x2;
            yy = y2;
        } else {
            xx = x1 + param * C;
            yy = y1 + param * D;
        }

        const dx = px - xx;
        const dy = py - yy;
        return Math.sqrt(dx * dx + dy * dy);
    }

    // ============================================
    // ВСПЛЫВАЮЩИЕ ПОДСКАЗКИ
    // ============================================
    showRouteTooltip(route) {
        const tooltip = document.getElementById('route-tooltip');
        tooltip.style.display = 'block';
        tooltip.style.left = this.mousePos.x + 20 + 'px';
        tooltip.style.top = this.mousePos.y + 20 + 'px';

        document.getElementById('tooltip-title').textContent = route.name;
        
        let content = '';
        if (route.goods) {
            content += `<p><strong>Товар:</strong> ${this.getGoodsName(route.goods)}</p>`;
        }
        if (route.markup !== 0) {
            content += `<p><strong>Наценка:</strong> ${route.markup > 0 ? '+' : ''}${route.markup}%</p>`;
        }
        content += `<p><strong>Точек:</strong> ${route.points.length}</p>`;

        document.getElementById('tooltip-content').innerHTML = content;
    }

    hideRouteTooltip() {
        document.getElementById('route-tooltip').style.display = 'none';
    }

    // ============================================
    // СОХРАНЕНИЕ/ЗАГРУЗКА
    // ============================================
    saveRoutes() {
        const data = {
            version: '1.0',
            routes: this.routes,
            timestamp: new Date().toISOString()
        };

        const json = JSON.stringify(data, null, 2);
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `kenshi-routes-${Date.now()}.json`;
        a.click();
        
        URL.revokeObjectURL(url);
    }

    loadRoutes(e) {
        const file = e.target.files[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (event) => {
            try {
                const data = JSON.parse(event.target.result);
                if (data.routes && Array.isArray(data.routes)) {
                    this.routes = data.routes;
                    this.updateRouteList();
                    alert('Маршруты успешно загружены!');
                } else {
                    alert('Неверный формат файла!');
                }
            } catch (err) {
                alert('Ошибка при загрузке файла: ' + err.message);
            }
        };
        reader.readAsText(file);
    }

    // ============================================
    // ОТРИСОВКА
    // ============================================
    animate() {
        this.draw();
        requestAnimationFrame(() => this.animate());
    }

    draw() {
        const ctx = this.ctx;
        const w = this.canvas.width;
        const h = this.canvas.height;

        // Очистка
        ctx.fillStyle = '#1a1a2e';
        ctx.fillRect(0, 0, w, h);

        ctx.save();
        ctx.translate(w / 2, h / 2);
        ctx.scale(this.camera.zoom, this.camera.zoom);
        ctx.translate(this.camera.x, this.camera.y);

        // Сетка
        if (this.showGrid) {
            this.drawGrid();
        }

        // Маршруты
        this.routes.forEach(route => this.drawRoute(route));

        // Временный маршрут
        if (this.isCreatingRoute) {
            this.drawTemporaryRoute();
        }

        // Города
        this.cities.forEach(city => this.drawCity(city));

        ctx.restore();

        // UI поверх карты
        this.drawUI();
    }

    drawGrid() {
        const ctx = this.ctx;
        const gridSize = 200;
        const startX = Math.floor((-this.camera.x - this.canvas.width / 2 / this.camera.zoom) / gridSize) * gridSize;
        const startY = Math.floor((-this.camera.y - this.canvas.height / 2 / this.camera.zoom) / gridSize) * gridSize;
        const endX = startX + this.canvas.width / this.camera.zoom + gridSize;
        const endY = startY + this.canvas.height / this.camera.zoom + gridSize;

        ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
        ctx.lineWidth = 1 / this.camera.zoom;

        for (let x = startX; x < endX; x += gridSize) {
            ctx.beginPath();
            ctx.moveTo(x, startY);
            ctx.lineTo(x, endY);
            ctx.stroke();
        }

        for (let y = startY; y < endY; y += gridSize) {
            ctx.beginPath();
            ctx.moveTo(startX, y);
            ctx.lineTo(endX, y);
            ctx.stroke();
        }
    }

    drawCity(city) {
        const ctx = this.ctx;
        const isHovered = this.hoveredCity && this.hoveredCity.id === city.id;

        // Тень
        ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
        ctx.shadowBlur = 10 / this.camera.zoom;
        ctx.shadowOffsetX = 2 / this.camera.zoom;
        ctx.shadowOffsetY = 2 / this.camera.zoom;

        // Круг города
        ctx.fillStyle = isHovered ? '#ffd700' : '#4facfe';
        ctx.beginPath();
        ctx.arc(city.x, city.y, city.size, 0, Math.PI * 2);
        ctx.fill();

        ctx.shadowColor = 'transparent';

        // Обводка
        ctx.strokeStyle = isHovered ? '#fff' : '#00f2fe';
        ctx.lineWidth = 2 / this.camera.zoom;
        ctx.stroke();

        // Название
        ctx.fillStyle = '#fff';
        ctx.font = `${14 / this.camera.zoom}px Arial`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'top';
        ctx.fillText(city.name, city.x, city.y + city.size + 5 / this.camera.zoom);
    }

    drawRoute(route) {
        const ctx = this.ctx;
        const isHovered = this.hoveredRoute && this.hoveredRoute.id === route.id;

        ctx.strokeStyle = isHovered ? '#ffd700' : route.color;
        ctx.lineWidth = (isHovered ? 5 : 3) / this.camera.zoom;
        ctx.lineCap = 'round';
        ctx.lineJoin = 'round';

        // Тень для маршрута
        if (isHovered) {
            ctx.shadowColor = 'rgba(255, 215, 0, 0.5)';
            ctx.shadowBlur = 15 / this.camera.zoom;
        }

        ctx.beginPath();
        ctx.moveTo(route.points[0].x, route.points[0].y);
        for (let i = 1; i < route.points.length; i++) {
            ctx.lineTo(route.points[i].x, route.points[i].y);
        }
        ctx.stroke();

        ctx.shadowColor = 'transparent';

        // Точки маршрута
        route.points.forEach((point, i) => {
            ctx.fillStyle = route.color;
            ctx.beginPath();
            ctx.arc(point.x, point.y, 4 / this.camera.zoom, 0, Math.PI * 2);
            ctx.fill();
        });
    }

    drawTemporaryRoute() {
        const ctx = this.ctx;

        ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
        ctx.lineWidth = 3 / this.camera.zoom;
        ctx.setLineDash([10 / this.camera.zoom, 5 / this.camera.zoom]);

        ctx.beginPath();
        if (this.routePoints.length > 0) {
            ctx.moveTo(this.routePoints[0].x, this.routePoints[0].y);
            for (let i = 1; i < this.routePoints.length; i++) {
                ctx.lineTo(this.routePoints[i].x, this.routePoints[i].y);
            }
            if (this.tempRoutePoint) {
                ctx.lineTo(this.tempRoutePoint.x, this.tempRoutePoint.y);
            }
        }
        ctx.stroke();
        ctx.setLineDash([]);

        // Точки
        this.routePoints.forEach(point => {
            ctx.fillStyle = '#fff';
            ctx.beginPath();
            ctx.arc(point.x, point.y, 5 / this.camera.zoom, 0, Math.PI * 2);
            ctx.fill();
        });
    }

    drawUI() {
        const ctx = this.ctx;

        if (this.isCreatingRoute) {
            ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
            ctx.fillRect(10, 10, 300, 80);

            ctx.fillStyle = '#fff';
            ctx.font = '16px Arial';
            ctx.textAlign = 'left';
            ctx.fillText('Режим создания маршрута', 20, 30);
            ctx.font = '12px Arial';
            ctx.fillText(`Точек: ${this.routePoints.length}`, 20, 50);
            ctx.fillText('Enter - завершить | Esc - отменить', 20, 70);
        }
    }

    // ============================================
    // ВСПОМОГАТЕЛЬНЫЕ ФУНКЦИИ
    // ============================================
    getRandomColor() {
        const colors = [
            '#667eea', '#764ba2', '#f093fb', '#4facfe',
            '#43e97b', '#fa709a', '#fee140', '#30cfd0'
        ];
        return colors[Math.floor(Math.random() * colors.length)];
    }

    getGoodsName(goods) {
        const names = {
            iron: 'Железная руда',
            copper: 'Медная руда',
            fabric: 'Ткань',
            leather: 'Кожа',
            food: 'Еда',
            sake: 'Саке',
            grog: 'Грог',
            building_materials: 'Стройматериалы',
            robotics: 'Робототехника',
            engineering: 'Инженерия'
        };
        return names[goods] || goods;
    }
}

// ============================================
// ЗАПУСК ПРИЛОЖЕНИЯ
// ============================================
window.addEventListener('DOMContentLoaded', () => {
    new KenshiTradeMap();
});
