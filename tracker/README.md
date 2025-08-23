<!-- deno-fmt-ignore-file -->
# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML,
CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) +
  [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) +
  [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Set up

deno install 
deno task tauri android init

for Desktop development, run: deno task tauri dev

you'll want to set up an api key to authenticate with your api server so only your computers can send your data to your api.

create api key using /utils/keygen.ts (at root) with `deno task keygen`

then use api key in /tracker/.env (create if haven't already)

```
API_KEY="your key here"
```

in /api, create .env following .env.example and paste in the same api key
