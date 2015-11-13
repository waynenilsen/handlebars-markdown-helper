![master status](https://travis-ci.org/waynenilsen/handlebars-markdown-helper.svg?branch=master)

# Handlebars Markdown Helper

The [handlebars crate](https://crates.io/crates/handlebars) provides easy templating for any file type. This crate provides a helper for rendering a variable containing markdown into HTML within a temlpate. 

## Example 

```
let t0 = Template::compile("{{markdown x}}".to_string()).ok().unwrap();

let mut handlebars = Handlebars::new();
handlebars.register_helper("markdown", Box::new(::markdown_helper));
handlebars.register_template("t0", t0);

let mut m :BTreeMap<String, String> = BTreeMap::new();
m.insert("x".into(), "# wow\n\n## second wow".into());

let r0 = handlebars.render("t0", &m);
assert_eq!(r0.ok().unwrap(), "<h1>wow</h1>\n<h2>second wow</h2>\n".to_string());
```

---

![deps](Cargo.jpg)
