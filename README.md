Can be used instead of Linux watch.

install
```bash
$> tar xzvf rust_watch_Linux_amd64.tar.gz
$> cp rust_watch /usr/local/bin
```
use
```bash
$> rust_watch 5 "cat /tmp/log"
$> kill -9 $(pgrep "rust_watch")
```
