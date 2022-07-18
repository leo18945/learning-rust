use crate::leotest::practise_trait_1::model1::User;
use crate::leotest::practise_trait_1::model1::To_String;

//实现trait的位置真的没关系吗？只要在全局能找得到这个实现，就可以调用它的方法吗？
impl To_String for User {
    fn to_string(&self) -> String {
        format!("{:?}", self)

        // &format!("{:?}", self)
        // rust cannot return reference to temporary value
        // 这里为什么不能返回引用？我认为是如果返回引用的话相当于3层外的第4层指针，但出了这个作用域，那个3层内的数据都销毁了，光指针用不了

        // "{id: ".to_string().push_str(self.id.to_string())
    }
}