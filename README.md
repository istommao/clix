# clix
command alias extension

## Feature

- [x] show command alias list
- [x] create new command alias
- [x] show command alias
- [x] rename command alias
- [x] update command alias
- [x] run command


## Install

```bash
curl -s https://sh.rustup.rs | clix
```

## Quick start

```bash
# show command alias list
clix show list

# create new command alias
clix new [alias name] [Your Command]

# show command alias
clix show [alias name]

# rename command alias
clix rename [alias old name] [new name]

# update command alias
clix update [alias name] [New Command]

# run command
clix run [alias name]
```
