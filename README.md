# Ankit NEET Notice Notifier & Data Collector

**Purpose**: Watch for new notices on medical counselling websites, and to
collect data for later analysis

Basically 2 kaam hai iske:

1. Scrape websites like [MCC](https://mcc.nic.in/WebinfoUG/Page/Page?PageId=1&LangId=P), [Bihar UGMAC](https://bceceboard.bihar.gov.in/UGMACIndex.php), to fetch latest notices
   It runs as a daemon in background, and shows a notification when new notices
   are available

2. Data Collectors: Each data collector are separated into individual binaries,
   for easy use with `crontab`, so you can chose the update frequency.
   Currently, there is only one, for [NMC](https://www.nmc.org.in/), it parses
   the website for data on number of pg students etc, and saves them in a sql
   table

## Requisites

* `notify-send` - For notifiers
* `mysql` - For data collectors
* `linux` - Because I use it, so I tested it on linux only... only problem might
    be `xdg` though, you can probably fix it easily on windows

## Usage

Things which are daemons, or needed to be started once, I use systemd units.

For things, which are one-shot and require to be started repeatedly, I used
`fcrontab`... I could have used systemd timers, but crontab is easier

* For notifiers, I create a systemd service unit, probably inside
    `~/.local/share/systemd/user`, with the following content:

```systemd
[Unit]
Description=Ankit NEET Notice Notifier
After=mariadb.service
StartLimitBurst=5

[Service]
Restart=always
RestartSec=10s
Environment="LD_LIBRARY_PATH=/home/MYUNAME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib" "SQL_USERNAME=xxxxx" "DB_NAME=medicaldata" "TABLE_NAME=nmc"
ExecStart=/home/MYUNAME/.local/bin/notice-notifier-daemon

[Install]
WantedBy=default.target
```

* For the ones that require to be repeatedly run, do it with:

```sh
fcrontab -e    # or crontab -e
```

Then,

```crontab
# Run after every 10 minutes
*/10 * * * * /home/MYUNAME/.local/bin/kuchh-to-hoga
```

