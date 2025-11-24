# SMB

To use YRBA with SMB uploads configure the protocol in your `config.toml` to SMB, and set up the username to log in
with.

Example: remote = 'smb://user@127.0.0.1/share-name/path/to/my/backup/storage/directory'

Afterwards, configure the following settings as well in the `config.toml`:
```toml
# SMB Settings
# SMB user password
smb_password = 'my-super-secure-password'
```

**For a more in-depth explanation of the configuration parameters, check out
[Configuration](../../configuration/configuration.md)**
