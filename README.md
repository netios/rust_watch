Can be used instead of Linux watch.

install
1. tar xzvf rust_watch_Linux_amd64.tar.gz
2. cp rust_watch /usr/local/bin

use
```bash
$> rust_watch 5 "cat /tmp/log"
$> kill -9 $(pgrep "rust_watch")
```
