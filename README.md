# Google DDNS

## Build

```shell
cargo build --release
```

## Systemd Service Setup

```shell
mkdir -p ~/.config/systemd/user/
```

```shell
cp google-ddns.service.template ~/.config/systemd/user/google-ddns.service
```

```shell
nano ~/.config/systemd/user/google-ddns.service
```

```shell
sysu start google-ddns.service
```

```shell
sysu enable google-ddns.service
```

