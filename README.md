## git-prune-branches

`git-prune-branches` is a text user interface to ease deletion of Git branches.
Call

    $ git prune-branches

and you will be presented a list of all branches except for `master`. Scroll the
list with <kbd>j</kbd> and <kbd>k</kbd> and select branches with the
<kbd>space</kbd>. After exiting with <kbd>q</kbd> all selected branches will be
deleted.


### Installation

Install Rust and Cargo and run `make install`. It understands the `DESTDIR`
variable so call

    $ DESTDIR=~/.local make install

to install the binary into the user's home directory.
