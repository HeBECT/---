# Скрипт запуска Kenshi Trade Map

Write-Host "╔═══════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         Kenshi Trade Map - Запуск приложения                     ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Проверка наличия собранного файла
$exePath = "target\release\kenshi-trade-map.exe"

if (Test-Path $exePath) {
    Write-Host "✅ Найден собранный файл" -ForegroundColor Green
    Write-Host "🚀 Запуск приложения..." -ForegroundColor Yellow
    Write-Host ""
    
    cargo run --release
} else {
    Write-Host "❌ Приложение не собрано!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Сначала соберите проект:" -ForegroundColor Yellow
    Write-Host "  .\build.ps1" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Или выполните:" -ForegroundColor Yellow
    Write-Host "  cargo build --release" -ForegroundColor Cyan
    Write-Host ""
    
    $build = Read-Host "Собрать сейчас? (Y/n)"
    
    if ($build -ne "n" -and $build -ne "N") {
        Write-Host ""
        Write-Host "🔨 Начинаем сборку..." -ForegroundColor Yellow
        cargo build --release
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host ""
            Write-Host "✅ Сборка завершена!" -ForegroundColor Green
            Write-Host "🚀 Запуск приложения..." -ForegroundColor Yellow
            Write-Host ""
            cargo run --release
        }
    }
}
