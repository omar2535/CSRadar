# Omar's CS Radar Hack in rust

Offsets: [CS2-OFFSETS](https://github.com/sezzyaep/CS2-OFFSETS)

Not much else here, still a WIP

## Setup

Backend

```ps1
cargo build
```

Frontend

```ps1
npm install -g
npm run build
```

## Development

To supress errors:

**Powershell:**

Backend:

```ps1
cargo run 2>$null
```

Frontend:

```ps1
cd frontend-react
serve -s build
```

**Bash:**

```sh
cargo run 2> /dev/null
```

## To update offsets

```ps1
./bin/update_offsets.ps1
```
