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

## Run ddns_client as a service

DDNS Client can run as a system service.

### Linux

- Download the latest release [here](https://github.com/Mouradost/ddns_client/releases)
- Copy the binary file to `/usr/bin/` using the following command

```shell
sudo cp ./ddns_client /usr/bin/

```

- Create a service descriptor file as shown

```shell
touch ddns_client.service

```

```
[Unit]
Description=DDNS Client updater.
After=network.target

[Service]
Type=simple
ExecStart=/usr/bin/ddns_client /home/[USERNAME]/.config/ddns_client/config.yaml
Restart=always
RestartSec=14400
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=%n

[Install]
WantedBy=multi-user.target
```

- Copy the file to `/etc/systemd/system/`

```shell
sudo cp ddns_client.service /etc/systemd/system/
```

- Reload systemctl deamon

```shell
sudo systemctl daemon-reload
```

- Enable the service

```shell
sudo systemctl enable ddns_client.service

```

- Start the service

```shell
sudo systemctl start ddns_client.service
```

- Check service status

```shell
sudo systemctl status ddns_client.service
```

- Check the service output

```shell
journalctl -u ddns_client.service
```
