use super::*;

ast_struct! {
    pub struct Crate {
        pub shebang: Option<String>,
        pub attrs: Vec<Attribute>,
        pub items: Vec<Item>,
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;
    use attr::parsing::inner_attr;
    use item::parsing::items;

    named!(pub krate -> Crate, do_parse!(
        // NOTE: The byte order mark and shebang are not tokens which can appear
        // in a TokenStream, so we can't parse them anymore.

        //option!(byte_order_mark) >>
        //shebang: option!(shebang) >>
        attrs: many0!(inner_attr) >>
        items: items >>
        (Crate {
            shebang: None,
            attrs: attrs,
            items: items,
        })
    ));

    // named!(byte_order_mark -> &str, tag!("\u{feff}"));

    // named!(shebang -> String, do_parse!(
    //     tag!("#!") >>
    //     not!(tag!("[")) >>
    //     content: take_until!("\n") >>
    //     (format!("#!{}", content))
    // ));
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;
    use attr::FilterAttrs;
    use quote::{Tokens, ToTokens};

    impl ToTokens for Crate {
        fn to_tokens(&self, tokens: &mut Tokens) {
            // TODO: how to handle shebang?
            // if let Some(ref shebang) = self.shebang {
            //     tokens.append(&format!("{}\n", shebang));
            // }
            tokens.append_all(self.attrs.inner());
            tokens.append_all(&self.items);
        }
    }
}
