# DDNS Client

A simple DDNS client updater.

## Supported DDNS Servers

- (Duck DNS)[https://www.duckdns.org]

## How to use it

```shell
ddns_client ~/.config/ddns_client/config.yaml -t 600
```

## Config sample

```yaml
domain: YOUR_DOMAIN
token: xxxx-xxxx-xxxx-xxxx
verbose: true
clear: true
```
