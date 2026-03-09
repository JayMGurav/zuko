# Hello. Zuko here

> “It’s time for you to look inward and begin asking yourself the big questions.” – Zuko

**Zuko** is a blazing-fast, terminal-first CLI tool to supercharge your Data Structures and Algorithms (DSA) practice.

## 🎬 Demo

Click on Zuko to play.

[![Watch the Demo](https://jaygurav.com/zuko-style.png)](https://jaygurav.com/zuko-demo.mp4)

### 🎯 Our Goal

Make the **developer terminal** the first and only stop for solving, exploring, and practicing questions — even offline — and use **version control** to track your progress over time.

Inspired by the discipline and drive of Zuko from *Avatar: The Last Airbender*, this tool helps you stay consistent, focused, and efficient — bringing the full problem-solving experience right into your terminal.

---

## ✨ Features

- 📦 **Project Generator**: Instantly scaffold a new problem with boilerplate code in your preferred language (Rust, Python, Java, etc.)
- 🔍 **Fuzzy Search with Preview**: Browse and preview problems at lightning speed using a terminal UI powered by `ratatui` and `nucleo-matcher`
- 📖 **Markdown Rendering**: View full problem descriptions directly in your terminal with clean formatting via `termimad`
- 📊 **Topic & Difficulty Filters**: Practice problems by tag, difficulty level, or let fate decide with a random selection
- 🔁 **Streak Tracking**: Build daily habits with automatic streak tracking
- 🧠 **Offline-First Experience**: Train anytime, anywhere with synced local problem sets using SQLite or Turso DB
- 🛠️ **Version Control Integration**: Automatically manage git repositories per problem — commit as you solve, and track your streak through version control

---

## 🚀 Quickstart

```sh
cargo install zuko
zuko init                   # Setup config (language, paths, etc.)
zuko list                   # Browse problems with fuzzy search UI
zuko pick --topic=dp        # Pick a random DP problem
zuko solve 1234             # Setup problem #1234 in template directory
zuko sync                   # Refresh local problem sets
```

## 🧠 Philosophy

Zuko embodies discipline, resilience, and growth. It's designed to help you build real momentum in your coding journey — by eliminating friction, promoting consistency, and making the terminal your training ground.

> “That’s who you are, Zuko. Someone who keeps fighting even when it’s hard.”


## 🛤️ Roadmap
- 🔔 Daily reminders
- 💡 Personalized practice plans
- config file for user perference
- 🌐 Leetcode/Codeforces integration
- 📅 Calendar view of streaks
- ☁️ VCS tracking for maintaing streak
- community support for sourcing problems



### Questions 

- can we use a CSV file for problem set data.
    - https://stackoverflow.com/questions/76953964/run-sql-query-on-csv-file-contents-from-command-line

<!-- That's rough buddy. for failed tests -->
<!-- 
