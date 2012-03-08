#[doc = "
Extracts markdown-formatted descriptions into specific doc fields

This allows for rustdocs to be written like:

    The frob function

    Arguments:

    * foo - The foo argument
    * bar - The bar argument

"];

export mk_pass;

fn mk_pass() -> pass {
    {
        name: "attr",
        f: run
    }
}

fn run(_srv: astsrv::srv, doc: doc::doc) -> doc::doc {
    let fold = fold::fold({
        fold_fn: fold_fn
        with *fold::default_any_fold(())
    });
    fold.fold_doc(fold, doc)
}

fn fold_fn(
    fold: fold::fold<()>,
    doc: doc::fndoc
) -> doc::fndoc {

    let srv = fold.ctxt;
    let doc = fold::default_seq_fold_fn(fold, doc);

    let (desc, return) = parse_return_doc(doc.desc());

    {
        item: {
            desc: desc
            with doc.item
        },
        return: if option::is_some(return) {
            return
        } else {
            doc.return
        }
        with doc
    }
}

fn parse_return_doc(desc: option<str>) -> (option<str>, option<str>) {
    let desc = alt desc {
      some(desc) { desc }
      none {
        ret (none, none);
      }
    };

    ret (none, none);
}

#[test]
fn should_extract_return_value_text() {

    let doc = test::mk_doc(
        "#[doc = \" \
         Return: a\
         \
         b\"]\
         fn a() { }");
    assert doc.cratemod().fns()[0].return.desc == some("a\n\nb");
}

#[test]
fn should_remove_return_value_text() {
    let doc = test::mk_doc(
        "#[doc = \"whatever\nReturn: hey\n\"] fn a() { }");
    let desc = option::get(doc.cratemod().fns()[0].desc());
    assert !str::contains(desc, "Return: hey");
}

#[test]
fn should_remove_desc_if_it_contains_only_return_value() {
    let doc = test::mk_doc("#[doc = \"Return: hey\"] fn a() { }");
    assert doc.cratemod().fns()[0].desc() == none;
}

#[cfg(test)]
mod test {
    fn mk_doc(source: str) -> doc::doc {
        astsrv::from_str(source) {|srv|
            let doc = extract::from_srv(srv, "");
            let doc = attr_pass::mk_pass().f(srv, doc);
            run(srv, doc)
        }
    }
}

