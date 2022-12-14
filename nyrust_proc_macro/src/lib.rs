use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Feil" => "Err",
        "Ok" => "Ok",
        "Streng" => "String",
        "Ordbok" => "HashMap",
        "Tilbakefallsverdi" => "Default",
        "Feilfunksjon" => "Error",
        "Kanskje" => "Option",
        "Noko" => "Some",
        "Ingen" => "None",
        "Resultat" => "Result",
        "Sjølv" => "Self",
        "skrivln" => "println",
        "bryt" => "break",
        "asynkron" => "async",
        "vent" => "await",
        "løkke" => "loop",
        "flytt" => "move",
        "kasse" => "crate",
        "unåeleg_kode" => "unreachable_code",
        "som" => "as",
        "konstant" => "const",
        "grensesnitt" => "trait",
        "utrygg" => "unsafe",
        "i" => "in",
        "frå" => "from",
        "dynamisk" => "dyn",
        "pakk_ut" => "unwrap",
        "tilbakefallsverdi" => "default",
        "som_referanse" => "as_ref",
        "iu" => "io",
        "ekstern" => "extern",
        "usant" => "false",
        "funksjon" => "fn",
        "forelder" => "super",
        "sett_inn" => "insert",
        "hent" => "get",
        "godta" => "allow",
        "panikk" | "faen" | "oisann" => "panic",
        "modul" => "mod",
        "muterbar" => "mut",
        "ny" => "new",
        "kor" => "where",
        "for" => "for",
        "hent_eller_tilbakefall_til" => "get_or_insert_with",
        "hovud" => "main",
        "offentleg" => "pub",
        "hæ?" => None?,
        "gje_tilbake" => "return",
        "oppfyll" => "impl",
        "referanse" => "ref",
        "jamstill" => "match",
        "viss" => "if",
        "elles" => "else",
        "sjølv" => "self",
        "la" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "forvent" => "expect",
        "mens" => "while",
        "bruk" => "use",
        "til" => "into",
        "sant" => "true",
        "opplisting" => "enum",
        "Gruppe" => "Group",
        "Kjennemerke" => "Ident",
        "Nøkkelordstrøm" => "TokenStream",
        "Nøkkelordtre" => "TokenTree",
        "til_streng" => "to_string",
        "som_streng" => "as_str",
        "rekkevidd" => "span",
        "Vek" => "Vec",
        "strøm" => "stream",
        "sett_på" => "push",
        "utvid" => "extend",
        "skilletegn" => "delimiter",
        "Punktsetjing" => "Punct",
        "Bokstaveleg" => "Literal",
        "erstatt_syntaks" => "proc_macro",
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
pub fn rouille(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
