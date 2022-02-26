# ken

Ken is a variant of markdown that maximizes expressive power. If you know markdown, you already "ken" a lot of ken's syntax. Read [the spec](spec) for details.


![definition](assets/definition.png)

#### Why

Regular markdown (a high-level *writing* format) aims at authoring content that is readable as plaintext, and transforming it into HTML (a lower-level, more powerful *publishing* format) that can display safely inside a containing website. It's great if simplicity is your overriding concern.

But heavy users of markdown hit limits. *How do I make callouts or endnotes? Can I put multi-line content in a table? Why can't I use colors? Is there any way to tweak the CSS?* Markdown's answer is to [use inline HTML for advanced constructs](https://daringfireball.net/projects/markdown/syntax#html). This is brilliant, for markdown's original use cases. However, it's not great for more ambitious projects. Because most markdown processors assume you'll render inside a web site container, they must sanitize many HTML constructs to prevent [cross-site scripting (XSS)](https://owasp.org/www-community/attacks/xss/#) and [scriptless attacks](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.469.7647&rep=rep1&type=pdf). These are the very features that make HTML so beautiful, expressive, and responsive (CSS, `<style>`, `<script>`, attributes on tags, etc.).

Coding HTML+CSS+JavaScript directly is an option, of course. If you're willing to roll up your sleeves, there's little you can't model with that combination. But this isn't just a bother, a time sink, or a learning curve &mdash; it's a strategy incompatible with static website generators and embedded wiki pages, so it changes your toolset and environment. And it may redirect the emphasis of writing from content to appearance. It's a bit like asking programmers to drop into low-level assembly language just because they want to split code into subroutines.

Ken is still a high-level *writing* format, with most of markdown's advantages. It generates HTML, so it can be used with static site generators. But it offers many extra options for advanced users. And instead of mandating the sanitization of dangerous HTML elements, it defines choices for [transforming](glossary#transformation) in [embeddable mode](glossary#embeddable-mode), [standalone mode](glossary#standalone-mode), or [natural mode](glossary#natural-mode). This preserves safety where it is needed, but enables more nuanced output.

Ken also introduces a few hypertext features that exceed the abilities of vanilla HTML. These constructs can be built into web pages with a combination of CSS and JavaScript. But in ken, you just write them directly without worrying about implementation details.
