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
  

    let head = @mut Node { right: NoNode, left: NoNode, up: NoNode, down: NoNode, data: int::min_value };
    let node1000 = @mut Node { right: NoNode, left: SomeNode(head), up: NoNode, down: NoNode, data: 1000 };
    head.right = SomeNode(node1000);

    print_nodes(head);
  
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
