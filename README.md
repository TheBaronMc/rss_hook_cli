# Rss Hook Cli

## Description

Written in [Rust](https://www.rust-lang.org/fr), in order to make the app blazingly fast, this CLI allow to interact with the [Rss Hook Backend](https://github.com/TheBaronMc/rss_hook_backend).

## How to use

```
//$$$$$$$                     /$$   /$$                     /$$        /$$$$$$  /$$ /$$
| $$__  $$                   | $$  | $$                    | $$       /$$__  $$| $$|__/
| $$  \ $$  /$$$$$$$ /$$$$$$$| $$  | $$  /$$$$$$   /$$$$$$ | $$   /$$| $$  \__/| $$ /$$
| $$$$$$$/ /$$_____//$$_____/| $$$$$$$$ /$$__  $$ /$$__  $$| $$  /$$/| $$      | $$| $$
| $$__  $$|  $$$$$$|  $$$$$$ | $$__  $$| $$  \ $$| $$  \ $$| $$$$$$/ | $$      | $$| $$
| $$  \ $$ \____  $$\____  $$| $$  | $$| $$  | $$| $$  | $$| $$_  $$ | $$    $$| $$| $$
| $$  | $$ /$$$$$$$//$$$$$$$/| $$  | $$|  $$$$$$/|  $$$$$$/| $$ \  $$|  $$$$$$/| $$| $$
|__/  |__/|_______/|_______/ |__/  |__/ \______/  \______/ |__/  \__/ \______/ |__/|__/



Usage: rss_hook_cli [OPTIONS] <COMMAND>

Commands:
  flux        Actions related to RSS Flux (add, del, update)
  webhooks    Actions related to webhooks (add, del, update)
  articles    Actions related to articles (ls)
  hooks       Actions related to hooks (create, ls, del)
  deliveries  Actions related deliveries (ls)
  help        Print this message or the help of the given subcommand(s)

Options:
  -s, --server <SERVER>  [default: localhost]
  -p, --port <PORT>      [default: 3000]
  -n, --no-output        No output in terminal
  -e, --export <EXPORT>  export to CSV
  -h, --help             Print help
  -V, --version          Print version
```

### Flux

```
Actions related to RSS Flux (add, del, update)

Usage: rss_hook_cli flux <COMMAND>

Commands:
  add     Insert a new RSS Flux
  del     Remove a registered RSS Flux
  update  Update a registered RSS Flux
  ls      List all RSS Flux
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Add a new Flux

```
rss_hook_cli flux add <FLUX_URL>
```

Result:
```
Created with id <ID>
```

#### Remove a Flux

```
rss_hook_cli flux del <FLUX_ID>
```

#### Change the url of a flux

```
rss_hook_cli flux update <FLUX_ID> <FLUX_URL>
```

#### List all flux

```
rss_hook_cli flux ls
```

Example for result:
```
0 - id: 822, url: http://flux1.url
1 - id: 823, url: http://flux2.url
2 - id: 824, url: http://flux3.url
3 - id: 825, url: http://flux4.url
4 - id: 826, url: http://flux5.url
```


### Webhooks

```
Actions related to webhooks (add, del, update)

Usage: rss_hook_cli webhooks <COMMAND>

Commands:
  add     Insert a new Webhook
  del     Remove a registered Webhook
  update  Update a registered Webhook
  ls      List all webhooks
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Add a new Webhook

```
rss_hook_cli webhooks add <WEBHOOK_URL>
```

Result:
```
Created with id <ID>
```

#### Remove a webhook

```
rss_hook_cli webhooks del <FLUX_ID>
```

#### Change the url of a webhook

```
rss_hook_cli webhooks update <FLUX_ID> <FLUX_URL>
```

#### List all webhooks

```
rss_hook_cli webhooks ls
```

Example for result:
```
0 - id: 822, url: http://webhook1.url
1 - id: 823, url: http://webhook2.url
2 - id: 824, url: http://webhook3.url
3 - id: 825, url: http://webhook4.url
4 - id: 826, url: http://webhook5.url
```


### Hooks

```
Actions related to hooks (create, ls, del)

Usage: rss_hook_cli hooks <COMMAND>

Commands:
  create  Create a link between a RSS Flux and a webhook
  ls      List all links
  del     Remove a link
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### Bind a flux to a webhook

```
rss_hook_cli hooks create <FLUX_ID> <WEBHOOK_ID>
```

#### Remove a webhook

```
rss_hook_cli hooks del <FLUX_ID> <WEBHOOK_ID>
```

#### List all webhooks

```
rss_hook_cli hooks ls --flux-id <FLUX_ID> --webhook_id <WEBHOOK_ID>
```

Example for result:
```
0 - id: 822, url: http://webhook1.url
1 - id: 823, url: http://webhook2.url
2 - id: 824, url: http://webhook3.url
3 - id: 825, url: http://webhook4.url
4 - id: 826, url: http://webhook5.url
```


### Articles

```
Actions related to articles (ls)

Usage: rss_hook_cli articles <COMMAND>

Commands:
  ls    List all articles
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### List all articles

```
rss_hook_cli articles ls --flux-id <FLUX_ID>
```

Example for result:
```
0 - id: 822, url: http://article1.url, source: 758
1 - id: 823, url: http://article2.url, source: 386
2 - id: 824, url: http://article3.url, source: 759
3 - id: 825, url: http://article4.url, source: 758
4 - id: 826, url: http://article5.url, source: 758
```

### Deliveries

```
Actions related deliveries (ls)

Usage: rss_hook_cli deliveries <COMMAND>

Commands:
  ls    List all deliveries
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### List all deliveries

```
rss_hook_cli deliveries ls --article-id <ARTICLE_ID> --webhook-id <WEBHOOK_ID>
```

Example for result:
```
0 - id: 822, url: http://article1.url, source: 758
1 - id: 823, url: http://article2.url, source: 386
2 - id: 824, url: http://article3.url, source: 759
3 - id: 825, url: http://article4.url, source: 758
4 - id: 826, url: http://article5.url, source: 758
```

### Exception

Raise when something ring happen.

Format:
```
Error: Exception { statusCode: <CODE>, message: <EXPLANATION> }
```

## Build 

This application has been developped with rust `v1.6.6`.

```
git clone https://github.com/TheBaronMc/rss_hook_cli.git
cd rss_hook_cli

cargo test -- --test-threads=1 # Run tests

cargo build --release
# OR
cargo install
```

The binary must be in `./target/release/rss_hook_cli`.