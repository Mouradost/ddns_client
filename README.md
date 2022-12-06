# DDNS Client

A simple DDNS client updater.

## Supported DDNS Servers

- [Duck DNS](https://www.duckdns.org)

## How to use it

```shell
ddns_client ~/.config/ddns_client/config.yaml -t 600
```

## Config sample

```yaml
# Domain name
domain: YOUR_DOMAIN
# DDNS server token
token: xxxx-xxxx-xxxx-xxxx
# Request confirmation from the server
verbose: true
# Clear all ip records from the server
clear: true
```
