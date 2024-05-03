use std::fmt::Debug;

pub struct MyStack<T, const N: usize> {
    array: [Option<T>; N],
    top: Option<usize>
}

impl<T: Copy, const N: usize> MyStack<T,N> {
    pub fn new() -> Self {
        Self { array: [None; N], top: None }
    }

    pub fn is_empty(self: &Self) -> bool {
        match self.top {
            Some(i) => false,
            None => true
        }
    }

    pub fn is_full(self: &Self) -> bool {
        match self.top {
            Some(i) => {
                if i == N - 1 {
                    true
                } else {
                    false
                }
            },
            None => false
        }
    }

    pub fn push(self: &mut Self, element: T) {
        self.top = match self.top {
            Some(i) => {
                if i == (N - 1) {
                    println!("O array já está no tamanho máximo. Não podemos inserir esse novo elemento.");
                    return;
                } else {
                    Some(i + 1)
                }
            },
            None => Some(0)
        };

        self.array[self.top.unwrap()] = Some(element);
    }

    pub fn pop(self: &mut Self) -> Option<T> {

        let mut pop_element = None;

        match self.top {
            Some(i) => {
                pop_element = self.array[i].take();

                if i > 0 {
                    self.top = Some(i - 1)
                } else {
                    self.top = None
                }
            },
            None => println!("O array está vazio. Não temos elemento para extrair.")
        }
        pop_element
    }
}
impl<T: Debug, const N: usize> MyStack<T,N> {

    pub fn show(self: &Self) {

        let mut v = vec![];

        match self.top {
            Some(i) => {

                for j in 0..=i {

                    v.push(self.array[j].as_ref().unwrap());
                }
            }
            None => {},
        }
        println!("{:?}", v);
    }
}

//----------------------

pub struct MyQueue<T, const N: usize> {
    array: [Option<T>; N],
    head: usize,
    tail: usize
}

impl<T: Copy, const N: usize> MyQueue<T, N> {   // o máximo é deixar um espaço livre
    pub fn new() -> Self {
        Self { array: [None; N], head: 0, tail: 0}
    }

    pub fn is_empty(self: &Self) -> bool {

        self.head == self.tail
    }

    pub fn is_full(self: &Self) -> bool {

        if self.tail > self.head {
            self.tail - self.head == (N - 1)
        } else {
            self.head - self.tail == 1
        }
    }

    pub fn push(self: &mut Self, element: T) {

        if self.is_full() {

            println!("O array já está no tamanho máximo. Não podemos inserir esse novo elemento.");

            return;

        } else {

            self.array[self.tail] = Some(element);

            if self.tail < (N - 1) {
                self.tail += 1;
            } else {
                self.tail = 0
            }
        }
    }

    pub fn pop(self: &mut Self) -> Option<T> {

        let pop_element;

        if self.is_empty() {

            println!("O array está vazio. Não temos elemento para extrair.");

            pop_element = None;

        } else {

            pop_element = self.array[self.head].take();

            if self.head < (N - 1) {
                self.head += 1;
            } else {
                self.head = 0;
            }
        }

        pop_element
    }
}
impl<T: Debug, const N: usize> MyQueue<T, N> {

    pub fn show(self: &Self) {

        let mut v = vec![];

        if self.tail > self.head {

            for i in self.head..self.tail {

                v.push(self.array[i].as_ref().unwrap())
            }

        } else if self.tail < self.head {

            for i in self.head..N {

                v.push(self.array[i].as_ref().unwrap())
            }
            for i in 0..self.tail {

                v.push(self.array[i].as_ref().unwrap())
            }

        }

        println!("{:?}", v);
    }
}

//----------------------------

pub struct MySmartQueue<T> {
    olds: Vec<T>,
    news: Vec<T>
}

impl<T> MySmartQueue<T> {

    pub fn new() -> Self {
        Self {olds: Vec::new(), news: Vec::new()}
    }

    pub fn is_empty(self: &Self) -> bool {
        self.olds.is_empty() && self.news.is_empty()
    }

    pub fn push(self: &mut Self, element: T) {

        self.news.push(element)

    }

    pub fn pop(self: &mut Self) -> Option<T> {

        if self.olds.is_empty() {

            std::mem::swap(&mut self.news, &mut self.olds);
            self.olds.reverse();
        }

        self.olds.pop()
    }
}

impl<T: Debug> MySmartQueue<T> {

    pub fn show(self: &Self) {

        let mut v = vec![];

        let old_len = self.olds.len();

        let new_len = self.news.len();

        for i in 0..old_len {

            v.push(&self.olds[old_len - i - 1])
        }
        for i in 0..new_len {

            v.push(&self.news[i])
        }

        println!("{:?}", v);
    }

}

//-----------------------------------

struct Element<T: Copy + PartialEq, E> {
    prev: Option<usize>,
    key: T,
    next: Option<usize>,
    satelite: E
}
pub struct LinkedList<T: Copy + PartialEq, E> {

    head: Option<usize>,

    list: Vec<Element<T,E>>
}

impl<T: Copy + PartialEq, E> LinkedList<T, E> {

