To run first argument must be location of a config file matching the format of config.json.example



docker build -t dns-updater . -f ./docker/Dockerfile
docker run -itd --restart unless-stopped --name dns-updater dns-updater