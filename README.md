More stupid fun with ChatGPT. Rewrites web text based on your prompt.

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
![imperial_example](/imperial_example.png)

# Bugs

the rust `webdriver` crate likes to panic when any kind of error happens. /shrug

the XPaths are very XPath and WebDriver isn't actually good at finding text nodes (as opposed to elements)

this should really be a browser extension instead of using webdriver at all, but it was ez