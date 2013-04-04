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

fn main() {
    let node1 = @mut Node { next: NoNode, prev: NoNode, data: 1 };
    let node2 = @mut Node { next: NoNode, prev: NoNode, data: 2 };
    let node3 = @mut Node { next: NoNode, prev: NoNode, data: 3 };

    node1.next = SomeNode(node2);
    node2.next = SomeNode(node3);

    print_nodes(node1);
}