    pub fn new() -> Self {

        Self {head: None, list: Vec::new()}
    }

    pub fn is_empty(self: &Self) -> bool {

        self.head.is_none()
    }

    pub fn prepend(self: &mut Self, key: T, satelite: E) {

        self.list.push(Element {prev: None, key, next: self.head, satelite: satelite});

        match self.head {

            Some(i) => self.list[i].prev = Some(self.list.len() - 1),

            None => {}
        }

        self.head = Some(self.list.len() - 1);

    }

    pub fn insert(self: &mut Self, key: T, satelite: E, prev_key: T) {

        let mut pos = None;

        for (i, element) in self.list.iter().enumerate() {
            if element.key == prev_key {
                pos = Some(i);
                break
            }
        }

        match pos {

            Some(i) => {

                let next = self.list[i].next;

                let ultimo = Some(self.list.len());

                self.list[i].next = ultimo;

                match next {
                    Some(j) => self.list[j].prev = ultimo,
                    None => {}
                }

                self.list.push(Element { prev: Some(i), key, next, satelite});
            }

            None => println!("Chave prévia inexistente!")
        }
    }

    pub fn delete(self: &mut Self, key: T) {

        let mut pos = None;

        for (i, element) in self.list.iter().enumerate() {

            if element.key == key {

                pos = Some(i);

                break
            }
        }

        match pos {

            Some(i) => {

                let prev = self.list[i].prev;
                let next = self.list[i].next;

                match prev {

                    Some(j) => self.list[j].next = next,
                    None => self.head = next
                }

                match next {

                    Some(j) => self.list[j].prev = prev,
                    None => {}
                }

                let ultimo = self.list.len() - 1;

                self.list.swap(i, ultimo);  // trocamos com um último índice

                self.list.pop();

                // agora temos que arrumar o prev e next do elemento que era o último índice e virou índice i

                let prev = self.list[i].prev;
                let next = self.list[i].next;

                match prev {

                    Some(j) => self.list[j].next = Some(i),
                    None => self.head = Some(i)
                }

                match next {

                    Some(j) => self.list[j].prev = Some(i),
                    None => {}
                }
            }
            None => println!("Chave inexistente!")
        }


    }

    pub fn get(self: &Self, key: T) -> Option<&E> {

        for element in &self.list {

            if element.key == key {

                return Some(&element.satelite)
            }
        }

        println!("Chave inexistente!");
        None
    }

    pub fn get_mut(self: &mut Self, key: T) -> Option<&mut E> {

        for element in &mut self.list {

            if element.key == key {

                return Some(&mut element.satelite)
            }
        }

        println!("Chave inexistente!");
        None
    }

    pub fn get_sucessor(self: &Self, key: T) -> Option<&E> {

        let mut chave = 0;

        let mut next = None;

        for element in &self.list {

            if element.key == key {

                next = element.next;

                chave = 1;

                break;
            }
        }

        if chave == 0 {

            println!("Chave inexistente!");
            return None
        }

        match next {

            Some(i) => return Some(&self.list[i].satelite),

            None => {
                println!("Não existe sucessor... chave selecionada é a cauda da série");
                return None
            }
        }
    }

    pub fn get_mut_sucessor(self: &mut Self, key: T) -> Option<&mut E> {

        let mut chave = 0;

        let mut next = None;

        for element in &self.list {

            if element.key == key {

                next = element.next;

                chave = 1;

                break;
            }
        }

        if chave == 0 {

            println!("Chave inexistente!");
            return None
        }

        match next {

            Some(i) => return Some(&mut self.list[i].satelite),

            None => {
                println!("Não existe sucessor... chave selecionada é a cauda da série");
                return None
            }
        }
    }

    pub fn get_antecessor(self: &Self, key: T) -> Option<&E> {

        let mut chave = 0;

        let mut prev = None;

        for element in &self.list {

            if element.key == key {

                prev = element.prev;

                chave = 1;

                break;
            }
        }

        if chave == 0 {

            println!("Chave inexistente!");
            return None
        }

        match prev {

            Some(i) => return Some(&self.list[i].satelite),

            None => {
                println!("Não existe antecessor... chave selecionada é a cabeça da série");
                return None
            }
        }

    }

    pub fn get_mut_antecessor(self: &mut Self, key: T) -> Option<&mut E> {

        let mut chave = 0;

        let mut prev = None;

        for element in &self.list {

            if element.key == key {

                prev = element.prev;

                chave = 1;

                break;
            }
        }

        if chave == 0 {

            println!("Chave inexistente!");
            return None
        }

        match prev {

            Some(i) => return Some(&mut self.list[i].satelite),

            None => {
                println!("Não existe antecessor... chave selecionada é a cabeça da série");
                return None
            }
        }
    }

}

impl<T: Copy + PartialEq + Debug, E: Debug> LinkedList<T,E> {

    pub fn show(self: &Self) {

        let mut v = vec![];

        let mut pos = self.head;

        while let Some(i) = pos {

            v.push((self.list[i].key, &self.list[i].satelite));

            pos = self.list[i].next;

        }

        println!("{:?}", v);

    }

}





