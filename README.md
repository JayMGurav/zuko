# Hello. Zuko here

> It’s time for you to look inward and begin asking yourself the big questions. -Zuko

**Zuko** is a blazing-fast, is a terminal-first CLI tool to supercharge your daily Data Structures and Algorithms (DSA) practice. It helps you stay consistent with your Data Structures & Algorithms (DSA) practice. With powerful UI, fast fuzzy search, and automatic boilerplate generation, Zuko brings discipline and speed to your daily routine.

Inspired by the discipline and drive of Zuko from Avatar: The Last Airbender, this tool pushes you to stay consistent, train smart, and master the elements of programming one problem at a time.


## ✨ Features

- 📦 **Project Generator**: Create directories with template files in your preferred language (Rust, Python, java, etc.)
- 🔍 **Fuzzy Search with Preview**: Search problems with blazing speed using a TUI (like `fzf`) powered by `ratatui` and `nucleo-matcher`
- 📖 **Markdown Rendering**: Preview the full problem description right in the terminal via `termimad`
- 📊 **Topics & Difficulty Filter**: Practice by tags, difficulty levels, or go fully random
- 🔁 **Streak Tracking**: Track your practice streaks and stay motivated
- 🧠 **Local Sync**: Offline support with a synced local cache (via SQLite/Turso DB)
- 🛠️ **VCS Integration**: Automatically set up git tracking and commit messages per problem, track your streak with your vcs.

## 🚀 Quickstart

```sh
cargo install zuko
zuko init                   # Setup config (language, paths, etc.)
zuko list                   # Browse problems with fuzzy search UI
zuko pick --topic=dp        # Pick a random DP problem
zuko solve 1234             # Setup problem #1234 in template directory
zuko sync                   # refresh problem sets 
```

## 🧠 Philosophy

Zuko embodies discipline, resilience, and growth. The tool nudges you to show up every day — to keep learning, keep solving, and keep progressing.

> “That’s who you are, Zuko. Someone who keeps fighting even when it’s hard.”


## 🛤️ Roadmap
- 🔔 Daily reminders
- 💡 Personalized practice plans
- config file for user perference
- 🌐 Leetcode/Codeforces integration
- 📅 Calendar view of streaks
- ☁️ VCS tracking for maintaing streak
- community support for sourcing problems

<!-- That's rough buddy. for failed tests -->