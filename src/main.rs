use chrono::Local;
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, time::Duration};
use tokio::time;

#[derive(Serialize, Deserialize)]
struct TaskConfig {
    url: String,
    method: String,                           // HTTP 方法
    headers: Option<HashMap<String, String>>, // 可选 Headers
    body: Option<String>,                     // 可选 Body，主要用于 POST、PUT 请求
    interval_seconds: u64,                    // 定时周期
}

// 定时任务的逻辑
async fn run_task(client: Client, task: TaskConfig) {
    let interval_duration = Duration::from_secs(task.interval_seconds);

    let mut interval = time::interval(interval_duration);
    loop {
        interval.tick().await;

        let response = send_request(&client, &task).await;

        match response {
            Ok(response) => {
                let status = response.status();
                let log_message = format!(
                    "{} - Successfully called URL: {}, Method: {}, Status: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    task.url,
                    task.method,
                    status
                );
                println!("{}", log_message); // 打印日志到控制台
            }
            Err(error) => {
                let log_message = format!(
                    "{} - Failed to call URL: {}, Method: {}, Error: {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    task.url,
                    task.method,
                    error
                );
                println!("{}", log_message); // 打印日志到控制台
            }
        }
    }
}

// 发送 HTTP 请求的逻辑
async fn send_request(client: &Client, task: &TaskConfig) -> Result<Response, reqwest::Error> {
    // 动态选择 HTTP 方法
    let method = match task.method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => {
            eprintln!(
                "Unsupported HTTP method: {}, defaulting to GET",
                task.method
            );
            Method::GET
        }
    };

    // 构建请求
    let mut request = client.request(method, &task.url);

    // 添加 Headers
    if let Some(headers) = &task.headers {
        for (key, value) in headers {
            request = request.header(key, value);
        }
    }

    // 添加 Body (仅在 POST 或 PUT 方法下有效)
    if let Some(body) = &task.body {
        request = request.body(body.clone());
    }

    // 发送请求并返回结果
    request.send().await
}

#[tokio::main]
async fn main() {
    let config_data = fs::read_to_string("tasks.json").expect("Failed to read tasks.json");
    let tasks: Vec<TaskConfig> = serde_json::from_str(&config_data).expect("Invalid tasks.json");

    // 创建 HTTP Client
    let client = Client::new();

    // 并发地运行所有定时任务
    let mut handles = vec![];
    for task in tasks {
        let client = client.clone();
        handles.push(tokio::spawn(run_task(client, task)));
    }

    for handle in handles {
        let _ = handle.await;
    }
}
