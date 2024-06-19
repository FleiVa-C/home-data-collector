### Building and running your application

The images can be built with buildx and bake:\
`docker buildx bake -f <path_to_bake_file>`\

By default the images will be built for the architectures ["linux/amd64", "linux/arm64/v8", "linux/arm/v7"].

To save the images locally, the tags must be overwritten with the following flag:
`--set server.tags=<image_name_server> --set collector.tag=<imagename_collector>`

To avoid building for multiple platforms you can overwrite the build platform aswell:
`--set *.platform=<architecture>`

The two applications will need a config mapped into the container. For development, the *_template.yml are enough to run the containers.

After building you can run the application locally by running `docker compose up`\
Your application will be available at http://localhost:8080.

### Deploying your application to the cloud

To deploy the application in the cloud, the images can be pushed to a registry by specifying the image tag as described above.
Make sure you are logged in to the registry before building with bake (see [Docker Login](https://docs.docker.com/reference/cli/docker/login/)).