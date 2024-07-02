# Rust Crate —— Select 使用示例

## 一、总览
这个库定位是： 从 html 中提取有用的信息。

```rust
// 使用的基本流程是
// 1. 把 html 转换成一个 document， 这个 document 包含若干 Node （节点）
// 2. 定义 Node 选择器
// 3. 尝试找到指定的 Node，然后从中提取信息


use select::document::Document;
use select::predicate::{Attr, Predicate};

fn get_page_src(url: &str) -> String {
    !unimplemented!("TODO: 获取页面源码")
}

fn main() {
    let url = "https://huggingface.co/blog";
    let page = get_page_src(&url);

    // 从 &str 定义 Document
    let mydoc = Document::from(page.as_str());

    // 定义一个 Node 选择器
    let list_selector = Attr("data-target", "BlogThumbnails");

    // 尝试找到 Node 并提取信息
    // 对找到的所有结果，应用 map 中的闭包，提取节点中的文本
    // 然后把文本收集到一个 Vec 中
    let infos: Vec<_> = mydoc.find(list_selector).map(|n| n.text()).collect();

    println!("{:#?}", infos);
}
```


核心内容是定义选择器和提取信息。

## 二、定义选择器
选择器的定义，通过预定义的谓词（Predicate）实现。

在进入具体实例之前，假设我们有这样一段代码：

```rust
fn main() {
    println!("Hello，梦想成为软件工程师，正在找工作的闲从容。\nPS：软件工程师 != 码农");
}

pub fn get_page_src(url: &str) -> String {
    let _ = url;
    r#"
<div>
<h1>Hello, world!</h1>

<custom class="custom-element">
    <p data-target="test-attr">我是一个自定义元素</p>
    <a class="Microsoft" href="https://www.bing.com">Bing 一下</a>
</custom>

<another-custom class="custom-element">
    <div>
        <p>我是另一个自定义元素</p>
    </div>
</another-custom>
<a class="AlphaBeta" href="https://www.google.com">Google 一下</a>
<a class="git code github" href="https://www.github.com">Github 一下</a>
</div>
"#
    .to_string()
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    use select::document::Document;
    use select::predicate::{And, Attr, Child, Class, Descendant, Name, Not, Or, Predicate};

    fn get_mydoc() -> Document {
        let html = get_page_src("https://www.bing.com");
        let doc = Document::from(html.as_str());
        doc
    }
    
    // 下面将会包含若干个 测试
}
```


### 1. Name
name 根据标签名选择 Node。
```rust
    #[test]
    fn test_name_1() {
        let doc = get_mydoc();

        let selector = Name("h1");

        // 从第一个匹配项中提取文本
        let info = doc.find(selector).next().unwrap().text();

        assert_eq!(info, "Hello, world!".to_string());
    }

    #[test]
    fn test_name_2() {
        let doc = get_mydoc();

        let selector = Name("p");

        // 从所有匹配选项中提取文本, 以 Vec<String> 形式返回
        let info = doc.find(selector).map(|n| n.text()).collect::<Vec<_>>();

        assert_eq!(info[0], "我是一个自定义元素".to_string());
        assert_eq!(info[1], "我是另一个自定义元素".to_string());
    }
```

2. Class
class 根据 class 属性包含的值选择节点。
```rust
    #[test]
    fn test_class() {
        let doc = get_mydoc();
        let selector = Class("Microsoft");
        let url = doc.find(selector).next().unwrap().attr("href").unwrap();
        let text = doc.find(selector).next().unwrap().text();
        assert_eq!(url, "https://www.bing.com");
        assert_eq!(text, "Bing 一下");
    }

    #[test]
    fn test_class_with_space_split_values() {
        let doc = get_mydoc();

        // Class 选择器中只能有一个值 `git code` 是两个值了
        // 所以找不到任何值
        // Class("git") 的意思是， 某个 Node 的 class 包含 "git"
        let selector = Class("git code");
        assert!(doc.find(selector).next().is_none());

        let selector_git = Class("git");
        let text = doc.find(selector_git).next().unwrap();
        assert_eq!(text.text(), "Github 一下");
    }
```

