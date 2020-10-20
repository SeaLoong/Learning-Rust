#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    /* 对任何类型 T 都有
     * &T 实现了 Copy<T> + Clone<T> + Deref<T> + Borrow<T> + Pointer<T>
     * &mut T 实现了 &T + DerefMut<T> + BorrowMut<T>
     * 假设有变量 x，则没有方法从 &x/&mut x 来获取 x 的真正意义上的"所有权"
     * 从 &x 只能获取 x.clone() 的 所有权，即其深拷贝的 所有权
     * 从 &mut x 可以获取其(浅拷贝)的 所有权，以下是几种方式
     * std::mem::take(dest: &mut T) -> T
     * std::mem::replace(dest: &mut T, src: T) -> T
     */
    /* ================================= 所有权与借用的转换 ================================= */
    let mut x: Vec<Vec<u8>> = vec![vec![1], vec![2]];
    {
        /* 从 不可变借用 获取 所有权 */
        let a = &x;
        // 不能通过解引用获取 所有权
        // let y = *a;
        // error[E0507]: cannot move out of `*a` which is behind a shared reference
        /* 总结
         * 无法通过 不可变借用 获取原内存的 所有权
         */
    }
    {
        /* 从 可变借用 获取 所有权 */
        let a = &mut x;
        // 记录 x 的内存地址
        let p1 = a as *const Vec<Vec<u8>>;
        // 记录 x 中的元素的内存地址
        let p11 = &mut a[0] as *const Vec<u8>;
        // 不能通过解引用获取 所有权
        // let y = *a;
        // error[E0507]: cannot move out of `*a` which is behind a mutable reference
        // std::mem::take 可行，std::mem::replace 可类似分析，以下不测试
        let mut y = std::mem::take(a);
        let b = &mut y;
        // 记录 y 的内存地址
        let p2 = b as *const Vec<Vec<u8>>;
        // 记录 y 中的元素的内存地址
        let p21 = &mut b[0] as *const Vec<u8>;
        // p1 != p2 因此实际上也是发生了拷贝，并且此时 x, y 都能使用
        assert_ne!(p1, p2);
        println!("x = {:?}, y = {:?}", x, y);
        // p11 == p21 结合上述可以知道实际上是发生了浅拷贝，由于 Rust 的设计，浅拷贝只能留存一份，否则会产生所有权的冲突
        assert_eq!(p11, p21);
        unsafe {
            println!("p1 = {:?}, p2 = {:?}", *p1, *p2);
            println!("p1 = {:?}, p2 = {:?}", *p11, *p21);
        }
        /* 总结
         * 无法通过 可变借用 获取原内存的 所有权
         * 可以通过 可变借用 获取其浅拷贝的所有权, 可以认为这是一种 (伪)所有权转移
         */
    }
    /* 由于 可变借用 只能从 可变绑定 中 获取，因此下列情况不会有包括这种情况的测试 */
    /* ================================= 所有权的借用 ================================= */
    let x: String = String::from("ooo");
    {
        /* 从 所有权 的 不可变绑定 中获取 不可变借用 的 不可变绑定 */
        let a: &String = &x;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 所有权 的 不可变绑定 中获取 不可变借用 的 可变绑定 */
        // 可以这样绑定，但是应当认为 mut 无效
        let mut a: &String = &x;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
    let mut y = x; // 所有权转移到可变绑定上
    {
        /* 从 所有权 的 可变绑定 中获取 不可变借用 的 不可变绑定 */
        let a: &String = &y;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 所有权 的 可变绑定 中获取 不可变借用 的 可变绑定 */
        let mut a: &String = &y;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 所有权 的 可变绑定 中获取 可变借用 的 不可变绑定 */
        let a: &mut String = &mut y;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 可以使用 可变方法
        a.push('1');
    }
    {
        /* 从 所有权 的 可变绑定 中获取 可变借用 的 可变绑定 */
        let mut a: &mut String = &mut y;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 可以使用 可变方法
        a.push('1');
    }
    /* 总结
     * 如果 所有权 转移到 不可变绑定 上，那么其 任意借用 都只能使用 不可变方法
     * 如果 所有权 转移到 可变绑定 上，那么其 不可变借用 只能使用 不可变方法，可变借用 能使用 不可变方法/可变方法
     */
    /* ================================= 不可变借用的借用 ================================= */
    {
        /* 从 不可变借用 的 不可变绑定 中获取 不可变借用 的 不可变绑定 */
        // 不可变借用 的 不可变绑定 x
        let x: &String = &String::from("bbb");
        // 不可变借用 的 不可变绑定 a
        let a: &&String = &x;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 不可变借用 的 不可变绑定 中获取 不可变借用 的 可变绑定 */
        // 不可变借用 的 不可变绑定 x
        let x: &String = &String::from("bbb");
        // 不可变借用 的 可变绑定 a
        let mut a: &&String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 不可变借用 的 可变绑定 中获取 不可变借用 的 不可变绑定 */
        // 不可变借用 的 可变绑定 x
        let mut x: &String = &String::from("bbb");
        // 不可变借用 的 不可变绑定 a
        let a: &&String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 不可变借用 的 可变绑定 中获取 不可变借用 的 可变绑定 */
        // 不可变借用 的 可变绑定 x
        let mut x: &String = &String::from("bbb");
        // 不可变借用 的 可变绑定 a
        let mut a: &&String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 不可变借用 的 可变绑定 中获取 可变借用 的 不可变绑定 */
        // 不可变借用 的 可变绑定 x
        let mut x: &String = &String::from("bbb");
        // 不可变借用 的 不可变绑定 a
        let a: &&String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 不可变借用 的 可变绑定 中获取 可变借用 的 可变绑定 */
        // 不可变借用 的 可变绑定 x
        let mut x: &String = &String::from("bbb");
        // 不可变借用 的 可变绑定 a
        let mut a: &&String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    /* 总结
     * 不可变借用 的 任意借用/任意绑定 都只能使用 不可变方法
     */
    /* ================================= 可变借用的借用 ================================= */
    {
        /* 从 可变借用 的 不可变绑定 中获取 不可变借用 的 不可变绑定 */
        // 可变借用 的 不可变绑定 x
        let x: &mut String = &mut String::from("bbb");
        // 不可变借用 的 不可变绑定 a
        let a: &&mut String = &x;
        // 可以使用 不可变方法
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 可变借用 的 不可变绑定 中获取 不可变借用 的 可变绑定 */
        // 可变借用 的 不可变绑定 x
        let x: &mut String = &mut String::from("bbb");
        // 不可变借用 的 可变绑定 a
        let mut a: &&mut String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 可变借用 的 可变绑定 中获取 不可变借用 的 不可变绑定 */
        // 可变借用 的 可变绑定 x
        let mut x: &mut String = &mut String::from("bbb");
        // 不可变借用 的 不可变绑定 a
        let a: &&mut String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 可变借用 的 可变绑定 中获取 不可变借用 的 可变绑定 */
        // 可变借用 的 可变绑定 x
        let mut x: &mut String = &mut String::from("bbb");
        // 不可变借用 的 可变绑定 a
        let mut a: &&mut String = &x;
        println!("{:?}", a);
        // 不能使用 可变方法
        // a.push('1');
        // error[E0596]: cannot borrow `**a` as mutable, as it is behind a `&` reference
    }
    {
        /* 从 可变借用 的 可变绑定 中获取 可变借用 的 不可变绑定 */
        // 可变借用 的 可变绑定 x
        let mut x: &mut String = &mut String::from("bbb");
        // 可变借用 的 不可变绑定 a
        let a: &mut &mut String = &mut x;
        println!("{:?}", a);
        // 可以使用 可变方法
        a.push('1');
    }
    {
        /* 从 可变借用 的 可变绑定 中获取 可变借用 的 可变绑定 */
        // 可变借用 的 可变绑定 x
        let mut x: &mut String = &mut String::from("bbb");
        // 可变借用 的 不可变绑定 a
        let mut a: &mut &mut String = &mut x;
        println!("{:?}", a);
        // 可以使用 可变方法
        a.push('1');
    }
    /* 总结
     * 可变借用 的 可变借用/任意绑定 都能使用 不可变方法/可变方法
     */
}
