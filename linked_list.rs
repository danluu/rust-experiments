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

// insert node y before node x
fn insert_before(x: @mut Node, y: @mut Node){
    match x.prev {
        SomeNode(xp) => {
            xp.next = SomeNode(y);
            x.prev = SomeNode(y);
            y.prev = SomeNode(xp);
            y.next = SomeNode(x)
        }
        NoNode => {
            x.prev = SomeNode(y);
            y.next = SomeNode(x);
        }
    }
}

fn insert_balanced(x: @mut Node){
    match x.next {
        SomeNode(xn) => {
            
        }
        NoNode => {

        }
    }
}

fn main() {
    let node100 = @mut Node { next: NoNode, prev: NoNode, data: 100 };
    let node150 = @mut Node { next: NoNode, prev: NoNode, data: 150 };
    let node200 = @mut Node { next: NoNode, prev: NoNode, data: 200 };
    let node250 = @mut Node { next: NoNode, prev: NoNode, data: 250 };
    let node300 = @mut Node { next: NoNode, prev: NoNode, data: 300 };

    insert_after(node100, node200);
    insert_after(node200, node300);
    insert_after(node200, node250);
    insert_before(node200, node150);

    print_nodes(node100);
}


