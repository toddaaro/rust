#[doc = "

Dependencies between `def_id`s

"];

export depgraph::{}, edge, methods;

import std::map::hashmap;
import util::common::new_def_hash;
import syntax::ast;
import syntax::ast::def_id;
import syntax::ast_util;
import driver::session;
import driver::session::session;
import middle::ty;
import metadata::cstore;
import middle::ast_map;
import driver::driver;

enum depgraph {
    depgraph_(@{
        in_edges: hashmap<def_id, edge>,
        out_edges: hashmap<def_id, edge>
    })
}

type edge = {
    in: def_id,
    out: def_id,
    kind: depkind
};

enum depkind {
    any,
    structural_dep,
    type_dep,
}

impl methods for depgraph {
    fn has_direct_dep(a: def_id, b: def_id, kind: depkind) -> bool { fail }
    fn is_direct_dep(a: def_id, b: def_id, kind: depkind) -> bool { fail }
}

fn depgraph(sess: session,
            ast_map: ast_map::map,
            tcx: ty::ctxt
           ) -> depgraph {

    type ctxt = {
        sess: session,
        ast_map: ast_map::map,
        tcx: ty::ctxt,
        in_edges: hashmap<def_id, edge>,
        out_edges: hashmap<def_id, edge>
    };

    let ctxt = {
        sess: sess,
        ast_map: ast_map,
        tcx: tcx,
        in_edges: new_def_hash(),
        out_edges: new_def_hash()
    };

    int::range(0, sess.next_node_id()) {|local_node_id|
        add_def_deps(ctxt, ast_util::local_def(local_node_id));
    }

    ret depgraph_(@{
        in_edges: ctxt.in_edges,
        out_edges: ctxt.out_edges
    });

    fn add_def_deps(ctxt: ctxt, def_id: def_id) {
    }
}

#[cfg(test)]
mod test {

    import rustsyntax::diagnostic;

    fn path_to_def_id(ast_map: ast_map::map, path: str) -> [def_id] {
        let mut candidates = [];
        ast_map.each_key {|def_id|
            alt ast_map::node_id_to_path(ast_map, def_id) {
              some(p) {
                if ast_map::path_to_str(p) == path {
                    candidates += [def_id];
                }
              }
              none { }
            }
        }
        ret candidates;
    }

    type test_ctxt = {
        sess: session,
        ast_map: ast_map::map,
        tcx: ty::ctxt
    };

    fn make_ctxt(src: str) -> test_ctxt {
        let sopts: @session::options = session::basic_options();
        let sess = driver::build_session(sopts, diagnostic::emit);
        let input = driver::str_input(src);
        let cfg = driver::build_configuration(sess, "", input);
        let {crate, tcx} = driver::compile_upto(
            sess, cfg, input, driver::cu_typeck, none);

        {
            sess: sess,
            ast_map: tcx.items,
            tcx: tcx
        }
    }

    #[test]
    fn fn_is_direct_dep_of_parent_mod() {
        let src = "mod m { fn f() { } }";
        let cx = make_ctxt(src);
        let dg = depgraph(cx.sess, cx.ast_map, cx.tcx);
        let m_id = path_to_def_id(cx.ast_map, "m");
        let f_id = path_to_def_id(cx.ast_map, "m::f");
        assert dg.has_direct_dep(m_id, f_id);
        assert dg.is_direct_dep(f_id, m_id);
    }

}