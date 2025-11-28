# Syntax

## 1. Numbers

```text
123
3.14
-42
+7
```

* Supports integers and floating-point numbers.
* Unary plus (`+`) and minus (`-`) are allowed.

---

## 2. Identifiers

```text
x
y
myVar
f
```

* Can be used for variables and function names.
* Must start with a letter or underscore, followed by letters, digits, or underscores.

---

## 3. Assignments

```text
x = 5
y = x + 3
f(x) = x * 2
```

* Use `=` to assign values to variables.
* Functions can be defined inline using the syntax: `f(param1, param2) = expression`.

---

## 4. Arithmetic Operators

| Operator | Description    | Example  |
| -------- | -------------- | -------- |
| `+`      | Addition       | `1 + 2`  |
| `-`      | Subtraction    | `5 - 3`  |
| `*`      | Multiplication | `2 * 4`  |
| `/`      | Division       | `10 / 2` |
| `%`      | Modulo         | `10 % 3` |
| `^`      | Exponentiation | `2 ^ 3`  |

* Multiplication can be **implicit**:

  ```text
  2x      // interpreted as 2 * x
  3(x+1)  // interpreted as 3 * (x + 1)
  ```

---

## 5. Comparison Operators

| Operator | Description      | Example  |
| -------- | ---------------- | -------- |
| `==`     | Equal            | `x == 5` |
| `!=`     | Not equal        | `x != 0` |
| `<`      | Less than        | `x < 10` |
| `>`      | Greater than     | `x > 10` |
| `<=`     | Less or equal    | `x <= 5` |
| `>=`     | Greater or equal | `x >= 5` |

---

## 6. Grouping

```text
(x + y) * z
```

* Parentheses `()` are used to group expressions and override operator precedence.

---

## 7. Function Calls

```text
f(2, 3)
sqrt(16)
```

* Functions are called with parentheses.
* No whitespace is allowed between the function name and `(` for proper parsing.

---

## 8. Implicit Multiplication

```text
2x      // equivalent to 2 * x
(x+1)(y-1) // equivalent to (x+1) * (y-1)
```

* Implicit multiplication works when a number or closing parenthesis is followed immediately by an identifier or another parenthesis.

---

## 9. Expressions

* Expressions can be separated by:

  * Semicolon `;`
  * Newline

```text
x = 5
y = x + 3; z = y^2
```

---

## 10. Comments

* Doesn't have any effects but they exist

  ```text
  // This is a comment
  ```

  ```text
  /*
    This is a multi-line comment
  */
  ```
