# История изменений

Формат этого файла соответствует рекомендациям [Keep a Changelog](http://keepachangelog.com/en/1.0.0/).
Проект использует [семантическое версионирование](http://semver.org/spec/v2.0.0.html).

## Не выпущено


## 1.0.1 — 20.07.2021

### Исправлено

- Не был учтён случай, когда вызванная команда не вернула кода завершения (была прервана аварийно).


## 1.0.0 — 16.07.2021

### Изменено

- Собрано с обновлёнными зависимостями.


## 0.4.0 — 08.08.2019

### Добавлено

- Добавлен ключ `--ignore-status`.


## 0.3.4 — 24.07.2019

### Исправлено

- Более подробное сообщение об ошибке преобразования найденного значения.


## 0.3.3 — 05.07.2019

### Исправлено

- Показывался неправильный номер версии.


## 0.3.2 — 05.07.2019

### Исправлено

- ptgrep завершался с кодом 0, даже если вызванная команда вернула другой код.


## 0.3.1 — 19.06.2019

### Исправлено

- ptgrep завершался с ошибкой если в выводе не найдено совпадений с шаблоном.


## 0.3.0 — 19.06.2019

### Изменено

- Ключи `--lower` и `--higher` больше не являются обязательными.

### Добавлено

- Добавлен ключ `--write-env`.
- Добавлен вывод найденных значений.


## 0.2.0 – 17.06.2019

### Изменено

- В аргументах вызова ключ `--threshold` переименован в `--lower`.  

### Добавлено

- Добавлен ключ `--higher` в аргументы вызова.


## 0.1.0 – 16.06.2019

Первая версия.
