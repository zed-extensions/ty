# ty: `astral-sh/ty` for Zed

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
  },
}
```

## Configure

Configure under `lsp.ty.settings` as required. The "binary" setting must be filled in.

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
```
