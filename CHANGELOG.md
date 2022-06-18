## 0.5.1 (June 19, 2022)

### Release 0.5.1

- clap = { version = "3.2.5", features = ["derive"] }
- fixing clap bug

---

## 0.5.0 (December 3, 2021)

### Release 0.5.0

- To prevent a conflict
- between the macro name and the structure name,
- the macro was renamed `ARGONE`.

---

## 0.4.0 (December 3, 2021)

### Release 0.4.0

- removing unnecessary #[clap()] tokens.
- now [Config] can parse Vector types.
- fixing small bugs.

[Example]

- (short, long)
- [Config] someList: Vec\<String\> = vec!["default".to_string()]

---

## 0.3.0 (December 2, 2021)

### Release 0.3.0

- adding prelude module.
- please use<br>
  `pub use argone::{*, prelude::*};`<br>or<br>`pub use argone::{..., prelude::*};`

---

## 0.2.0 (December 2, 2021)

### Release 0.2.0
