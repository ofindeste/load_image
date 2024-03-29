# Приложение для загрузки изображений

  

## Используемые библиотеки

Были использованы следующие библиотеки:

- `actix-web`

- `reqwest`
  > библиотека `reqwest` использует `openssl` на ОС GNU/Linux, поэтому её будет необходимо установить. На Debian подобных системах используется команда:
   > ```
   > sudo apt install libssl-dev

## Запуск приложения

Для запуска используется команда:

```

cargo run

```

## Тестирование

Для тестирования используется команда:

```

cargo test

```

## Использование
Были созданы пять маршрутов:
  - `/`
  - `/image/load/original`
  - `/image/load/preview`
  - `/image/show/original`
  - `/image/show/preview`
  Все маршруты работают только с одним HTTP методом - GET.
Методы `/image/load/original` и  `/image/load/preview` принимают один параметр `image_url`, в который передается url на изображение.
Методы `/image/show/original` и `/image/show/preview` принимают неограниченное количество параметров (начиная с нуля). Например, `/image/show/original?image_url[0]=https://sr.gallerix.ru/_UNK/1018810316/3526.jpg`.
Для работы с данными методами был сделан минимальный интерфейс.