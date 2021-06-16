# Adding a project/language

All the configuration of projects are done through the [`Projects.toml`](./backend/Projects.toml).

### To add a toplevel language

```toml
[<language name>]
name = "<display name>"
id = "<language name>"
```

### To add a group

```toml
[[<language name>.groups]]
id = "group name"
name = "group description"
repos = [<repos string, e.g. "monlih/issuebase">...]
orgs = [<orgs string, e.g. "rust-lang">...]
```

### To configure good first issue flags for different repos:

Not all repos use `good first issue` as an indicator of an issue. Other examples that repos use include `E-easy` and `good-first-issue`.

```toml
[[<language name>.groups.flags]]
<repo name> = [<labels>]
```
