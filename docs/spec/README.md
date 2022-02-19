# Spec

Ken's syntax derives from [Github-Flavored Markdown](https://github.github.com/gfm/) (which in derives from [CommonMark](https://spec.commonmark.org/)). If you know markdown, you can probably take most of ken's syntax for granted. We do not repeat the content of those specs here; we just describe the differences.

## Not supported
Ken removes some optional, uncommon markdown constructs. No expressive power is lost. The simplifications just improve consistency and readability, and make parsing easier.

removed construct | comment
--- | ---
[Setext headings](https://github.github.com/gfm/#setext-headings) | Use [ATX headings](https://github.github.com/gfm/#atx-heading) instead. Eliminates examples <a href="https://github.github.com/gfm/#example-50">50</a>-[76](https://github.github.com/gfm/#example-76) from the GFM spec.
[Trailing `#` on ATX headings](https://github.github.com/gfm/#example-41) | Unnecessary. Eliminates examples <a href="https://github.github.com/gfm/#example-41">41</a>-[46](https://github.github.com/gfm/#example-46) from the GFM spec.
[leading tabs](https://github.github.com/gfm/#tabs) | Indents created with mixed tabs and spaces creates surprises. Indent with 4 spaces. Leading tabs are an error.
extra leading spaces | Many markdown implementations ignore sequences of 1 to 3 extra leading spaces at the beginning of a line, so a 2-space indent has the same meaning as no indent, and a 7-space indent has the same meaning as a single 4-space indent. You can see a requirement to support this behavior in [GFM's 4.1 Thematic Breaks](https://github.github.com/gfm/#tabs), for example. Ken disallows this behavior. Indent as much as you need, but don't put extra leading whitespace on a line. 

## Repurposed or extended

Regular markdown often provides more than one way to do the same thing. Ken provides only one way. Alternatives are repurposed to increase expressiveness. Some new syntax is added.

### Bullets

Regular markdown treats [all the bullet characters](https://github.github.com/gfm/#list-items) as synonyms, and always renders them with a filled-in disc • (CSS `list-style-type: disc`). In ken, only the `*` is for bullets, and it can be styled. To style, add `:` plus [a value of the CSS `list-style-type`](https://www.w3schools.com/CSSref/pr_list-style-type.asp). The style applies to all subsequent bullets in the same list. So, for bullets that are open circles ○, the first item in a list should be `*:circle First item`; for squares ■, the first item in a list should be `*:square First item`; and so forth.

You can also use a single arbitrary unicode character as a bullet by substituting it for the token after `:`. For example, to have list items that are checkmarks, use `*:✓`. (Ken is always UTF-8. What you put here is one logical unicode codepoint, even if it takes more than one byte to represent.)

### Collapsible Outlines

Regular markdown treats `+` as a bullet prefix. In ken, this followed by an indented block beneath it maps to a collapsible HTML 5 `<details><summary>...</summary><section>...</section></details>` structure. The effect in most browsers is similar to a treeview or collapsible outline.

```ken
+ My Section Title

    Section content
```
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;▼
```html
<details
    <summary>My Section Title</summary>
    <section>
        Section content
    </section>
</details>
```

TODO: how to say whether the section is open or closed by default.
TODO: should summary be styled like a header? Maybe only if followed by a header?

#### Transformation modes

For safety and simplicity reasons, intent docs are assumed to intend [embeddable mode](../glossary.html#embeddable-mode) only. The transformation of individual files can always be forced into [standalone mode](../glossary.html#standalone-mode) with the `--standalone-mode` switch. Alternatively, the `--natural-mode` switch will render all files in their [natural mode](../glossary.html#natural-mode).

A file can declare or inherit a `standalone prefix` and/or `standalone suffix` property; the direct declaration of such a prefix or suffix changes the natural mode for the content to standalone. During standalone mode transformation, the content of `standalone prefix` is prepended to the main output as if it were a pure HTML fragment that contained all structure up to and including `<body>`; the content of `standalone suffix` is appended as if it were a pure HTML fragment that contained at least `</body></html>`. 

#### Standalone features

The sanitization of embeddable mode allows the same tags, HTML attributes, and CSS attributes as [HTMLSanitizer's default posture](https://github.com/mganss/HtmlSanitizer#tags-allowed-by-default). Note that this is substantially richer than the set allowed in Github Markdown.

Any HTML constructs beyond this require standalone mode. 

#### Hyperlinks and Anchors

See [hypertext](hypertext.md).

#### Includes

IML supports includes as a convenience.

#### Mustaches

[Mustache templates](https://mustache.github.io/) are supported. A number of predefined metadata variables are defined:

* `{{doc.*}}` &mdash; Document attributes: `size`, `lastmod`, `author`, `commit-hash`, `version`, `relative-path`...
* `{{time.*}}` &mdash; Timestamps in various formats: `yyyy-mm-ddThh:nn:ss.xZ` and similar.
* `{{space.*}}` &mdash; Information about the codebase that contains the doc: `doc-count`, `upstream`, `origin`.

