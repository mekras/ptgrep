# PHP_CodeSniffer

Создание значка с количеством ошибок при помощи [anybadge](https://pypi.org/project/anybadge/):

    ptgrep --pattern='OF (?P<errors>\d+) ERRORS' --higher=errors=$STYLE_ERRORS_MAX --write-env=checkstyle.result \
      -- php vendor/bin/phpcs --report-summary --no-colors
    . checkstyle.result && anybadge -l checkstyle -v $ERRORS -f checkstyle.svg 0=green 10=yellow 100=orange 9999999=red
