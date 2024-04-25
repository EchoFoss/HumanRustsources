FROM ubuntu:latest
LABEL authors="fernandobalieiro"

ENTRYPOINT ["top", "-b"]