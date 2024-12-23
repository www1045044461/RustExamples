use hyper::Method;
use reqwest::cookie::Jar;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::{copy, Write};
use std::sync::Arc;
use tokio;

/// 第一重跳转的节点
///
fn use_class_selector(html: &Html) -> Result<String, String> {
    // 使用 CSS 选择器查找登录按钮
    let login_button_selector_nt = Selector::parse(".scarabLink");

    if let Ok(result) = login_button_selector_nt {
        let mut jump_url = String::new();
        let mut target_id = 0;

        // 查找按钮并打印
        for button in html.select(&result) {
            // 获取按钮的文本内容或属性（例如 id, class 等）
            println!(
                "Found login button: {:?}",
                button.text().collect::<Vec<_>>()
            );
            // 可以获取其他属性，如 href
            if let Some(href) = button.value().attr("href") {
                println!("Button href: {}", href);
                jump_url = href.to_string();
                target_id += 1;
                break;
            }
        }
        if target_id == 1 {
            Result::Ok(jump_url)
        } else {
            Err(String::from("没有找到目标类型"))
        }
    } else {
        Err(String::from("跳转链接获取失败"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个 Cookie Jar
    let var_name = Arc::new(Jar::default());
    let cookie_jar = var_name;

    // 创建一个带有 Cookies 和 HTTPS 支持的 Client
    let client = Client::builder()
        .cookie_store(true) // 启用 Cookies 管理
        .cookie_provider(cookie_jar.clone()) // 设置 Cookie Jar
        .build()?;

    // 发送 HTTPS 请求
    let url = Url::parse("https://marketplace.visualstudio.com/items?itemName=vscodevim.vim")?;

    let res = client.get(url.clone()).send().await?;

    //打印响应体
    let body = res.text().await.expect("No parsing content!");

    // println!("body:{}", &body);

    let mut file = File::create("./1.html").expect("Create File failed!");
    file.write_all(&body.as_bytes())
        .expect("write content failed!");

    let html_doc = Html::parse_document(&body);

    let next_url = use_class_selector(&html_doc);

    if let Ok(url) = next_url {
        let respond = client.get(url).send().await?;
        let body = respond.text().await.expect("No parsing content!");
        // println!("New Body:{}", &body);

        let mut file2 = File::create_new("./login.html").expect("第二階段打開失敗");

        file2
            .write_all(&body.as_bytes())
            .expect("Write html level2 error!");
    }

    let cookie = "session=abcd1234; path=/;";
    cookie_jar.add_cookie_str(cookie, &url);

    Ok(())
}
