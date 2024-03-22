# rust 版本web框架gee实现

## day01 Http基础

- go中gee封装好像比rust简单 rust这块主要涉及到异步的问题？

## 参考hyper-router

> [!NOTE]
> 没整出来 参考了这个库 <https://github.com/marad/hyper-router> ? 不对 hyper版本太低

- 原因主要是老版本hyper的body trait 和我这最新版的不一样

## 参考salvo

- 还是参考salvo吧 hh

> [!NOTE]
> salvo 国人朽木大佬做的rust框架 <https://github.com/salvo-rs/salvo> 使用起来比个人感觉比axum更简单一些 也是对hyper进行封装

- 仔细看了下 salvo中对hyper的request response 等等都实现了 From 的trait

```rust
    pub fn from_hyper<B>(req: hyper::Request<B>, scheme: Scheme) -> Self
    where
        B: Into<ReqBody>,

```

- 我们ree中暂时不需要像大佬一样这么复杂吧？

## 实现

1. 第一步 实现这个type 对hyper的封装

```go
// HandlerFunc defines the request handler used by gee
type HandlerFunc func(http.ResponseWriter, *http.Request)
```

- 朽木大佬好像是对handler也重写了 Orz

```rust

pub type Handler =
    fn(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

```

- 暂时这样定义 并不通用

2. 第二步 整一个结构体 里面放个router的类似的东西

- 想着自己实现一个handler 然后在handler里直接返回service_fn的结果 然后提示hyper中这个结果的结构体是私有的 Orz

```rust

async fn handler(req: Request<hyper::body::Incoming>, engine: &mut Engine) {
    if let Some(f) = engine.routers.iter().find_map(|(method, path, handler)| {
        if req.method() == *method && req.uri().path() == path {
            Some(handler)
        } else {
            None
        }
    }) {
        // 就是这里这个f是私有的
        let f = service_fn(f).await;
    } else {
        Ok(Response::builder().status(404).body(empty()).unwrap());
    };
}

```

- g 自己实现不了呢 参考朽木大佬吧
