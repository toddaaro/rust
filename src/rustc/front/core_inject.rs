import driver::session::session;
import syntax::codemap;
import syntax::ast;
import syntax::ast_util::*;
import syntax::attr;

export maybe_inject_common_refs;

fn maybe_inject_common_refs(sess: session,
                            crate: @ast::crate) -> @ast::crate {
    let use_rt = use_rt(crate);
    let use_core = use_core(crate);
    if use_rt || use_core {
        inject_libcore_ref(sess, crate, use_rt, use_core)
    } else {
        crate
    }
}

fn use_rt(crate: @ast::crate) -> bool {
    !attr::attrs_contains_name(crate.node.attrs, "no_rt")
}

fn use_core(crate: @ast::crate) -> bool {
    !attr::attrs_contains_name(crate.node.attrs, "no_core")
}

fn inject_libcore_ref(sess: session,
                      crate: @ast::crate,
                      use_rt: bool,
                      use_core: bool) -> @ast::crate {

    fn spanned<T: copy>(x: T) -> @ast::spanned<T> {
        ret @{node: x,
            span: dummy_sp()};
    }

    let n1 = sess.next_node_id();
    let n2 = sess.next_node_id();

    let mut vis = [];
    if use_rt {
        vis += [@{node: ast::view_item_use(@"rt", [], n1),
                  attrs: [],
                  vis: ast::public,
                  span: dummy_sp()}];
    }
    if use_core {
        vis += [@{node: ast::view_item_use(@"core", [], n1),
                  attrs: [],
                  vis: ast::public,
                  span: dummy_sp()}];
        let vp = spanned(ast::view_path_glob(
            ident_to_path(dummy_sp(), @"core"),
            n2));
        vis += [@{node: ast::view_item_import([vp]),
                  attrs: [],
                  vis: ast::public,
                  span: dummy_sp()}];
    }

    vis += crate.node.module.view_items;

    ret @{node: {module: { view_items: vis with crate.node.module }
                 with crate.node} with *crate }
}
