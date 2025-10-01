# ty: `astral-sh/ty` for Zed

**This extension is deprecated. Ty support is now built into Zed**

This extension provides [`ty`](https://github.com/astral-sh/ty), an extremely fast Python type checker and language server, for Zed editor.

## Installation

Open Zed extensions page, and search `ty` to install.

## Enable

Enable `ty` in your settings.

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["ty"]
    }
  }
}
```

## Configure

This extension will look for `ty` in your path or will automatically download the appropriate binary from the [astral-sh/ty releases](https://github.com/astral-sh/ty/releases).

If you prefer to use a custom binary or arguments you can alternatively add the following your zed settings:

```jsonc
{
  "lsp": {
    "ty": {
      "binary": {
        "path": "/Users/yourname/.local/bin/ty",
        "arguments": ["server"]
      }
    }
  }
}
```
