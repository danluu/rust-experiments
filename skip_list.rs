use core::rand::RngUtil;

struct Node {
    left: MaybeNode,
    right: MaybeNode,
    down: MaybeNode,
    up: MaybeNode,
    data: int
}

enum MaybeNode {
    SomeNode(@mut Node),
    NoNode
}

fn print_nodes(x: @mut Node){
    println(fmt!("%d", x.data));
    match x.right {
        NoNode => println("Done!"),
        SomeNode(n) => print_nodes(n)
    }
    
}

// insert node y after node x
fn insert_after(x: @mut Node, y: @mut Node){
    match x.right {
        SomeNode(xn) => {
            xn.left = SomeNode(y);
            x.right = SomeNode(y);
            y.left = SomeNode(x);
            y.right = SomeNode(xn);
        }
        NoNode => {
            x.right = SomeNode(y);
            y.left = SomeNode(x);
        }
    }
}

// insert node y before node x
fn insert_before(x: @mut Node, y: @mut Node){
    match x.left {
        SomeNode(xp) => {
            x.left = SomeNode(y);
            xp.right = SomeNode(y);
            y.left = SomeNode(xp);
            y.right = SomeNode(x)
        }
        NoNode => {
            x.left = SomeNode(y);
            y.right = SomeNode(x);
        }
    }
}

fn insert_balanced_helper(list: MaybeNode, x: @mut Node, left: MaybeNode){
    match list {
        SomeNode(n) => {
            if (x.data < n.data) {
                insert_before(n, x);
            } else {
                insert_balanced_helper(n.right, x, list);
            }
        }
        NoNode => match left {
            SomeNode(n) => insert_after(n, x),
            NoNode => println("FIXME: can't insert into empty list")
        }
    }
}

fn insert_balanced(list: MaybeNode, x: @mut Node){
    insert_balanced_helper(list, x, NoNode);
}

fn main() {
    let rng = rand::Rng();
    println(fmt!("%f",rng.gen_float()));
  
    let mut max_height = 0;
    let mut top = NoNode;

    let head0 = @mut Node { right: NoNode, left: NoNode, up: NoNode, down: NoNode, data: int::min_value };
    let node1000 = @mut Node { right: NoNode, left: SomeNode(head0), up: NoNode, down: NoNode, data: 1000 };
    head0.right = SomeNode(node1000);
    top = SomeNode(head0);
    
    print_nodes(head0);

    match top {
        NoNode => println("Empty top"),
        SomeNode(n) => print_nodes(n)
    }

/*
    let node150 = @mut Node { right: NoNode, left: NoNode, data: 150 };
    let node200 = @mut Node { right: NoNode, left: NoNode, data: 200 };
    let node250 = @mut Node { right: NoNode, left: NoNode, data: 250 };
    let node300 = @mut Node { right: NoNode, left: NoNode, data: 300 };

    insert_after(node100, node200);
    insert_after(node200, node300);
    insert_after(node200, node250);
    insert_before(node200, node150);

    let node75 =  @mut Node { right: NoNode, left: NoNode, data: 75 };
    let node125 =  @mut Node { right: NoNode, left: NoNode, data: 125 };
    let node325 =  @mut Node { right: NoNode, left: NoNode, data: 325 };
    insert_balanced(SomeNode(node100), node75);
    insert_balanced(SomeNode(node75), node125);
    insert_balanced(SomeNode(node75), node325);

    print_nodes(node75);
*/
}

#[test]
fn search_simple() {
    let mut max_height = 0;
    let mut top = NoNode;

    let head0 = @mut Node { right: NoNode, left: NoNode, up: NoNode, down: NoNode, data: int::min_value };
    let head1 = @mut Node { right: NoNode, left: NoNode, up: NoNode, down: SomeNode(head0), data: int::min_value };
    let head2 = @mut Node { right: NoNode, left: NoNode, up: NoNode, down: SomeNode(head1), data: int::min_value };
    head0.up = SomeNode(head1);
    head1.up = SomeNode(head2);
    top = SomeNode(head2);

    let node1000 = @mut Node { right: NoNode, left: SomeNode(head0), up: NoNode, down: NoNode, data: 1000 };
    head0.right = SomeNode(node1000);

    let node2000 = @mut Node { right: NoNode, left: SomeNode(node1000), up: NoNode, down: NoNode, data: 1000 };
    let node2000u = @mut Node { right: NoNode, left: SomeNode(head1), up: NoNode, down: NoNode, data: 1000 };
    let node2000uu = @mut Node { right: NoNode, left: SomeNode(head2), up: NoNode, down: NoNode, data: 1000 };
    head1.right = SomeNode(node2000u);
    head2.right = SomeNode(node2000uu);

    let node3000 = @mut Node { right: NoNode, left: SomeNode(node2000), up: NoNode, down: NoNode, data: 1000 };
    node2000.right = SomeNode(node3000);
}
