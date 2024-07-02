fn main() {
    println!("Hello, world!");
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
}
