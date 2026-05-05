# SFTP

## Configuration

To use YRBA with SFTP uploads configure the protocol in your `config.toml` to SFTP, and set up the username to log in
with.

**NOTE:** The remote system *has* to be a unix like system like Linux or macOS due to the required cli commands on the
remote machine!

Example: 
```toml
remote = 'sftp://root@127.0.0.1/path/to/my/backup/storage/directory'
```

Afterwards, configure the following settings as well in the `config.toml`:
```toml
# SFTP Settings (only used if remote string above is set to sftp protocol)
# if both public key path & password is defined, first the private key authentication is tried,
# and if that fails the password is tried next.
sftp_public_key_path = '~/.ssh/id_ed25519.pub'
sftp_private_key_path = '~/.ssh/id_ed25519'
sftp_private_key_password = 'my-super-secure-password'
sftp_password = 'my-super-secure-password'
```

**Note:** Password and private key authentication can both be configured to try logging in with password, in case
private key authentication fails.

**For a more in-depth explanation of the configuration parameters, check out
[Configuration](../../configuration/configuration.md)**


### Note
Make sure the system you're uploading your backups to has the following shell commands available:
- tail
- ls
- grep
- xargs
- cd


### Full configuration example

A complete file copy backup configuration could look like this.
```toml
# Remote address for uploading backups
# SFTP: 'sftp://root@127.0.0.1/path/to/my/backup/storage/directory'
remote = 'sftp://root@127.0.0.1/path/to/my/backup/storage/directory'

# SFTP Settings (only used if remote string above is set to sftp protocol)
# if both public key path & password is defined, first the private key authentication is tried,
# and if that fails the password is tried next.
sftp_public_key_path = '~/.ssh/id_ed25519.pub'
sftp_private_key_path = '~/.ssh/id_ed25519'
sftp_private_key_password = 'my-super-secure-password'
sftp_password = 'my-super-secure-password'

# Optional: Enable SFTP compression
# Default: true
sftp_compression_enabled = true

# The amount of backups to keep
# Set to 0 to never delete old backups
amount_of_backups_to_keep = 5

# Path to folders to back up, supports relative paths
folders_to_backup = [
    "/backup"
]
```
