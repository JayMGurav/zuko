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



### Questions 

- can we use a CSV file for problem set data.
    - https://stackoverflow.com/questions/76953964/run-sql-query-on-csv-file-contents-from-command-line

<!-- That's rough buddy. for failed tests -->
<!-- 


todo

1. remove default config toml, keep it in-memory code level defaults
2. on start of the program create a .zuko/config.toml and zuko_db at .zuko/db/local.db
2. zuko init??  -->

<!-- 
fn main() {
    let questions = vec![
        Question {
            title: "Two Sum".into(),
            title_slug: "two-sum".into(),
            content: "...".into(),
        },
        Question {
            title: "Add Two Numbers".into(),
            title_slug: "add-two-numbers".into(),
            content: "...".into(),
        },
    ];

    let results = search_questions(&questions, "add");

    for q in results {
        println!("Matched: {}", q.title);
    }
} -->