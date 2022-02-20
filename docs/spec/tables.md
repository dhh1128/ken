# Tables

Ken supports the [table syntax of Github-flavored markdown](https://github.github.com/gfm/#tables-extension-). Ken also adds many extra features to control appearance.

In general, cells can be styled by appending one or more **style instructions** to the pipe character that precedes a cell. Multiple style instructions can often be specified by pure concatenation; if something is ambiguous, separate with semicolon. Entire columns can be styled by appending these style instructions to the associated pipe character on the delimiter row. Style instructions are inherited across a row, but not from one row to another. This means you can style a whole row just by styling its first cell. If you want different styles in subsequent cells on the same row, just specify the styles each time they change.

## Shading

Shading instructions are single characters. Three are possible:

instruction | meaning
--- | ---
`x` | darken
`o` | reset
`*` | double darken

Reset cancels an inherited darkening on the current row. It isn't needed when doing full-row styling, but it allows for alternating cell shading on a single row.

To shade alternate rows, use both the `x` and `o` characters as a single instruction on the delimiter row of the table. In this case, if `x` appears first, the odd rows below the header will be darkened; if `o` appears first, even rows will be darkened.

## Highlighting

Ken supports highlighting cells in a table. The format and appearance is the same as what's used for [standalone highlights](highlights.md) on text. For example, highlight a cell red with `~red`.

## Border style

`bs:s` (only have to use enough letters from CSS to disambiguate, so `s` means solid). Use commas for 2, 3, or 4-value versions (e.g., `bs:s,s`)

## Border color

`bc:blue`. Use commas for 2, 3, or 4-value versions (e.g., `bs:s,s`). If colors are specified without styles, assume `bs:s` in whatever variant (1, 2, 3, or 4) the color uses.

## Column width

A column width instruction is an expression like `w:20%` or `w:3em`. Append it to the pipe that precedes a cell on the delimiter row; it is not valid on content rows or header rows.

## Tables without headers

In ken, a table can begin with the delimiter row; the header row is optional. 

## Vertical alignment

Vertical alignment instructions are `^` (`vertical-align: top`), `v` (`vertical-align: bottom)`, and `<` (`vertical-align: middle`):

```ken
| Month    | Savings|
|^ :-----  |v ----: |
| January  | $100   |
| February | $80    |
|< March   | $75    |
```

```html
<table>
    <tr>
        <th>Month</th><th>Savings</th>
    </tr>
    <tr>
        <td style="vertical-align:top;align:left">January</td><td style="vertical-align:bottom;align:right">$100</td>
    </tr>
    <tr>
        <td style="vertical-align:top;align:left">February</td><td style="vertical-align:bottom;align:right">$80</td>
    </tr>
    <tr>
        <td style="vertical-align:middle;align:left">February</td><td style="vertical-align:bottom;align:right">$80</td>
    </tr>
</table>
```

## colspan
A cell (including in the header row) can be given an HTML `colspan` value (merging it with the cell(s) to its right) by preceding its delimiting pipe character with one or more greater-than (`>`) characters (where the number of `>` characters tells how many rightward cells to merge):

```ken
Month |	Savings
--- | ---
January	| $100
February | $80
|> Sum : $180
```

To begin a cell's content with a `>` character, simply put a space between the pipe delimiter and the content.

## rowspan
A cell can be given an HTML `rowspan` value (merging it with the cell(s) below it) by preceding its delimiting pipe character with one or more underscore (`_`) characters (where the number of `_` characters tells how many downward cells to merge):


```ken
Month | Savings | Savings for holiday!
--- | --- | ---
January | $100 |_ $50
February | $80
```

To begin a cell's content with a `>` character, simply put a space between the pipe delimiter and the content.

## block (multiline) cells

Unlike markdown, where [table rows must fit on a single line](https://stackoverflow.com/a/48754707), ken allows you to have arbitrary content inside a table. Any cell that has such content must be preceded by a `|+` that ends the line, then indented block content, then `+|` (end of cell) or `+|+` (end and begin another multiline cell). Although pipe characters at the end of a row are optional in GFM's table extension, you MUST end a multiline cell with a `+|` if it is the last cell of the row.

```ken
property address | description | notes
--- | --- | ---
123 Main Street |+
    A gorgeous antebellum mansion on the
    edge of New Orleans' lake district.
    
    Stately trees festooned with Spanish
    Moss.
    +| Call after 5 pm.
456 Willow Ave |+
    Direct access to the marina. Private
    garage and sun deck.
    
    HOA fees extra.
    +|+
    Some unusual things about this property:
    * Originally the home of a French pirate
      that operated out of the Gulf of Mexico.
    * Once cursed by a witchdoctor.
    +|
```