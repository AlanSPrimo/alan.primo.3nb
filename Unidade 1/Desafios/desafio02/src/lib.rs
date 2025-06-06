// Testes Simples para Implementação de Árvore Binária de Busca (BST)
// Os alunos devem implementar a árvore para fazer estes testes passarem

#[cfg(test)]
mod bst_tests {
    // Importe sua implementação de BST aqui
    use crate::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    fn new() -> Self {
        BST { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                Some(node) => {
                    if value < node.value {
                        current = &mut node.left;
                    } else if value > node.value {
                        current = &mut node.right;
                    } else {
                        return; // valor duplicado
                    }
                }
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if value == node.value {
                return true;
            } else if value < node.value {
                current = &node.left;
            } else {
                current = &node.right;
            }
        }
        false
    }
}