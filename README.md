# Basedpyright Extension for Zed

## Pre-requisites

* Python 3.9+
* `basedpyright` installed into your python environment

## Installation

Search `basedpyright` in the zed extensions. Click to install.

## Enable

Disable `pyright` and enable `basedpyright` in your settings.

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["basedpyright", "!pyright"]
  },
}
```

## Configure

Configure under `lsp.basedpyright.settings` as required.

```jsonc
{
  "lsp": {
    "basedpyright": {
      "settings": {
        "python": {
          "pythonPath": ".venv/bin/python"
        },
        "basedpyright.analysis": {
          "diagnosticMode": "workspace",
          "inlayHints": {
            "callArgumentNames": false
          }
        }
      }
    }
  }
}
```
