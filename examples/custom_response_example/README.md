# Ree Framework - 自定义响应类型示例

这个示例展示了如何在 Ree 框架中创建自定义响应类型并实现 `IntoResponse` trait。

## 🎯 核心特性

### 1. 自定义 API 响应结构体
```rust
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub code: u16,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        // 自动序列化为 JSON，设置正确的状态码和头部
    }
}
```

### 2. 分页响应结构体
```rust
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
    pub total: usize,
}

impl<T: Serialize> IntoResponse for PaginatedResponse<T> {
    fn into_response(self) -> Response {
        // 自动添加分页相关的头部信息
    }
}
```

### 3. 错误处理结构体
```rust
#[derive(Debug)]
pub enum AppError {
    NotFound,
    ValidationError(String),
    DatabaseError,
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 错误自动转换为标准 API 响应格式
    }
}
```

## 🏃‍♂️ 运行示例

```bash
cd examples/custom_response_example
cargo run
```

## 🧪 测试端点

### 成功响应示例
```bash
# 基本 API 响应
curl http://127.0.0.1:8080/

# 获取所有用户
curl http://127.0.0.1:8080/users

# 获取特定用户
curl http://127.0.0.1:8080/users/1

# 分页响应
curl http://127.0.0.1:8080/users/page/1
curl http://127.0.0.1:8080/users/page/2

# 统计数据
curl http://127.0.0.1:8080/stats

# 健康检查
curl http://127.0.0.1:8080/health
```

### 错误处理示例
```bash
# 资源不存在 (404)
curl http://127.0.0.1:8080/users/999

# 无效参数 (400)
curl http://127.0.0.1:8080/users/invalid

# 模拟数据库错误 (500)
curl http://127.0.0.1:8080/error

# 模拟资源不存在 (404)
curl http://127.0.0.1:8080/notfound
```

## 📋 响应格式示例

### 成功响应
```json
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com",
    "age": 25,
    "created_at": "2024-01-01T00:00:00Z"
  },
  "message": "Success",
  "timestamp": "2024-01-01T12:00:00Z",
  "code": 200
}
```

### 错误响应
```json
{
  "success": false,
  "data": null,
  "message": "Resource not found",
  "timestamp": "2024-01-01T12:00:00Z",
  "code": 404
}
```

### 分页响应
```json
{
  "items": [...],
  "pagination": {
    "page": 1,
    "page_size": 2,
    "total_pages": 2,
    "has_next": true,
    "has_prev": false
  },
  "total": 4
}
```

## 💡 设计亮点

1. **类型安全** - 编译时保证响应格式正确
2. **统一格式** - 所有 API 响应都有一致的结构
3. **自动转换** - 通过 `IntoResponse` 自动处理序列化和 HTTP 头部
4. **错误处理** - 错误类型也能自动转换为标准响应格式
5. **元数据支持** - 自动添加时间戳、状态码等元信息
6. **分页支持** - 内置分页响应结构，自动添加分页头部

## 🔧 扩展性

你可以轻松创建更多自定义响应类型：
- 文件下载响应
- 流式响应
- 缓存响应
- API 版本化响应
- 等等...

只需要实现 `IntoResponse` trait 即可！
