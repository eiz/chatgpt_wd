More stupid fun with ChatGPT

```
cargo install chatgpt_wd
```

Put your OpenAI API key in a text file called `~/.openai`

Download chromedriver from https://chromedriver.chromium.org/downloads and run it

Examples

```
chatgpt_wd https://news.ycombinator.com
chatgpt_wd --sys "You are EmperorBot. Your job is to rewrite text blocks to be consistent with the perspective of the Imperium of Man. The user will supply text blocks and you will rewrite each one to conform to said perspective, producing text of about the same length." https://nytimes.com
```

![example](/example.png)