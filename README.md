# Vleer 

This is a full rewrite of Vleer with the same stack but cleaned up. And with proper rust implementations.

![Alt](https://repobeats.axiom.co/api/embed/476c97ad30ff96e3217cf756e84c292836b8f44e.svg "Repobeats analytics image")

***

For signing on macOS builds:
```zsh
codesign -s - --force --deep --timestamp --options runtime src-tauri/target/release/bundle/macos/Vleer.app
```
or
```zsh
# build first
cd src-tauri/target/release/bundle/dmg/
hdiutil attach -nobrowse -mountpoint /Volumes/vleer ./Vleer*.dmg
cp -R /Volumes/vleer .
hdiutil detach /Volumes/vleer
codesign -s - ./vleer/Vleer.app/Contents/MacOS/vleer
hdiutil create -format UDZO -srcfolder ./vleer Vleer-signed.dmg
```


To convert all webm files to mp3:
```pwsh
# windows
for %i in (*.webm) do ffmpeg -i "%i" "%~ni.mp3"
```
```zsh
# unix (linux/mac)
for i in *.webm; do ffmpeg -i "$i" "${i%.webm}.mp3"; done
```
