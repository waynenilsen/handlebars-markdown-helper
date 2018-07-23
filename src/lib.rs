extern crate handlebars;
use handlebars::{Handlebars, JsonRender, RenderContext, RenderError, Helper, Context, Output};

extern crate pulldown_cmark;
use self::pulldown_cmark::Parser;
use self::pulldown_cmark::html;

pub fn render_html(text: String) -> String {
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = Parser::new(&*text);
    html::push_html(&mut s, p);
    s
}

pub fn markdown_helper(h: &Helper,
                       _: &Handlebars,
                       _: &Context,
                       _: &mut RenderContext,
                       out: &mut Output)
                       -> Result<(), RenderError> {
    let markdown_text_var =
        try!(h.param(0)
                 .ok_or_else(|| RenderError::new("Param not found for helper \"markdown\"")));
    let markdown_text = markdown_text_var.value().render();
    let html_string = render_html(markdown_text);
    try!(out.write(&html_string));
    Ok(())
}

#[cfg(test)]
mod test {
    use handlebars::Handlebars;
    use std::collections::BTreeMap;

    #[test]
    fn test_markdown() {
        let t0 = "{{markdown x}}";

        let mut handlebars = Handlebars::new();
        handlebars.register_helper("markdown", Box::new(::markdown_helper));
        assert!(handlebars.register_template_string("t0", t0).is_ok());

        let mut m: BTreeMap<String, String> = BTreeMap::new();
        m.insert("x".into(), "# wow\n\n## second wow".into());

        let r0 = handlebars.render("t0", &m);
        assert_eq!(r0.ok().unwrap(),
                   "<h1>wow</h1>\n<h2>second wow</h2>\n".to_string());
    }
}
