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
