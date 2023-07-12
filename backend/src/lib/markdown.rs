use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;

enum CodeHighlightingState<'a> {
    NotInCodeBlock,
    RequiresFirstLineParse,
    UnknownSyntax,
    KnownSyntax(ClassedHTMLGenerator<'a>),
}
// https://github.com/chriskrycho/lightning-rs/blob/7c9854a701be41d38483b9cad4bda004b0324f10/src/page/markdown.rs
pub fn render_markdown_highlight_class<S: AsRef<str>>(src: S) -> anyhow::Result<String> {
    let src = src.as_ref();
    let parser = Parser::new_ext(src, Options::all());
    let mut state = CodeHighlightingState::NotInCodeBlock;

		let syntax_set = SyntaxSet::load_defaults_newlines();		
		// use syntect::html::css_for_theme_with_class_style;
		// let theme_set = ThemeSet::load_defaults();
		// let dark_theme = &theme_set.themes["base16-ocean.dark"];
		// let light_theme = &theme_set.themes["base16-ocean.light"];
		// let css_dark = css_for_theme_with_class_style(dark_theme, ClassStyle::Spaced)?;
		// let css_light = css_for_theme_with_class_style(light_theme, ClassStyle::Spaced)?;
		// println!("css dark: {}", css_dark);
		// println!("css light: {}", css_light);

    let mut events = Vec::<Event>::with_capacity(src.len() * 2);
    for event in parser {
        match event {
            Event::Text(text) => match &mut state {
                // This is a little quirky: it hands off the text to the highlighter
                // and relies on correctly calling `highlighter.finalize()` when we
                // reach the end of the code block.
                CodeHighlightingState::KnownSyntax(ref mut generator) => {
                    generator.parse_html_for_line_which_includes_newline(text.as_ref())?;
                    events.push(Event::Text("".into()));
                }
                // This has the same constraint as `KnownSyntax`, but requires that
                // we also try to get a
                CodeHighlightingState::RequiresFirstLineParse => match syntax_set.find_syntax_by_first_line(&text) {
                    Some(definition) => {
                        let mut generator = ClassedHTMLGenerator::new_with_class_style(definition, &syntax_set, ClassStyle::Spaced);
                        events.push(Event::Html(format!("<pre lang='{name}'><code class='{name}'>", name = definition.name).into()));
                        generator.parse_html_for_line_which_includes_newline(&text)?;
                        state = CodeHighlightingState::KnownSyntax(generator);
                        events.push(Event::Text("".into()));
                    }
                    None => {
                        events.push(Event::Html("<pre><code>".to_string().into()));
                        state = CodeHighlightingState::UnknownSyntax;
                        events.push(Event::Text(text));
                    }
                },
                CodeHighlightingState::UnknownSyntax | CodeHighlightingState::NotInCodeBlock => events.push(Event::Text(text)),
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(name))) => {
                if let Some(looked_up) = syntax_set.find_syntax_by_token(name.as_ref()) {
                    state = CodeHighlightingState::KnownSyntax(ClassedHTMLGenerator::new_with_class_style(looked_up, &syntax_set, ClassStyle::Spaced));
                    let html = format!("<pre><code class='{}'>", looked_up.name);
                    events.push(Event::Html(html.into()));
                } else {
                    state = CodeHighlightingState::UnknownSyntax;
                    events.push(Event::Html("<pre><code>".into()));
                }
            }
            Event::Start(Tag::CodeBlock(CodeBlockKind::Indented)) => match state {
                CodeHighlightingState::NotInCodeBlock => {
                    state = CodeHighlightingState::RequiresFirstLineParse;
                }
                _ => return Err(anyhow::anyhow!("should never be entering a codeblock when already in a codeblock")),
            },
            Event::End(Tag::CodeBlock(_)) => match state {
                CodeHighlightingState::KnownSyntax(generator) => {
                    let highlighted = generator.finalize();
                    state = CodeHighlightingState::NotInCodeBlock;
                    events.push(Event::Html((highlighted + "</code></pre>").into()));
                }
                CodeHighlightingState::UnknownSyntax | CodeHighlightingState::RequiresFirstLineParse => {
                    state = CodeHighlightingState::NotInCodeBlock;
                    events.push(Event::Html("</code></pre>".into()));
                }
                CodeHighlightingState::NotInCodeBlock => return Err(anyhow::anyhow!("Cannot *not* be in a code block when ending a code block")),
            },
            _ => events.push(event),
        }
    }

    let mut html_output = String::with_capacity(src.len() * 2);
    html::push_html(&mut html_output, events.into_iter());
    Ok(html_output)
}

// https://github.com/raphlinus/pulldown-cmark/issues/167
pub fn render_markdown_highlight_style<S: AsRef<str>>(src: S, is_dark: bool) -> anyhow::Result<String> {
    let src = src.as_ref();
    let parser = Parser::new_ext(src, Options::all());
    let mut events = Vec::<Event>::with_capacity(src.len() * 2);
		let mut in_code_block = false;
		let mut to_highlight = String::new();

		let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
    let theme = {
			if is_dark {
				&theme_set.themes["base16-ocean.dark"]
			} else {
				&theme_set.themes["base16-ocean.light"]
			}
		};

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                // In actual use you'd probably want to keep track of what language this code is
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                if in_code_block {
                    // Format the whole multi-line code block as HTML all at once
                    let html = highlighted_html_for_string(&to_highlight, &syntax_set, &syntax, &theme)?;
                    // And put it into the vector
                    events.push(Event::Html(html.into()));
                    to_highlight = String::new();
                    in_code_block = false;
                }
            }
            Event::Text(t) => {
                if in_code_block {
                    // If we're in a code block, build up the string of text
                    to_highlight.push_str(&t);
                } else {
									events.push(Event::Text(t))
                }
            }
            e => {
							events.push(e);
            }
        }
    }


		let mut ret = String::new();
    html::push_html(&mut ret, events.into_iter());
		Ok(ret)

}


pub fn render_markdown<S: AsRef<str>>(src: S) -> String {
	let parser = pulldown_cmark::Parser::new(src.as_ref());
	let mut html_output = String::new();
	pulldown_cmark::html::push_html(&mut html_output, parser);
	html_output
}