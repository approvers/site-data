# Site Data

> [限界開発鯖公式サイト](https://approvers.dev)の各種データを管理するリポジトリです。

---

# ブログの追加方法

1. `data/blogs/` 下に `.md` ファイルを追加します (連番にする必要は別にありません).
2. Markdown ファイルの先頭に以下のような Frontmatter を書きます.

```md
---
title: (タイトル)
date: (作成日)
author: (著者名)
---
```

3. その下にブログ記事を Markdown で記述します.
4. ブランチを作成してコミットし, プルリクエストを作成します.
5. ブログの内容や記述方法についてのレビューに対応します.
6. *Approve* されたら, マージしましょう. そのうち [限界ブログ](https://approvers.dev/blog) 上に反映されます.

省略しますが, 編集や削除も同様に行ってください.

# リンク集の編集方法

1. `data/links.yaml` を開きます.
2. リンクは以下の形式になっているので, これを追記, 編集, 削除します.

```yaml
- name: (リンク名)
  url: (リンク URL)
```

3. ブランチを作成してコミットし, プルリクエストを作成します.
4. 記述方法についてのレビューに対応します.
5. *Approve* されたら, マージしましょう. そのうち [限界リンク集](https://approvers.dev/link) に反映されます.

# メンバーリストの編集方法


1. `data/members/list.yaml` を開きます.
2. メンバーの情報は以下の形式になっているので, これを追記, 編集, 削除します.

```yaml
- name: (名前)
  avatar: (アバターの画像 URL, なければ空文字列 "" に)
  role: (代表的なロール名, ガチプロなど)
  links:
    - type: (twitter または github, なければ空のリスト [] に)
      url: (その URL)
```

3. ブランチを作成してコミットし, プルリクエストを作成します.
4. 記述方法についてのレビューに対応します.
5. *Approve* されたら, マージしましょう. そのうち [メンバー紹介](https://approvers.dev/member) に反映されます.
