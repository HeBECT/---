# Создаёт минимальное PNG изображение для компиляции
# После компиляции замените src/watermark.png на ваше изображение

Write-Host "Создание временного изображения watermark.png..." -ForegroundColor Yellow

# Минимальное валидное PNG изображение 1x1 пиксель (красный)
$pngBase64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8DwHwAFBQIAX8jx0gAAAABJRU5ErkJggg=="

$bytes = [Convert]::FromBase64String($pngBase64)
[IO.File]::WriteAllBytes("$PSScriptRoot\src\watermark.png", $bytes)

Write-Host "✓ Создан файл src/watermark.png (временная заглушка 1x1)" -ForegroundColor Green
Write-Host ""
Write-Host "ВАЖНО!" -ForegroundColor Red
Write-Host "Это временное изображение для компиляции." -ForegroundColor Yellow
Write-Host "Замените src/watermark.png на ваше изображение аниме-персонажа" -ForegroundColor Yellow
Write-Host "и запустите: cargo build --release" -ForegroundColor Cyan
