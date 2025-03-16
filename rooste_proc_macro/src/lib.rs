use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "kollektsioonid" => "collections",
        "Viga" => "Err",
        "Okei" => "Ok",
        "Sõne" => "String",
        "Paisktabel" => "HashMap",
        "Vaikimisi" => "Default",
        "Error" => "Error",
        "Võimalik" => "Option",
        "Olemas" => "Some",
        "Puudu" => "None",
        "Tulemus" => "Result",
        "Ise" => "Self",
        "trüki" => "println",
        "katke" => "break",
        "asünk" => "async",
        "oota" => "await",
        "kordus" => "loop",
        "liiguta" => "move",
        "teek" => "crate",
        "kättesaamatu_kood" => "unreachable_code",
        "nagu" => "as",
        "konstant" => "const",
        "omadus" => "trait",
        "ebaturvaline" => "unsafe",
        "olles" => "in",
        "loo" => "from",
        "dün" | "dünaamiline" => "dyn",
        "ava" => "unwrap",
        "vaikimisi" => "default",
        "viitena" => "as_ref",
        "es" => "io",
        "väline" => "extern",
        "väär" => "false",
        "funktsioon" => "fn",
        "ülem" => "super",
        "sisesta" => "insert",
        "võta" => "get",
        "luba" => "allow",
        "raisk" | "paanika" | "ups" => "panic",
        "moodul" => "mod",
        "muutuv" => "mut",
        "uus" => "new",
        "kus" => "where",
        "iga" | "jaoks" => "for",
        "võta_või_sisesta_koos" => "get_or_insert_with",
        "peamine" => "main",
        "avalik" => "pub",
        "Puudub" => None?,
        "tagasta" => "return",
        "teostus" => "impl",
        "viide" => "ref",
        "sobita" => "match",
        "kui" => "if",
        "muidu" => "else",
        "ise" => "self",
        "on" => "let",
        "staatiline" => "static",
        "struktuur" => "struct",
        "eelda" => "expect",
        "kuni" => "while",
        "kasuta" => "use",
        "muunda" => "into",
        "tõene" => "true",
        "loend" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rooste(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
