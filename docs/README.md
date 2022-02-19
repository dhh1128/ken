# ken

Ken is a variant of markdown that increases expressive power.

![definition](assets/definition.png)

Ken is a variation of [Github-Flavored Markdown](https://github.github.com/gfm/) (which is in turn a variation of [CommonMark](https://spec.commonmark.org/)). Rather than reproducing a slight variation on the GFM spec here, ken's documentation of IML emphasizes just the differences.

#### Different assumptions

Markdown (a *writing* format) prioritizes the goal of writing content that is readable as plaintext, and transforming it into HTML (a *publishing* format) that can display safely inside a containing website. This means it [favors simple syntax and advocates using inline HTML for advanced constructs](https://daringfireball.net/projects/markdown/syntax#html). But typically, implementations also sanitize various HTML constructs (CSS, `<style>`, `<script>`, many HTML tag attributes, etc.) to prevent [cross-site scripting (XSS)](https://owasp.org/www-community/attacks/xss/#) and [scriptless attacks](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.469.7647&rep=rep1&type=pdf); this means markdown is linmited as a basis for fancy documents.

Ken is still a *writing* format, like markdown; it also renders to HTML for publication. However, it prioritizes expressive power and standalone documents more highly. Although most ken is as simple to learn and read as markdown, it has more options for advanced users. And instead of mandating the sanitization of dangerous HTML elements, it defines choices for [transforming](../glossary.html#iml-transformation) in [embeddable mode](../glossary.html#embeddable-mode), [standalone mode](../glossary.html#standalone-mode), or [natural mode](../glossary.html#natural-mode). This preserves safety where it is needed, but allows more nuanced output.

Ken also introduces some semantics that extend beyond the limits of ordinary HTML. Read [the spec](spec/README.md).
