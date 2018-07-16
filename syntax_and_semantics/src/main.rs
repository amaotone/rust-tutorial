#[allow(unused_assignments, unused_variables)]

fn main() {
    // 関数
    {
        let x = 1;
        println!("x+1 = {}", add_one(x));
    }

    // 関数束縛
    {
        let f: fn(i32) -> i32 = add_one;
        let six = f(5);
        println!("{}", six);
    }

    // 配列
    {
        let a = [1, 2, 3, 4, 5];
        let b = [1; 20];
        println!("len(a)={}, len(b)={}", a.len(), b.len());
    }

    // スライス
    {
        let a = [0, 1, 2, 3];
        let complete = &a[..];
        let middle = &a[1..3];  // [a..b] => [a, b)
        println!("{}, {}", complete.len(), middle.len())
    }

    // タプル
    {
        let mut x = (1, 2);
        let y = (2, 3);
        x = y;  // アリティが同じなら代入可能
        println!("{}", x.0);  // コロンでアクセスする

        // let z = (3, "hello");  // 型が違っても良い
        // x = z;  // だめ

        let (a, b, c) = (1, 2, 3);
        println!("{}", a);
    }

    // if
    {
        let x = 5;
        let y = if x == 5 { x*2} else {x*3};
        println!("x={}, y={}", x, y);
    }

    // for
    {
        for (i, j) in (5..10).enumerate() {
            println!("i={}, j={}", i, j);
        }

        let a = ["hello", "world", "rust"];
        for s in &a {
            println!("{}", s);
        }

        // ループラベル
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x%2 == 0 { continue 'outer; }
                if y%2 == 0 { continue 'inner; }
                println!("x={}, y={}", x, y);
            }
        }
    }

    // 所有権
    {
        let v = vec![1, 2, 3];
        let v2 = v;
        println!("{}", v2[0]);
        // println!("{}", v[0]);  // これはできない

        let a = 1;
        let b = a;
        println!("a={}, b={}", a, b);  // プリミティブ型はCopyトレイトが実装されているためOK
    }

    // 借用
    {
        let v1 = vec![1, 2, 3];
        let v2 = vec![1, 2, 3, 4];
        println!("{}", add_vec_length(&v1, &v2));  // 参照を渡す
        println!("v1[0]={}, v2[0]={}", v1[0], v2[0]);  // 使える

        // &mut参照
        let mut x = 5;
        {
            let y = &mut x;
            *y += 1;
        }
        println!("{}", x);  // 6
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_vec_length(v1: &Vec<i32>, v2: &Vec<i32>) -> usize {
    v1.len() + v2.len()
}