3. Attr
通过 attr 选择含有自定义属性和特定属性值的节点。
```rust
    #[test]
    fn test_attr_1() {
        let doc = get_mydoc();
        let selector = Attr("data-target", "test-attr");
        let text = doc.find(selector).next().unwrap().text();
        assert_eq!(text, "我是一个自定义元素");
    }

    #[test]
    fn test_attr_2() {
        let doc = get_mydoc();

        // 某个 Node 的 class 属性值 == "git code github"
        let selector = Attr("class", "git code github");
        let url = doc.find(selector).next().unwrap().attr("href").unwrap();
        assert_eq!(url, "https://www.github.com");
    }
```

4. Child 和 Descendant
严格来说，Child 和 Descendant 和下面的 Add Or Not 都应该叫组合器。（想起来之前介绍的 Nom （闲从容：Rust库nom入门），也有类似的思想）。

Child(A, B)，A 是 B 的父亲，二者相邻。 A 和 B 可以是上面的 Name Class Attr 或者与任意组合器的合法组合。

Descendant(A, B)， A 是 B 的任意级别祖先。
```rust
    #[test]
    fn test_child() {
        let doc = get_mydoc();

        // div 标签下面的 p 标签
        // div 是 p 的父亲
        let selector = Child(Name("div"), Name("p"));

        let text = doc.find(selector).map(|n| n.text()).collect::<Vec<_>>();

        assert_eq!(text, vec!["我是另一个自定义元素"]);
    }

    #[test]
    fn test_descendant() {
        let doc = get_mydoc();

        // div 标签下面的 p 标签
        // div 是 p 的任意级别祖先
        let selector = Descendant(Name("div"), Name("p"));
        let text = doc.find(selector).map(|n| n.text()).collect::<Vec<_>>();

        assert_eq!(text, vec!["我是一个自定义元素", "我是另一个自定义元素"]);
    }
```

5. Add、Or 和 Not
见名知意。

```rust
    #[test]
    fn test_and() {
        let doc = get_mydoc();
        let selector = And(Name("p"), Attr("data-target", "test-attr"));

        let text = doc.find(selector).next().unwrap().text();
        assert_eq!(text, "我是一个自定义元素");
    }

    #[test]
    fn test_and_link() {
        let doc = get_mydoc();

        // And Or Not Child Descendant 都可以通过链式调用
        // 只需要将首字母改为小写即可
        // 这样调用需要导入 select::predicate::Predicate 这个 trait
        // 相对于用构建 struct 的方式调用， 链式调用更加优雅，而且可以无限向后
        // 最重要的是， 也更符合语意
        let selecotr = Name("p").and(Attr("data-target", "test-attr"));

        let text = doc.find(selecotr).next().unwrap().text();
        assert_eq!(text, "我是一个自定义元素");
    }

    #[test]
    fn test_or() {
        let doc = get_mydoc();

        // 选择
        // a 标签
        // 或
        // 含有 data-target 属性值为 test-attr 的 p 标签
        let target_p = Name("p").and(Attr("data-target", "test-attr"));
        let selector = Or(target_p, Name("a"));
        let texts = doc.find(selector).map(|n| n.text()).collect::<Vec<_>>();

        assert_eq!(
            texts,
            vec![
                "我是一个自定义元素",
                "Bing 一下",
                "Google 一下",
                "Github 一下",
            ]
        )
    }

    #[test]
    fn test_not() {
        let doc = get_mydoc();

        // 选择
        // p 标签
        // 且
        // daata-target 属性值不等于 test-attr
        // 的节点
        let selector = And(Name("p"), Not(Attr("data-target", "test-attr")));

        let text = doc.find(selector).next().unwrap().text();
        assert_eq!(text, "我是另一个自定义元素");
    }
```

三、提取信息
在 html 文件中，有用的信息大致可以分为两类： 标签属性值和标签内容。

标签属性值可以通过 `.attr(<标签>)` 提取， 标签内容可以直接通过 `.text()` 提取。

四、结果
上面 12 个测试用例的结果是全部通过。

```shell
running 12 tests
test tests::test_attr_2 ... ok
test tests::test_class ... ok
test tests::test_and_link ... ok
test tests::test_not ... ok
test tests::test_and ... ok
test tests::test_name_2 ... ok
test tests::test_child ... ok
test tests::test_attr_1 ... ok
test tests::test_or ... ok
test tests::test_class_with_space_split_values ... ok
test tests::test_name_1 ... ok
test tests::test_descendant ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


五、延伸阅读
Github： https://github.com/utkarshkukreti/select.rs
官方Example： https://github.com/utkarshkukreti/select.rs/tree/master/examples
