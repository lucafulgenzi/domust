
# Domust

Domust is a home automation system that allows you to control your home form your computer.

## Installation

### Arch Linux

```bash  
yay -S domust
```  

### Cargo

```bash
cargo install domust
```

## Disclaimer

#### Broadlink
Domust works with [broadlinkmanager](https://github.com/t0mer/broadlinkmanager-docker) to control broadlink devices. You need to have it running in order to use Domust.

## Configuration

### Device types
- Broadlink

```bash  
touch ~/.config/domust/config.yaml  
```  

```yaml  
# ~/.config/domust/config.yaml  
  
broadlink:
    manager_url: <URL>  
    device_type: <TYPE>  
    device_ip: <IP>  
    device_mac: <MAC>

devices:
  - name: <DEVICE_NAME>
    device_type: <DEVICE_TYPE>
    commands:
      - name: <COMMAND_NAME>
        code: <HEX_CODE>
      - name: <COMMAND_NAME>
        code: <HEX_CODE>
```
