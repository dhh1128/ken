# CSS

Ken supports CSS far more than ordinary markdown.

## Classes
Ken uses CSS classes to define the appearance of some document structures.

css class name | meaning | default appearance
--- | --- | ---
`darker` | In a table with alternating stripes, make this row or cell one of the darker ones. | Background color #c0c0c020.
`lighter` | In a table with alternating stripes, make this row or cell one of the lighter ones. | Background color #ffffff20.

## Declaring and referencing a CSS

Ken allows CSS to be referenced in the YAML front matter that precedes a document:

```yaml
css:
  - abc.css
  - def.css
```

## Direct styling

In addition to styling at the document level, ken lets an author declare very granular styles by appending a **stylizer** to other structural elements. Example, to style a cell of a table, append `~hi:green` immediately after the pipe character that delimits it.

The syntax of a stylizer is:

```abnf
stylizer "~" +(prop_name ":" prop_val)
```

Valid values of `prop_name` are: any officially defined CSS value (e.g., `background-color`, `text-align`), or an abbreviation thereof. Abbreviations can be any shorter version of the word that is still unique, or, for multiword tokens, the first character of each initial token plus one or more characters from the last token to make the symbol unique. Thus, `background-color` could be abbreviated `bcolor`, `bcol`, or `bc`. 

