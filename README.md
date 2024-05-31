# Vleer 

![Vleer banner](https://github.com/vleerapp/Vleer/assets/70103896/f4a619ab-4f4c-4c2f-babe-79a4555a93c5)

Vleer is a lightweight and fast privacy first music app replacing your traditional Spotify or Apple Music. It serves the purpose for people that don't have enough money or don't want to pay monthly subscriptions, this is why Vleer is completely free and will stay that way.

🚧 Despite undergoing a major rewrite since January 2024, Vleer is already showing great promises in terms of functionality. We are actively seeking contributors to help us continue improving the project. If you are interested in contributing, we welcome your participation.

#### [⬇️ Download Vleer](https://github.com/vleerapp/Vleer/releases)

## 📦 Preview

![Vleer preview gif](https://github.com/vleerapp/Vleer/assets/70103896/aa9a0be4-0f3f-4cef-b2c4-b9b21602885b)

## ❤️ Donations & Support

Vleer is an open-source project, and we rely on the support of our community to continue developing and improving the app. Although Vleer is free to use, we welcome donations from those who have found it to be a valuable app and would like to contribute to its development.

Please note that Vleer is and will always be free to use. Your donation is entirely voluntary and is not required to use the app.

<a href="https://ko-fi.com/pandadev_"><img src="https://img.shields.io/badge/Buy_Me_A_Coffee-323842?style=for-the-badge&logo=buy-me-a-coffee&logoColor=white"/></a>

Find more options by clicking the Sponsor ❤️ button on the top of this page.

## 🤝 Contributing

To get Vleer set up on your machine, you'll need to have Rust and pnpm installed. Then, follow these steps:

1. Clone the project using `git clone https://github.com/vleerapp/Vleer.git`
2. Change into the project directory: `cd Vleer`
3. Install dependencies: `pnpm i`
4. Run the development server: `pnpm dev`

## 🛠️ Building for Production

```zsh
pnpm build
```

Don't worry, it will fail at the end because it can not detect a Private key, but the installer files will be generated regardless of that.

You can find them in `src-tauri/target/release/bundle`.

Checkout the [Issues section](https://github.com/vleerapp/Vleer/issues).

## 📝 License

Vleer is licensed under the Creative Commons Attribution-Noncommercial-Share Alike. See the [LICENSE file](./LICENCE) for more information.
