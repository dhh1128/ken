# Tables

Ken supports the [table syntax of Github-flavored markdown](https://github.github.com/gfm/#tables-extension-). Ken also adds many extra features to control appearance.

In general, cells can be styled by appending one or more **style instructions** to the pipe character that precedes a cell. Entire columns can be styled by appending these same instructions to the associated pipe character on the delimiter row. Style instructions are inherited across a row, but not from one row to another. This means you can style a whole row just by styling its first cell. If you want different styles in subsequent cells on the same row, just specify the styles.

## Shading

Shading instructions are single characters. Three are possible:

instruction | meaning
--- | ---
`x` | darken
`o` | reset
`X` | double darken

Reset just cancels an inherited darkening; normally it is not needed. To shade alternate rows, use both the `x` and `o` characters as a single instruction. In this case, if `x` appears first, the odd rows below the header will be darkened; if `o` appears first, even rows will be darkened.

## Highlighting

Just like [standalone highlights](highlights.md) on text, ken supports highlighting full cells in a table. Append a highlight instruction like `~red~` to the 

Entire columns can be shaded by appending shade characters to the pipe(s) on the delimiter row.

## vertical alignment

Vertical alignment for an individual cell can be set by appending a character after the pipe that precedes it. If such a character is appended to the pipe characters on the delimiter row, it sets the default for the whole column (including the header rows above it). The alignment characters are `^` (`vertical-align: top`), `v` (`vertical-align: bottom)`, and `<` (`vertical-align: middle`):

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


```iml
Month | Savings | Savings for holiday!
--- | --- | ---
January | $100 |_ $50
February | $80
```

To begin a cell's content with a `>` character, simply put a space between the pipe delimiter and the content.

## multiline cells

Table rows in markdown must fit onto a single line. This means that if you want complex substructure inside a cell (e.g., multiple paragraphs, lists), [you must use HTML constructs](https://stackoverflow.com/a/48754707).   