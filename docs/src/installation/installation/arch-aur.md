# Arch Linux AUR Setup

YRBA provides an AUR package for Arch Linux to easily get started with YRBA on your system, and features a systemd
service and timers to have automatic updates with almost no setup required.

The AUR package is called `yrba-git`, and can be installed using an AUR helper of your choice.

https://aur.archlinux.org/packages/yrba-git

## Installation example using `paru`

If paru is the AUR helper of your choice, installing YRBA is as simple as running the following command.
```shell
paru -Syu yrba-git
```


---
This will automatically install the systemd service & timers as well. To set those up, please check out the systemd
instructions: [systemd configuration](../../configuration/systemd-config.md).
