# rust-interpreter
interpreter learning

Rust没有null，使用Option枚举类替代，Option的最方便，最好的使用方法还没有找到，每次都用match来进行匹配感觉比较麻烦，全都使用unwrap的话又不好.

指定变量的lifetime相当于是指定变量的存货时间，一般只在引用类型的变量中使用，这一块比较难，还需看看.

```rs
while let Some(c) = optional {

}
```

需要记住lifetime的语法，定义时候书写的顺序

```rs
pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}
```

enum类型的变量比较相等，可能需要derive PartialEq

读引用可以有多个，mut引用只能同时有一个，经过修改后的变量的读引用需要重新赋值，或重新获取