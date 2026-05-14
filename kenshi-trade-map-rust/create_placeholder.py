#!/usr/bin/env python3
"""
Создаёт временное изображение-заглушку для компиляции.
После компиляции замените src/watermark.png на ваше изображение
и перекомпилируйте проект.
"""

from PIL import Image, ImageDraw, ImageFont

# Создаём изображение 200x200 с прозрачным фоном
img = Image.new('RGBA', (200, 200), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Рисуем простую рамку
draw.rectangle([10, 10, 190, 190], outline=(255, 100, 100, 255), width=3)

# Добавляем текст
try:
    font = ImageFont.truetype("arial.ttf", 20)
except:
    font = ImageFont.load_default()

text = "Замените\nна ваше\nизображение"
draw.text((100, 100), text, fill=(255, 100, 100, 255), font=font, anchor="mm")

# Сохраняем
img.save('src/watermark.png', 'PNG')
print("✓ Создан файл src/watermark.png (заглушка)")
print("! Замените его на ваше изображение и перекомпилируйте проект")
