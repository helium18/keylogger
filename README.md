# keylogger
Keylogger for termput which is a WIP

## How does it work?
Reads the keyboard input at `/dev/input/` and parses it

## Safe?
Yes since the input isn't stored anywhere on the machine. As and when the program is killed, so are all the logs. Also, it doesn't make any HTTP requests so everything stays on your machine.

## Building
No build instructions (I don't want people to build it on their machines. This project would serve as the base for another project which is a WIP)
