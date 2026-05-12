# Скрипт сборки Kenshi Trade Map

Write-Host "╔═══════════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║         Kenshi Trade Map - Сборка приложения                     ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Проверка установки Rust
Write-Host "🔍 Проверка установки Rust..." -ForegroundColor Yellow

# Попробуем найти cargo в PATH
$cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue

if ($null -eq $cargoCmd) {
    # Проверяем стандартное расположение
    $cargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
    if (Test-Path $cargoPath) {
        Write-Host "✅ Rust найден в: $cargoPath" -ForegroundColor Green
        Write-Host "⚠️  Cargo не в PATH, используем полный путь" -ForegroundColor Yellow
        $cargo = $cargoPath
    } else {
        Write-Host "❌ Rust не установлен!" -ForegroundColor Red
        Write-Host ""
        Write-Host "Установите Rust с официального сайта:" -ForegroundColor Yellow
        Write-Host "https://www.rust-lang.org/tools/install" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "После установки перезапустите PowerShell и запустите этот скрипт снова." -ForegroundColor Yellow
        exit 1
    }
} else {
    $cargo = "cargo"
    $cargoVersion = & $cargo --version
    Write-Host "✅ Rust установлен: $cargoVersion" -ForegroundColor Green
}

Write-Host ""

# Очистка предыдущей сборки (опционально)
$clean = Read-Host "Очистить предыдущую сборку? (y/N)"
if ($clean -eq "y" -or $clean -eq "Y") {
    Write-Host "🧹 Очистка..." -ForegroundColor Yellow
    & $cargo clean
    Write-Host "✅ Очистка завершена" -ForegroundColor Green
    Write-Host ""
}

# Сборка
Write-Host "🔨 Начинаем сборку..." -ForegroundColor Yellow
Write-Host "⏳ Это может занять 5-15 минут при первой сборке..." -ForegroundColor Yellow
Write-Host ""

$startTime = Get-Date
& $cargo build --release

if ($LASTEXITCODE -eq 0) {
    $endTime = Get-Date
    $duration = $endTime - $startTime
    
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                  ✅ СБОРКА УСПЕШНА! ✅                            ║" -ForegroundColor Green
    Write-Host "╚═══════════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "⏱️  Время сборки: $($duration.Minutes) мин $($duration.Seconds) сек" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "📁 Исполняемый файл находится здесь:" -ForegroundColor Yellow
    Write-Host "   target\release\kenshi-trade-map.exe" -ForegroundColor Cyan
    Write-Host ""
    
    # Проверка размера файла
    $exePath = "target\release\kenshi-trade-map.exe"
    if (Test-Path $exePath) {
        $fileSize = (Get-Item $exePath).Length / 1MB
        Write-Host "📊 Размер файла: $([math]::Round($fileSize, 2)) MB" -ForegroundColor Cyan
    }
    
    Write-Host ""
    Write-Host "🚀 Запустить приложение сейчас? (Y/n)" -ForegroundColor Yellow
    $run = Read-Host
    
    if ($run -ne "n" -and $run -ne "N") {
        Write-Host ""
        Write-Host "🎮 Запуск приложения..." -ForegroundColor Green
        & $cargo run --release
    }
} else {
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════════════╗" -ForegroundColor Red
    Write-Host "║                  ❌ ОШИБКА СБОРКИ! ❌                             ║" -ForegroundColor Red
    Write-Host "╚═══════════════════════════════════════════════════════════════════╝" -ForegroundColor Red
    Write-Host ""
    Write-Host "Попробуйте:" -ForegroundColor Yellow
    Write-Host "1. & `"$cargo`" clean" -ForegroundColor Cyan
    Write-Host "2. & `"$cargo`" build --release" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Или обновите Rust:" -ForegroundColor Yellow
    Write-Host "rustup update" -ForegroundColor Cyan
    exit 1
}
