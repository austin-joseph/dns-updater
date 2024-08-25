docker build -t dns-updater . -f ./docker/Dockerfile
docker container stop dns-updater
docker container rm dns-updater
docker run -itd --name dns-updater dns-updater