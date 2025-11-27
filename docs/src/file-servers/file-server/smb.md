# SMB

## Configuration
To use YRBA with SMB uploads configure the protocol in your `config.toml` to SMB, and set up the username to log in
with.

Example: 
```toml
remote = 'smb://user@127.0.0.1/share-name/path/to/my/backup/storage/directory'
```

Afterwards, configure the following settings as well in the `config.toml`:
```toml
# SMB Settings
# SMB user password
smb_password = 'my-super-secure-password'
```

**For a more in-depth explanation of the configuration parameters, check out
[Configuration](../../configuration/configuration.md)**


### Guest authentication

To authenticate as a guest user against the smb server set the username to `guest`, and don't set a password field, or
set `smb_password = ''`.


### Full configuration example

A complete file copy backup configuration could look like this.
```toml
# Remote address for uploading backups
# SMB: 'smb://user@127.0.0.1/share-name/path/to/my/backup/storage/directory'
remote = 'smb://user@127.0.0.1/share-name/path/to/my/backup/storage/directory'

# SMB Settings
# SMB user password
smb_password = 'my-super-secure-password'

# The amount of backups to keep
# Set to 0 to never delete old backups
amount_of_backups_to_keep = 5

# Path to folders to back up, supports relative paths
folders_to_backup = [
    "/backup"
]
```