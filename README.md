# Home Data Collector

The "home data collector" is a personal IOT project to collect data from sensors which are registered in the local network.
## Description

This project is tailored to personal needs and sensor equipment used and does not suit a general application.
It is used to have an on premise database of all the IOT devices, rather than having to send their data to the cloud of the device.
The core component is the actix web backend service that provides REST API endpoints for operations like registering a new device, ingesting timeseries data or querying timeseries/meta data of the sensors.
To store all data (not only timeseries data), a surrealdb instance is used. Collection of the timeseries data is managed by a collector service that requests the sensor data at equal intervals and ingests them through the backend REST API's.

## Notes
This project is not finished or ready to be run at all. Therefore the chapter "Deployment" is just a placeholder on what i think will be needed or is yet to be created.
Since i work alone in this project, branches/pull requests/merges would not be needed but it was for me to learn git. Also i use issues to track on what is on my mind.

## Deployment

The project is designed to run in a k3s kubernetes cluster once the first version is done.

### Dependencies

* k3s (on device you want to deploy the project to)
* kubectl (on the control device)
* docker-desktop

### Running the service

* build the docker image
```
docker build --tag <chosen_name_for_image> .
```
* deploy the service to k3s
    * see example .yaml files

## Author

* [@FleiVa-c](https://github.com/FleiVa-C)

## Version History
* no "release" yet - see [Notes] (#notes)

## License

This project is licensed under the [MIT] License - see the `LICENSE.md` file for details.

## Acknowledgments

* [awesome-readme](https://github.com/matiassingers/awesome-readme)
* [example backend service with actix-web - by Code to the moon](https://www.youtube.com/watch?v=L8tWKqSMKUI&list=PLqnVCl9hPjM4wvPyuRerufBmaOTx7OMLo&index=5&t=938s)
