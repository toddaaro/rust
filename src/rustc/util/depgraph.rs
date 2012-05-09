#[doc = "

Dependencies between `def_id`s

"];

export depgraph::{}, edge, methods;

import std::map::hashmap;
import util::common::new_def_hash;
import syntax::ast;
import syntax::ast::def_id;
import syntax::ast_util;
import driver::session::session;
import middle::ty;
import metadata::cstore;

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
    structural_dep,
    type_dep,
}

impl methods for depgraph {
    fn is_direct_dep(a: def_id, b: def_id) -> bool { fail }
    fn has_direct_dep(a: def_id, b: def_id) -> bool { fail }
}

fn depgraph(sess: session,
            tcx: ty::ctxt
           ) -> depgraph {

    type ctxt = {
        sess: session,
        tcx: ty::ctxt,
        in_edges: hashmap<def_id, edge>,
        out_edges: hashmap<def_id, edge>
    };

    let ctxt = {
        sess: sess,
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
