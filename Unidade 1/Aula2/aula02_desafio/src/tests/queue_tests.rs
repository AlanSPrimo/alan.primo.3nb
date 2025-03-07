use meu_projeto::queue::Queue; // Importa a estrutura Queue da biblioteca

#[test]
fn test_enqueue_dequeue() {
    let mut queue = Queue::new();

    assert!(queue.is_empty());
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(queue.len(), 3);
    assert_eq!(queue.peek(), Some(&1));

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.len(), 1);

    assert_eq!(queue.dequeue(), Some(3));
    assert!(queue.is_empty());
}

#[test]
fn test_peek() {
    let mut queue = Queue::new();
    queue.enqueue(42);
    
    assert_eq!(queue.peek(), Some(&42));
    assert_eq!(queue.len(), 1);
}

#[test]
fn test_empty_dequeue() {
    let mut queue: Queue<i32> = Queue::new();
    assert_eq!(queue.dequeue(), None);
}
