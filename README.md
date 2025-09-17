# Fragekasten

> [!CAUTION]  
> **This project is made for me, my needs, and my infrastructure.**
>
> No support will be offered for this software. Breaking changes to functionalty or features may be made any time. I may decide to support this project properly in the future.

A simple 'ask me anything' site using Discord webhook-powered notifications.

## Setup

### Docker

1. Copy [compose.yml](./compose.yml) to a local file named `compose.yml` or add the
   service to your existing stack and fill in the environment variables.
   Information about configuration options can be found in the
   [configuration](#configuration) section.

2. Start the stack

```
docker compose up -d
```

### Manual

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed and
   in your `$PATH`.
2. Install the project binary

```
cargo install --git https://codeberg.org/Blooym/fragekasten.git
```

3. Set configuration values as necessary.
   Information about configuration options can be found in the
   [configuration](#configuration) section.

4. Run the binary either with environment variables set or with your desired configuration flags
   ```
   fragekasten <flags>
   ```

## Configuration

Fragekasten is configured via command-line flags or environment variables and has full support for loading from .env files. Below is a list of all supported configuration options. You can also run fragekasten --help to get an up-to-date including default values.

| Name                      | Description                                                                                                                                                                                                                                                                                                                                                      | Flag                                                      | Env                                     | Default                                                |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- | --------------------------------------- | ------------------------------------------------------ |
| Address                   | Internet socket address that the server should be ran on                                                                                                                                                                                                                                                                                                         | `--address <ADDRESS>`                                     | `FRAGEKASTEN_ADDRESS`                   | `127.0.0.1:6251`                                       |
| Database URL              | SQLite database connection string to use for temporarily storing questions                                                                                                                                                                                                                                                                                       | `--database-url <DATABASE_URL>`                           | `DATABASE_URL`                          | `sqlite://logs.db?mode=rwc`                            |
| IP Source                 | ClientIpSource to use when obtaining the users's IP address. Defaults to ConnectInfo, although this will not work when behind a Reverse Proxy and should be changed accordingly. See [here](https://github.com/imbolc/axum-client-ip/blob/6d970edce4f7f0d1782e328fe688e021c42f1f3e/README.md#configurable-vs-specific-extractors) for details on accepted values | `--ip-source <IP_SOURCE>`                                 | `FRAGEKASTEN_IP_SOURCE`                 | `ConnectInfo`                                          |
| Discord Webhook URL       | Discord Webhook URL to send asked questions to                                                                                                                                                                                                                                                                                                                   | `--discord-webhook-url <DISCORD_WEBHOOK_URL>`             | `FRAGEKASTEN_DISCORD_WEBHOOK_URL`       | -                                                      |
| Discord User ID           | Discord User ID (not name) to mention when sending asked questions                                                                                                                                                                                                                                                                                               | `--discord-user-id <DISCORD_USER_ID>`                     | `FRAGEKASTEN_DISCORD_USERID`            | -                                                      |
| Page Title                | The title to use for the questions page                                                                                                                                                                                                                                                                                                                          | `--page-title`                                            | `FRAGEKASTEN_PAGE_TITLE`                | -                                                      |
| Page Description          | The description to use for the questions page. Supports inline HTML tags.                                                                                                                                                                                                                                                                                        | `--page-description`                                      | `FRAGEKASTEN_PAGE_DESCRIPTION`          | -                                                      |
| Page Theme Colour         | The theme colour to use for the questions page. This can be a hex code, an rgb(x, x, x) or any other CSS-accepted format. You should pick a colour with good contrast on both light and dark backgrounds.                                                                                                                                                        | `--page-theme-colour`                                     | `FRAGEKASTEN_PAGE_THEME_COLOUR`         | `#dc64ffff`                                            |
| Page Owner Name           | The name of the owner of the page - you probably want to use your online username                                                                                                                                                                                                                                                                                | `--page-owner-name <PAGE_OWNER_NAME>`                     | `FRAGEKASTEN_PAGE_OWNER_NAME`           | -                                                      |
| Page Question Min Length  | The minimum length a question is allowed to be                                                                                                                                                                                                                                                                                                                   | `--page-question-min-length <PAGE_QUESTION_MIN_LENGTH>`   | `FRAGEKASTEN_PAGE_QUESTION_MIN_LENGTH`  | `15`                                                   |
| Page Question Max Length  | The maximum length a question is allowed to be                                                                                                                                                                                                                                                                                                                   | `--page-question-max-length <PAGE_QUESTION_MAX_LENGTH>`   | `FRAGEKASTEN_PAGE_QUESTION_MAX_LENGTH`  | `300`                                                  |
| Page Question Placeholder | The placeholder text to use in the question ask box, this can be anything you want                                                                                                                                                                                                                                                                               | `--page-question-placeholder <PAGE_QUESTION_PLACEHOLDER>` | `FRAGEKASTEN_PAGE_QUESTION_PLACEHOLDER` | `"Would you like to hold hands in the rain together?"` |
