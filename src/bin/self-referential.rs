use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            data,
            // 只有在数据到位时，才创建指针，否则数据会在开始之前就被转移所有权
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        
        // 这里其实安全的，因为修改一个字段不会转移整个结构体的所有权
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }
}

fn main() {
    let unmoved = Unmovable::new("hello".to_string());

    let mut still_unmoved = unmoved;
    assert_eq!(still_unmoved.slice, NonNull::from(&still_unmoved.data));
}

// 只要结构体没有被转移，那么指向该结构体的指针，在初始化之后就一定会指向正确的值，即便这个指针的所有权被随便转移。
// 比如，如果该结构体的地址为100，被绑定给某个变量x，那么只要结构体的地址维持100，不论x转移给y，又转移给z，依旧能通过这变量访问到指针100的结构体