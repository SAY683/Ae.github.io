///#[不安全数据]
#[macro_export]
macro_rules! nothing {
    ($a:expr)=>{
        ManuallyDrop::new($a)
    }
}
pub mod living_example {
//	///#[类型]
//	#[macro_export]
//	macro_rules! data_default {
//    ($tp:ty)=>{
//        Database::<$tp>::default()
//    }
//}
//	///#[链接]
//	/// ####
//	/// [x,x,x,x ...]-> [【x.(x1..) -> x1.(x,x) -> x.(x) ->...】] ->x.【】
//	/// ####
//	/// 参数最终被分配成 [2/3] arg 由此[再次]向上返回传递
//	#[macro_export]
//	macro_rules! links {
//    ($a:expr)=>{
//        $a
//    };
//    ($a:expr,$b:expr)=>{
//        Database::merge($a,$b)
//    };
//    ($a:expr,$b:expr,$c:expr)=>{
//        $c.merge($b.merge($a))
//    };
//    ($a:expr,$($b:tt)*)=>{
//        $a.merge(links!($($b)*))
//    };
//}
}
// #[macro_export]
// macro_rules! 区分
//  宏的参数目前有如下类型：
//
// item：Item，如函数定义，常量声明 等
// block：BlockExpression，如{ ... }
// stmt：Statement，如 let 表达式（传入为 stmt 类型的参数时不需要末尾的分号，但需要分号的 item 语句除外）
// pat：Pattern，模式匹配中的模式，如 Some(a)
// expr：Expression，表达式，如 Vec::new()
// ty：Type，类型，如 i32
// ident：IDENTIFIER_OR_KEYWORD，标识符或关键字，如 i 或 self
// path：TypePath，类型路径，如 std::result::Result
// tt：TokenTree，Token 树，被匹配的定界符 (、[] 或 {} 中的单个或多个 token
// meta：Attr，形如 #[...] 的属性中的内容
// lifetime：LIFETIME_TOKEN，生命周期 Token，如 'static
// vis：Visibility，可能为空的可见性限定符，如 pub
// literal：匹配 -? LiteralExpression
// 其中，tt 类型可以被视为 Rust 宏的 Any。
// item：任何标记
// block：任何标记
// stmt：=>、;、,
// pat：=>、,、=、|、if 或 in
// expr：=>、;、,
// ty：{、[、=>、,、>、=、:、;、|、as 或 where
// ident：任何标记
// path：{、[、=>、,、>、=、:、;、|、as 或 where
// meta：任何标记
// tt：任何标记