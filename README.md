```
┏┓   •    
┣┫┏┳┓┓┏┓┏┓
┛┗┛┗┗┗┛┗┗┛
static site generator
```

🦀 Sweet simple static site generator

I decided to port over the logic of my ReasonML SSG to Rust! 

Production

```
amino markdown out
```

Development

```
git clone git@github.com:jottenlips/amino-ssg.git
cargo build
cargo run -- markdown out
```

Here is an example of using custom `base.html`.

[Blog](https://jottenlips.github.io/)

[Repo](https://github.com/jottenlips/jottenlips.github.io)

## 📂 File structure

```
./markdown
└──base.html
└──index.md
└──another-page.md
└──topic.md (index for your folder)
└──/topic
    └──2019-1-1.md
    └──2020-1-1.md
    └──2021-1-1.md
```

results in

```
./public
└──index.html
└──another-page/index.html
└──/topic
    └──index.html (topic.md)
    └──2019-1-1/index.html
    └──2020-1-1/index.html
    └──2021-1-1/index.html
```
