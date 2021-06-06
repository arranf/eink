#!/bin/bash

## This command creates a FUSE link to the remote Pi. When you are done unmount the folder.
## It assumes raspberry.local is a valid address to access the pi and that there is a root directory /pi where the pi can be mounted and that you have setup a fuse user group that the current user is in
## More info on ssfs: https://unix.stackexchange.com/a/106485

sshfs pi@raspberry.local:/home/pi ~/pi

# To unmount: fusermount -u ~/pi or (OSX) umount ~/pi