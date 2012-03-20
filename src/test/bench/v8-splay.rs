// Based on splay.js from version 7 of v8

const kSplayTreeSize: int = 8000;
const kSplayTreeModifications: int = 80;
const kSplayTreePayloadDepth: int = 5;

enum Payload {
    Leaf({
        array: [int],
        string: str
    }),
    Branch({
        left: ~Payload,
        right: ~Payload
    })
}

fn GeneratePayloadTree(depth: int, tag: str) -> {
    if depth == 0 {
        ret Leaf({
            array: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            string: "String for key " + tag + " in leaf node"
        });
    } else {
        ret Branch({
        left: GeneratePayloadTree(depth - 1, tag),
        right: GeneratePayloadTree(depth - 1, tag)
        });
    }
}

fn GenerateKey(rng: rand::rng) -> uint {
    rng::next() as uint
}

fn InsertNewNode(splayTree: SplayTree, rng: rand::rng) {
    let mut key;
    do {
        key = GenerateKey(rng);
    } while splayTree.find(key) != none;
    let payload = GeneratePayloadTree(kSplayTreePayloadDepth, uint::str(key));
    splayTree.insert(key, payload);
    ret key;
}

fn SplaySetup() -> SplayTree {
    let splayTree = new SplayTree();
    iter::repeat(kSplayTreeSize as uint) {||
        InsertNewNode(splayTree);
    }
    ret splayTree;
}

fn SplayTearDown(splayTree: SplayTree) {
    let keys = splayTree.exportKeys();

    let length = keys.length;
    if length != kSplayTreeSize {
        fail "Splay tree has wrong size";
    }

    int::range(0, length - 1) {|i|
        if keys[i] >= keys[i + 1] {
            fail "Splay tree not sorted";
        }
    }
}

fn SplayRun(splayTree: SplayTree) {
    int::range(0, kSplayTreeModifications) {|i|
        let key = InsertNewNode();
        let greatest = splayTree.findGreatestLessThan(key);
        if greatest == none {
            splayTree.remove(key);
        } else {
            splayTree.remove(option::get(greatest).key);
        }
    }
}

fn SplayTree() -> SplayTree {
}

type SplayTree = {
    root_: @option<SplayTree>,
    node: ()
};

impl util for SplayTree {
    fn isEmpty() -> bool {
        option::is_none(self.root_)
    }

    fn insert(key: uint, value: Payload) {
        if self.isEmpty() {
            self.root_ = Node(key, value);
            ret;
        }

        // Splay on the key to move the last node on the search path for
        // the key to the root of the tree.
        self.splay_(key);
        if (*self.root_).key == key {
            ret;
        }

        let node = Node(key, value);
        if key > (*self.root_).key {
            node.left = self.root_;
            node.right = self.root_.right;
            self.root_.right = none;
        }
        self.root_ = node;
    }
}

