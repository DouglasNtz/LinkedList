use Linked_List::{LinkedList, MyQueue, MyStack, MySmartQueue};


fn main() {

    let mut my_stack = MyStack::<_,5>::new(); // stack de tamanho 5
    my_stack.show();
    println!("O array está vazio: {:?}", my_stack.is_empty());
    my_stack.push(3);
    my_stack.push(7);
    my_stack.push(2);
    my_stack.show();
    my_stack.pop();
    my_stack.show();
    println!("O array está vazio: {:?}", my_stack.is_empty());
    my_stack.pop();
    my_stack.pop();
    my_stack.show();
    println!("O array está vazio: {:?}", my_stack.is_empty());
    my_stack.pop();
    my_stack.push(3);
    my_stack.push(7);
    my_stack.push(2);
    my_stack.push(3);
    my_stack.push(7);
    my_stack.show();
    println!("O array está cheio: {:?}", my_stack.is_full());
    my_stack.push(2);


    let mut my_queue = MyQueue::<_,6>::new(); // queue de tamanho 6, capacidade para 5
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.push(3);
    my_queue.push(7);
    my_queue.push(2);
    my_queue.show();
    my_queue.pop();
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.pop();
    my_queue.pop();
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.pop();
    my_queue.push(3);
    my_queue.push(7);
    my_queue.push(2);
    my_queue.push(3);
    my_queue.push(7);
    my_queue.show();
    println!("O array está cheio: {:?}", my_queue.is_full());
    my_queue.push(2);


    let mut my_queue = MySmartQueue::new();
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.push(3);
    my_queue.push(7);
    my_queue.push(2);
    my_queue.show();
    my_queue.pop();
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.pop();
    my_queue.pop();
    my_queue.show();
    println!("O array está vazio: {:?}", my_queue.is_empty());
    my_queue.pop();
    my_queue.push(3);
    my_queue.push(7);
    my_queue.push(2);
    my_queue.push(3);
    my_queue.push(7);
    my_queue.show();

    //----------------

    let mut link = LinkedList::new();

    println!("Linked lista vazia: {}", link.is_empty());

    link.prepend(1, "A".to_string());
    link.show();
    link.prepend(4, "B".to_string());
    link.show();
    link.prepend(16, "C".to_string());
    link.show();
    link.prepend(9, "D".to_string());
    link.show();
    link.prepend(25, "E".to_string());
    link.show();
    link.insert(36, "F".to_string(), 9);
    link.show();
    link.delete(4);
    link.show();
    println!("{:?}", link.get(36));

    let x = link.get_mut(36);
    x.unwrap().push_str("X");
    println!("{:?}", link.get(36));

    println!("{:?}", link.get_antecessor(36));

    let x = link.get_mut_antecessor(36);
    x.unwrap().push_str("Y");
    println!("{:?}", link.get_antecessor(36));

    println!("{:?}", link.get_sucessor(36));

    let x = link.get_mut_sucessor(36);
    x.unwrap().push_str("W");
    println!("{:?}", link.get_sucessor(36));

    println!("{:?}", link.get_sucessor(1));
    println!("{:?}", link.get_antecessor(25));

    println!("Linked list vazia: {}", link.is_empty());


    //----------

    let mut link = LinkedList::new();

    println!("Linked lista vazia: {}", link.is_empty());

    link.pospend(25, "E".to_string());
    link.show();
    link.pospend(9, "D".to_string());
    link.show();
    link.pospend(16, "C".to_string());
    link.show();
    link.pospend(4, "B".to_string());
    link.show();
    link.pospend(1, "A".to_string());
    link.show();
    link.insert(36, "F".to_string(), 9);
    link.show();
    link.delete(4);
    link.show();
    println!("{:?}", link.get(36));

    let x = link.get_mut(36);
    x.unwrap().push_str("X");
    println!("{:?}", link.get(36));

    println!("{:?}", link.get_antecessor(36));

    let x = link.get_mut_antecessor(36);
    x.unwrap().push_str("Y");
    println!("{:?}", link.get_antecessor(36));

    println!("{:?}", link.get_sucessor(36));

    let x = link.get_mut_sucessor(36);
    x.unwrap().push_str("W");
    println!("{:?}", link.get_sucessor(36));

    println!("{:?}", link.get_sucessor(1));
    println!("{:?}", link.get_antecessor(25));

    println!("Linked list vazia: {}", link.is_empty());

}


