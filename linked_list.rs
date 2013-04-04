struct Node {
    next: MaybeNode,
    prev: MaybeNode,
    data: int
}

enum MaybeNode {
    SomeNode(@mut Node),
    NoNode
}

fn print_nodes(x: @mut Node){
    println(fmt!("%d", x.data));
    match x.next {
        NoNode => println("Done!"),
        SomeNode(n) => print_nodes(n)
    }
    
}

// insert node y after node x
fn insert_after(x: @mut Node, y: @mut Node){
    match x.next {
        SomeNode(xn) => {
            xn.prev = SomeNode(y);
            x.next = SomeNode(y);
            y.prev = SomeNode(x);
            y.next = SomeNode(xn);
        }
        NoNode => {
            x.next = SomeNode(y);
            y.prev = SomeNode(x);
        }
    }
}

fn main() {
    let node1 = @mut Node { next: NoNode, prev: NoNode, data: 1 };
    let node2 = @mut Node { next: NoNode, prev: NoNode, data: 2 };
    let node3 = @mut Node { next: NoNode, prev: NoNode, data: 3 };

    insert_after(node1, node2);
    insert_after(node2, node3);


    print_nodes(node1);
}


