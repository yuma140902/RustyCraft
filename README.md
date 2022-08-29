# RustyCraft

マインクラフトっぽいゲームです。今はまだ動き回ることしかできないのでゲームとは言えないかもしれません。W、A、S、Dキーで移動、スペースでジャンプ、マウスで視点の回転ができます。ゲームエンジンを使わず、OpenGLを直接使ってゲームを作るという目標で制作しています。

PistonやAmethystなどのゲームエンジンを使っていません。ただし、ECSライブラリとしてSpecsを使っています。依存クレートの中ではこれが一番ゲームエンジンっぽいと思われます。いずれECSライブラリも自前で実装したいと思っています。

描画処理はすべてOpenGLです。

![screenshot](https://user-images.githubusercontent.com/23431077/134598490-542474aa-095e-4939-a7c1-49a5a95d300a.png)

![demo](https://user-images.githubusercontent.com/23431077/134598462-0c093e6a-dc22-4eb2-af0d-f1e54facfb0a.gif)


# ダウンロード

## Windows

- [RustyCraft-yagami-rc-1.1.zip](https://1drv.ms/u/s!AqAj8LwUfPOGgd8nJc6vDXEfzmo_Pw?e=J8UWuM)
  - 1.27 MB / 2021-09-24


# ビルド方法

```
> cargo build
```

リリースビルドをする場合は `cargo build` の代わりに `cargo build --release` を実行する。

# 実行
```
> cargo run
```

または

```
> cargo run --release
```
