# Docker

При использовании в CI может быть удобно включить ptgrep в используемый образ Docker.

Пример:

```Dockerfile
ARG PTGREP_VERSION=1.0.1
ARG PTGREP_SHA256=6d3953c0798ba170ebfe5be79e49084e04775f10bcf2ede8398b1093031599e1

ADD https://github.com/mekras/ptgrep/releases/download/${PTGREP_VERSION}/ptgrep /usr/local/bin/

RUN set -eux; \
    cd /usr/local/bin; \
    echo "${PTGREP_SHA256}" ptgrep | sha256sum -c -; \
    chmod a+x ptgrep
```
