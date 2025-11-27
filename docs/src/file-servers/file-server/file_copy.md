# File copy backup

## Configuration

If you want your backup to be stored by copying to a different directory, for example to store backups on an external
drive, you can use the file protocol as your remote string in the YRBA configuration.

Example:
```toml
remote = "file:///path/to/my/backup/storage/directory"
```

**For a more in-depth explanation of the configuration parameters, check out
[Configuration](../../configuration/configuration.md)**


### Full configuration example

A complete file copy backup configuration could look like this.
```toml
# Remote address for uploading backups
# File copy: "file:///path/to/my/backup/storage/directory"
remote = "file:///path/to/my/backup/storage/directory"

# The amount of backups to keep
# Set to 0 to never delete old backups
amount_of_backups_to_keep = 5

# Path to folders to back up, supports relative paths
folders_to_backup = [
    "/backup"
]
```
