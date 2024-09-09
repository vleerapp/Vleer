<div align="center">

<img width="128px" src="src-tauri/icons/128x128@2x.png" />
<h1><b>Vleer</b></h1>

Explore high-quality music with Vleer a fast, simple, and reliable app, tailored to your preferences.
<br>
<a href="https://vleer.app"><strong>vleer.app »</strong></a>

<table>
  <tbody>
    <tr>
      <td>Download for</td>
      <td>
        <a href="https://github.com/vleerapp/Vleer/releases/download/v0.1.1/Vleer-0.1.1.msi">
          <img src="./public/windows.png"> Windows
        </a>
      </td>
      <td>
        <a href="https://github.com/Vleerapp/Vleer/releases/download/v0.1.1/Vleer-0.1.1_silicon.dmg">
          <picture>
            <img src="./public/apple.png">
          </picture> macOS (Silicon)
        </a>
      </td>
      <td>
        <a href="https://github.com/Vleerapp/Vleer/releases/download/v0.1.1/Vleer-0.1.1_intel.dmg">
          <picture>
            <img src="./public/apple.png">
          </picture> macOS (Intel)
        </a>
      </td>
      <td>
        <a href="https://github.com/Vleerapp/Vleer/releases/download/v0.1.1/Vleer-0.1.1.AppImage">
          <img src="./public/linux.png"> Linux
        </a>
      </td>
    </tr>
  </tbody>
</table>

<sup>Unstable Nightly releases can be found <a href="https://github.com/vleerapp/Vleer/actions/workflows/build.yml">here</a> </sup>
</div>

<br>

![Vleer banner](https://github.com/vleerapp/Vleer/assets/70103896/f4a619ab-4f4c-4c2f-babe-79a4555a93c5)

> \[!IMPORTANT]
>
> **Star Us**, You will receive all release notifications from GitHub without any delay \~ ⭐️

<details>
  <summary><kbd>Star History</kbd></summary>
  <a href="https://star-history.com/#vleerapp/vleer&Date">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=vleerapp/vleer&theme=dark&type=Date">
      <img width="100%" src="https://api.star-history.com/svg?repos=vleerapp/vleer&type=Date">
    </picture>
  </a>
</details>

## 📀 What is Vleer?

Vleer is a lightweight, fast, privacy-first music app, offering a free alternative to Spotify or Apple Music. Ideal for those who can't afford or don't want to pay for these shitty subscriptions.

🚧 Despite a major rewrite since January 2024, Vleer is already showing great promise. We are actively seeking contributors to help improve the project. If you're interested, we welcome your participation.

> \[!WARNING]
>
> **Legal disclaimer**
> We do not assume responsibility for copyright infringements or misuse of downloaded content. Users must ensure they have the rights to download and use any material. Use at your own risk.

## 📦 Features

- [X] Playlists
- [X] Offline music
- [X] Equalizer
- [X] Custom Backend
- [ ] Music streaming

![Vleer preview gif](https://github.com/vleerapp/Vleer/assets/70103896/aa9a0be4-0f3f-4cef-b2c4-b9b21602885b)

<sup>If you have ideas for features to include, please write a feature request [here](https://github.com/vleerapp/vleer/issues).</sup>

## ❤️ Donations & Support

Vleer is open-source and free to use. We appreciate donations to support ongoing development and improvements. Your contributions are voluntary and help us enhance the app for everyone.

<a href="https://buymeacoffee.com/pandadev_"><img src="https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black"/></a>

More options available via the Sponsor ❤️ button above.

## ⌨️ Local development

You can use GitHub Codespaces for online development:

[![][codespaces-shield]][codespaces-link]

Or to get Vleer set up on your machine, you'll need to have Rust and pnpm installed. Then, follow these steps:

```zsh
git clone https://github.com/vleerapp/Vleer.git
cd Vleer
pnpm i
pnpm dev
```

> \[!TIP]
>
> If you are interested in contributing code, feel free to check out our GitHub [Issues](https://github.com/vleerapp/Vleer/issues).

## 🔨 Building for production

To build for production simply execute:

```zsh
pnpm build
```

> \[!NOTE]
>
> Don't worry, it will fail at the end because it can not detect a Private key, but the installer files will be generated regardless of that.
>
> You can find them in `src-tauri/target/release/bundle`.

## 📝 License

Vleer is licensed under the GPL-3. See the [LICENSE file](./LICENCE) for more information.

[codespaces-link]: https://codespaces.new/vleerapp/vleer
[codespaces-shield]: https://github.com/codespaces/badge.svg
