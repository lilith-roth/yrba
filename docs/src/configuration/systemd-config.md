# systemd Configuration

YRBA provides a systemd service & timers for automated frequent backups.

If YRBA is installed using the AUR it will automatically install the necessary files, otherwise it is possible to set
those up yourself, but this is only recommended if you know what you're doing.

The systemd files for setting this up by yourself can be found here, but for the sake of simplicity, this guide won't
cover on how to set these up with your systemd manually.
[GitHub systemd files](https://github.com/lilith-roth/yrba/tree/main/systemd)


## Enabling automatic backups using systemd

YRBA provides 3 timers that can be enabled with a single line for daily, weekly and monthly backups respectively.

**To enable daily backups use:**
```shell
systemctl enable --now yrba.daily.timer
```

**To enable weekly backups use:**
```shell
systemctl enable --now yrba.weekly.timer
```

**To enable monthly backups use:**
```shell
systemctl enable --now yrba.monthly.timer
```


**To verify if they've been enabled correctly type:**
```shell
systemctl status yrba.daily.timer # Replace `daily` with the corresponding activated timer
```

**In case log messages for the application are desired, they can be viewed using:**
```shell
journalctl -xefu yrba.service
```
