# SDAC

The **S**oftware **D**efined **A**utomatic **C**alculator, is a research project that aims to implement a programmable calculator with its own [LISP](https://en.wikipedia.org/wiki/Lisp_(programming_language)) dialect.

### Types

**Expressions** (exp): Things to be evaluated.

**Values** (val): Things that are evaluated.

### Keywords

| Keyword | Symbol |   Description  |
|:-------:|:------:|:--------------:|
|   add   |    +   |    Addition    |
|   mul   |    *   | Multiplication |
|   sub   |    -   |   Subtraction  |
|   div   |    /   |    Division    |
|   mod   |    %   |     Modulus    |
|   and   |    &   |       AND      |
|   ior   |   \|   |  Inclusive OR  |
|   xor   |    ^   |  Exclusive OR  |
|   eqa   |    =   |    Equality    |
|   neq   |    !   |  Non Equality  |
|   min   |    <   |     Minimum    |
|   max   |    >   |     Maximum    |

### Example (The answer to everything)

**Input**
```lisp
(exp mul (val . 21) (exp add (val . 3) (val . -1)))
```
**Output**
```
+ 3 -1
* 21 2
: 42
```
For which ':' is the complete evaluation of the program.
Meaning that the answer to life, the universe, and everything is 42.
