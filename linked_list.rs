struct Node<T> {
    next: MaybeNode<T>,
    prev: MaybeNode<T>,
    data: T
}

enum MaybeNode<T> {
    SomeNode(@mut Node<T>),
    NoNode
}

fn main() {
    let node1 = @mut Node { next: NoNode, prev: NoNode, data: 1 };
    let node2 = @mut Node { next: NoNode, prev: NoNode, data: 2 };
    let node3 = @mut Node { next: NoNode, prev: NoNode, data: 3 };

    node1.next = SomeNode(node2);
    node2.next = SomeNode(node3);
    node3.next = SomeNode(node1);
}


