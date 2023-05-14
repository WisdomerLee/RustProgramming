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

//아래의 structure는 clone 할 수 있음
#[derive(Clone)]
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

fn traversal(node: Option<Box<Node>>, low: i32, high: i32, mut output: &mut Vec<i32>){
    if node.as_ref().unwrap().val >= low &&node.as_ref().unwrap().val <= high{
        //reference의 값을 얻어서 값을 복사해서 넣기
        output.push(node.as_ref().unwrap().val);
    }
    if node.as_ref().unwrap().val >= low{
        //reference로 얻어 그대로 넣지 않고 clone으로 처리한 이유?? > 검색을 했는데 원본 값이 바뀌면 안 되므로...
        traversal(node.as_ref().unwrap().left.clone(), low, high, &mut output);
    }
    if node.as_ref().unwrap().val <= high{
        traversal(node.as_ref().unwrap().right.clone(), low, high, &mut output);
    }
}

fn productsInRange(root: Node, low:i32, high: i32) -> Vec<i32>{
    let mut output: Vec<i32> = Vec::new();
    traversal(Some(Box::new(root)), low, high, &mut output);
    output
}

fn main(){
    let product_prices = vec![9,6,14,20,1,30,8,17,5];

    let mut bst = BinarySearchTree{
        root: Node::new(product_prices[0])
    };

    for i in 1..product_prices.len(){
        bst.root.insert(product_prices[i]);
    }

    let result = productsInRange(bst.root, 7, 20);
    println!("{:?}", result);
}