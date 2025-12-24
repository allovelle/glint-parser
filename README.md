# glint-parser

> A parser, for the McKeeman-form parser-generator.
> Interprets a given MKF grammar to parse the language the grammar describes.

McKeeman Form is a type of notation that is used to clearly express the
structure of grammars in a simplified and readable way. It is essentially a
streamlined version of the more traditional Backus-Naur Form (BNF), which is
often used to formally describe the syntax of programming languages and data
formats. The key characteristic of McKeeman Form is that it relies heavily on
whitespace to organize and present the rules of the grammar, reducing the need
for many metacharacters or symbols that can make other notations appear more
complex or cluttered. By minimizing the use of special symbols, McKeeman Form
enhances human readability and makes it easier to understand the relationships
between different parts of a grammar at a glance. This approach is particularly
helpful for documentation, teaching, and collaborative work, where clarity is
more important than compactness. Its simplicity allows both newcomers and
experienced developers to focus on the logical structure of the grammar rather
than deciphering intricate notation.

### Usage

```sh
# grammar.mkr describes source.lang
# glint runs the grammar.mkf like a script
$ glint grammar.mkf source.lang
```






SOURCE -> TOKEN_TABLE -> PREDICTIVE_PARSE_TABLE -> PARSER -> AST_TREE_AS_ROWS
