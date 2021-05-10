## git-prune-branches

`git-prune-branches` is a text user interface to ease deletion of Git branches.
Call

    $ git prune-branches

and you will be presented a list of all branches except for `master`. Scroll the
list with <kbd>j</kbd> and <kbd>k</kbd> and select branches with the
<kbd>space</kbd>. After exiting with <kbd>q</kbd> all selected branches will be
deleted.


## git-pick

`git-pick` is a text user interface for the Git cherry pick command. Given a
branch name on the command line

    $ git pick <branch>

it will show a list of commits differing between the HEAD and the given branch:

Choose commits to cherry-pick and accept with <kbd>q</kbd>

    [ ] 13579ef: A commit message
    [•] fa45678: Another commit message

Commits can be picked with Enter and will be passed unconditionally to the `git
cherry-pick` command.


## Installation

Install Rust and Cargo and run `make install`. It understands the `DESTDIR`
variable so call

    $ DESTDIR=~/.local make install

to install the binary into the user's home directory.
