# PTGREP

[![Состояние сборки](https://travis-ci.org/mekras/ptgrep.svg?branch=master)](https://travis-ci.org/mekras/ptgrep)

Запускает произвольные консольные команды, ищет в их выводе значения по указанным шаблонам и
завершается с ошибкой, если эти значения не укладываются в заданные границы.

PTGREP появился чтобы при использовании GitLab иметь возможность задавать требования к качеству
кода. Например, чтобы сборка проваливалась если покрытие кода тестами ниже определённого порога или
в оформлении найдено слишком много ошибок.

## Использование

В общем случае вызов ptgrep выглядит так:

    ptgrep --pattern=<шаблон> <действия> -- <команда с аргументами>

Пример: *провалить сборку если покрытие кода тестами PHPUnit меньше 80%*

    ptgrep --pattern='\n\s+Lines:\s*(?P<coverage>[\d\.]+)%' --lower=coverage=80 -- vendor/bin/phpunit --coverage-text --colors=never
      
Пример: *провалить сборку если PHP_CodeSniffer нашёл больше 10 ошибок **или** больше 20 предупреждений*

    ptgrep --pattern='OF (?P<errors>\d+) ERRORS AND (?P<warnings>\d+)' --higher=errors=10 --higher=warnings=20 -- vendor/bin/phpcs --report-summary --no-colors

Пример: *записать количество ошибок и предупреждений PHP_CodeSniffer в файл*

    ptgrep --pattern='OF (?P<errors>\d+) ERRORS AND (?P<warnings>\d+)' --write-env=checkstyle.env -- vendor/bin/phpcs --report-summary --no-colors

### --pattern

Ключ `--pattern` позволяет указать регулярное выражение, которое *ptgrep* будет искать в выводе
вызываемой команды.

- [Синтаксис регулярных выражений](https://docs.rs/regex/latest/regex/#syntax)

В выражении надо **обязательно** указать хотя бы одну
[именованную группу захвата](https://docs.rs/regex/latest/regex/#example-replacement-with-named-capture-groups).
Такие группы задаются в формате `(?P<имя_группы>выражение)`.
Имена групп затем можно использовать в других ключах, таких как `--lower` и `--higher`.

### --lower, --higher

Ключи `--lower` и `--higher` задают соответственно нижний и верхний порог некоторого значения.
Формат значений ключей: `имя_параметра=число`. Имя параметра — это имя группы захвата из ключа
`--pattern`. Примеры:

- `--lower=foo=10`
- `--higher=foo=12.34`
- `--lower=foo=0 --lower=bar=0.5 --higher=baz=1000`

### --write-env

Записывает значения, соответстсующие шаблону `--pattern`, в указаннный файл в формате `.env`.
Например команда

    ptgrep --pattern='OF (?P<errors>\d+) ERRORS AND (?P<warnings>\d+)' --write-env=checkstyle.env -- vendor/bin/phpcs --report-summary --no-colors

запишет в файл `checkstyle.env` примерно следующее:

    ERRORS=668
    WARNINGS=32
 
### Команда

Вызываемая команда должна указываться после всех ключей и отделяться от них двумя дефисами — `--`:
