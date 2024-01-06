# Examples

## Setup

Create a `.env` file in this repository:

```sh
ZOSMF_BASE_URL=https://mainframe.my-company.com:1234 # everything before the `/zosmf`
ZOSMF_USERNAME=USERNAME
ZOSMF_PASSWORD=PASSWORD
```

## SSL/TLS

In the examples, the client will use the certificates of your machine by default. If the z/OSMF REST API uses another root certificate, you can provide the path to that certificate in the `.env` file:

```sh
ZOSMF_CERT_PATH=/path/to/my/zosmf/cert.pem
```
