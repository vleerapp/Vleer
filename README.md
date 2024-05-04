# Vleer Rewrite 

This is a full rewrite of Vleer with the same stack but cleaned up. And with proper rust implementations.

![Alt](https://repobeats.axiom.co/api/embed/476c97ad30ff96e3217cf756e84c292836b8f44e.svg "Repobeats analytics image")

***

For signing on macOS builds:
```zsh
codesign -s - --force --deep --timestamp --options runtime src-tauri/target/release/bundle/macos/Vleer.app
```
