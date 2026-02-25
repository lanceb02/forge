# Package Manager

## Usage

forge add <repo>

creates a config for the package based on the repo name.

#TODO: handle namespace conflict

config options:

update = <no, live, tagged>
build = "build command"
install = "install command"
uninstall = "uninstall command"
clean = "clean command"
deps = ["named list of deps"]

forge update

pulls latest for all tracked packages per update rules

forge upgrade [pkgnames]

If no names are provided assume all otherwise do build and install on named packages

forge remove [pkgnames]

Removes named packages and orphaned deps

forge list

lists all packages currently tracked

forge search [term]

lists packages with the given substring not case sensitive

forge clean [pkgnames]

runs the clean command on every tracked package or named

forge show [pkgname]

shows the configured info about the package
