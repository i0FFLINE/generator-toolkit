# USERDOC-ru.md

# Руководство пользователя

Проект: gentk

Полное название: Generator Toolkit

---

# Назначение

gentk — консольный набор инструментов для генерации:

- паролей;
- UUID;
- ULID;
- идентификаторов CUID2;
- Nano ID;
- TSID.

Программа работает в Linux, Windows и macOS.

---

# Генерация паролей

Один пароль (по умолчанию):

```text
gentk
```

Десять паролей:

```text
gentk 10
```

Двадцать паролей с цифрами:

```text
gentk --digits=true 20
```

Пароли со всеми группами, включая дополнительные символы:

```text
gentk --lower=true --upper=true --digits=true --special=true --extra=true 20
```

---

# Длина пароля

```text
gentk --length=32
```

Минимум: 8, по умолчанию: 13, максимум: 65535.

---

# Группы символов

Строчные буквы (a-z):

```text
--lower=true
```

Заглавные буквы (A-Z):

```text
--upper=true
```

Цифры (0-9):

```text
--digits=true
```

Безопасные специальные символы (@#%-_=+:,./):

```text
--special=true
```

Дополнительные специальные символы (!$^&*()[]{}|;'"\`<>?~):

```text
--extra=true
```

Группы можно комбинировать. Должна быть включена хотя бы одна группа.

---

# Ограничение повторов подряд

```text
--repeat=2
```

0 — без ограничений, максимум 3.  
Пример: `AA` разрешено, `AAA` — нет.

---

# Ограничение общего количества одинаковых символов

```text
--reuse=3
```

0 — без ограничений, максимум 3.

---

# Стратегия удовлетворения ограничений

```text
--strategy=retry   # retry (по умолчанию), slide, error
```

- `retry` — повторная генерация (до 1000 попыток).
- `slide` — замена нарушающих символов на лету.
- `error` — остановка с ошибкой.

---

# Исключение неоднозначных символов

```text
--exclude-ambiguous=true
```

Исключаются: `0 O o 1 l I 5 S 8 B`.

---

# JSON-вывод

Добавьте `--json` к любой команде.  
Одно значение: `{"value":"..."}`, несколько: `{"values":["...", "..."]}`.

Пример:

```text
gentk --json
gentk uuid4 3 --json
```

---

# UUID4

Сгенерировать один UUID4:

```text
gentk uuid4
```

Сто штук:

```text
gentk uuid4 100 --json
```

---

# UUID5

Требует пространства имён и имени. Если не указаны в командной строке, используются значения по умолчанию из конфигурации (`dns` и `example.com`, если конфиг существует).

Пример:

```text
gentk uuid5 --namespace=dns --name=example.com
```

Десять значений:

```text
gentk uuid5 --namespace=url --name=myapp 10
```

Поддерживаемые пространства имён: `dns`, `url`, `oid`, `x500` или UUID-строка.

---

# UUID7

```text
gentk uuid7
```

Пятьдесят:

```text
gentk uuid7 50 --json
```

---

# ULID

```text
gentk ulid
gentk ulid 100
```

---

# CUID2

```text
gentk cuid2
gentk cuid2 100
```

---

# Nano ID

Длина по умолчанию 21, алфавит по умолчанию (62 символа).

```text
gentk nanoid
```

Своя длина:

```text
gentk nanoid --length=32
```

Свой алфавит (минимум 2 символа):

```text
gentk nanoid --alphabet=abcdef123
```

Пятьдесят идентификаторов:

```text
gentk nanoid 50
```

---

# TSID

```text
gentk tsid
gentk tsid 100
```

---

# Конфигурация

Создать файл конфигурации по умолчанию:

```text
gentk config
```

Записывает `gentk.json` в текущий каталог. Пример содержимого:

```json
{
  "password": {
    "length": 13,
    "lower": true,
    "upper": true,
    "digits": false,
    "special": false,
    "extra": false,
    "repeat": 0,
    "reuse": 0,
    "exclude_ambiguous": false,
    "strategy": "retry"
  },
  "uuid5": {
    "namespace": "dns",
    "name": "example.com"
  },
  "nanoid": {
    "length": 21,
    "alphabet": null
  }
}
```

Порядок поиска:
1. Встроенные значения по умолчанию
2. `./gentk.json`
3. `~/.config/gentk/config.json`
4. `/etc/gentk/config.json`
5. Аргументы командной строки (наивысший приоритет)

---

# Справка

Основная:

```text
gentk --help
```

По конкретной команде:

```text
gentk --help config
gentk --help uuid5
gentk --help nanoid
...
```

Все страницы показывают допустимые диапазоны (например, `--repeat <0-3>`, `--length <8-65535>`).

---

# Версия

```text
gentk --version
```

Release-сборки показывают полную версию, debug-сборки добавляют суффикс `-dev`.

---

# Лицензия

MIT
