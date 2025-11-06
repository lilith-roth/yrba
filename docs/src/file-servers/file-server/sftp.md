# SFTP

To use YRBA with SFTP uploads configure the protocol in your `config.toml` to SFTP, and set up the username to log in
with.

Example: remote = "sftp://root@127.0.0.1/path/to/my/backup/storage/directory"

Afterwards, configure the following settings as well in the `config.toml`:
```toml
# SFTP Settings (only used if remote string above is set to sftp protocol)
# if both public key path & password is defined, first the private key authentication is tried,
# and if that fails the password is tried next.
sftp_pubkey_path = "~/.ssh/id_ed25519.pub"
sftp_privkey_path = "~/.ssh/id_ed25519"
sftp_privkey_password = ""
sftp_password = ""
```

**Note:** Password and private key authentication can both be configured to try logging in with password, in case
private key authentication fails.

**For a more in-depth explanation of the configuration parameters, check out [Configuration](../../configuration.md)** 
