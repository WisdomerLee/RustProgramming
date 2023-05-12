/*
items in range
: 물건들의 값이 벡터로 제공되고 제공되는 가격의 아이템을 찾기
: binary search tree, box pointer를 활용하기

binary search tree
이진 트리

 */
struct BinarySearchTree{
    root: Node,
}


struct Node{
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node{
    fn new(value: i32) -> Self{
        Node{
            val: value,
            left: None,
            right: None
        }
    }
    fn insert(&mut self, value: i32){

        if value > self.val{
            match self.right{
                None => self.right = Some(Box::new(Node{val:value, left: None, right: None})),
                Some(ref mut node) => node.insert(value),
            }
        } else{
            match self.left{
                None => self.left = Some(Box::new(Node{val:value, left: None, right: None})),
                Some(ref mut node) => node.insert(value),
            }
        }

    }
}

fn main(
    let product_prices = vec![9,6,14,20,1,30,8,17,5];

    let mut bst = BinarySearchTree{
        root: Node::new(product_prices[0])
    };

    for i in 1..product_prices.len(){
        bst.root.insert(product_prices[i]);
    }
)