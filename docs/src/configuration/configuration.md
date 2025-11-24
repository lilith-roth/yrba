# Configuration

The configuration of YRBA is rather simple, and requires just a few adjustments to get going.

The default location the configuration is read from is `~/.config/yrba/config.toml`, and `/etc/yrba.toml` if run as the
*root* user, and can be changed by starting the executable with the `--config /path/to/config/file` or 
`-c /path/to/config/file` flag.

If the application has been started *without* the `-c` flag, and the default location does not contain a configuration
file, it will automatically create an example file.


An example configuration file looks somewhat like this:
```toml
# Remote address for uploading backups
# SFTP: 'sftp://root@127.0.0.1/path/to/my/backup/storage/directory'
# SMB: 'smb://user@127.0.0.1/share-name/path/to/my/backup/storage/directory'
# FILE: 'file:///path/to/my/local/backup/storage/directory'
remote = 'sftp://root@127.0.0.1/path/to/my/backup/storage/directory'

# The amount of backups to keep
# Set to 0 to never delete old backups
amount_of_backups_to_keep = 5

# SFTP Settings (only used if remote string above is set to sftp protocol)
# if both public key path & password is defined, first the private key authentication is tried,
# and if that fails the password is tried next.
sftp_pubkey_path = '/auth/id_ed25519.pub'
sftp_private_key_path = '/auth/id_ed25519'
sftp_private_key_password = 'my-super-secure-password'
sftp_password = 'my-super-secure-password'

# SMB Settings
# SMB user password
smb_password = 'my-super-secure-password'

# Path to folders to back up, supports relative paths
folders_to_backup = [
    '/backup'
]

# Optional: Temporary folder to store archive before upload
# Default: '~/.cache/yrba'
# temporary_folder = '/tmp/yrba'
```

**Tip:** Use `'` instead of `"` for the path strings in the file configuration. `'` refers to a raw string in `.toml`
files, which makes it allows the use of certain special characters without escaping them. Especially useful when the
application runs on Windows due to the nature of Windows directory paths often containing `\` characters that would
otherwise need to be escaped with another backslash like `\\`.

The example file can be found in the root directory of the GitHub repository.
https://github.com/lilith-roth/yrba/blob/main/config.example.toml


## Explanation

### `remote`
This configures the target to upload your backups to incl. path on the target system to which your backups
will be uploaded.

It is intended to be written in a URL format.

Example: `remote = 'sftp://user@127.0.0.1/path/to/my/backup/storage/directory'`

**Note:** The password can be omitted if a private key is used for authentication


### `amount_of_backups_to_keep`
This defines the amount of backups that will be kept on the remote server, older ones will be deleted.

Set this to `0` to never remove old backups.

Example: `amount_of_backups_to_keep = 5`


### SFTP Options
#### `sftp_public_key_path`
If SFTP private key authentication is used set this to the path of your public key.

`sftp_pubkey_path = '~/.ssh/id_ed25519.pub'`


#### `sftp_private_key_path`
If SFTP private key authentication is used set this to the path of your private key.

`sftp_public_key_path = '~/.ssh/id_ed25519'`


#### `sftp_private_key_password`
If SFTP private key authentication is used, and your private key is protected with a password, set the password for your
key here.

`sftp_public_key_path = '~/.ssh/id_ed25519'`


#### `sftp_password`
If SFTP password authentication is used, enter the password of your user here.

`sftp_password = 'my-super-secure-password'`


### SFTP Options
#### `smb_password`
User of the smb user connecting to the remote storage system.

`smb_password = 'my-super-secure-password'`


### `folders_to_backup`
Configure the folders you want to back up here.

If a docker setup is used, set this to the folders that are mounted into your container.

Example:
```toml
folders_to_backup = [
    '/backup',
    '~/all-my-data'
]
```


### Optional: `temporary_folder`
Default: `~/.cache/yrba`
If you would like to change the temporary folder YRBA will use to archive your backups to, define them here.

Example: `temporary_folder = '/tmp/yrba'`
