# Advanced Features Example

这个示例展示了 Ree HTTP 框架的高级功能和特性。

## 功能展示

### 🌟 核心特性
- **路由系统**: 基于Trie的高效路由匹配
- **中间件链**: 灵活的中间件系统
- **路由组**: 支持路由分组和组级中间件
- **多种响应类型**: JSON, HTML, Text等
- **错误处理**: 标准化的错误响应

### 🔧 中间件示例
- **访问日志**: 记录请求信息和响应时间
- **CORS**: 跨域资源共享支持
- **自定义日志**: 带应用名称的详细日志
- **限流**: 请求频率限制
- **认证**: Token验证中间件

### 🎯 API 设计
- **RESTful API**: 标准的REST API设计
- **统一响应格式**: 包含success、data、message字段
- **参数验证**: 路径参数验证和错误处理
- **状态码**: 正确的HTTP状态码使用

## 运行示例

```bash
cd examples/advanced_features
cargo run
```

或从项目根目录：

```bash
cargo run --example advanced_features
```

## 测试端点

### 基础页面
- http://127.0.0.1:3000/ - 首页 (HTML)
- http://127.0.0.1:3000/health - 健康检查 (JSON)
- http://127.0.0.1:3000/about - 关于页面 (Text)
- http://127.0.0.1:3000/docs - API文档 (JSON)

### API v1 端点
- `GET` http://127.0.0.1:3000/api/v1/users - 用户列表
- `POST` http://127.0.0.1:3000/api/v1/users - 创建用户
- `GET` http://127.0.0.1:3000/api/v1/users/1 - 获取用户详情
- `PUT` http://127.0.0.1:3000/api/v1/users/1 - 更新用户
- `DELETE` http://127.0.0.1:3000/api/v1/users/1 - 删除用户

### 管理员端点 (需要认证)
- `GET` http://127.0.0.1:3000/admin/stats - 系统统计
- `GET` http://127.0.0.1:3000/admin/logs - 系统日志

**认证方式**: 在请求头中添加 `Authorization: Bearer admin-secret-token`

## 测试命令

```bash
# 基础测试
curl http://127.0.0.1:3000/health

# 获取用户列表
curl http://127.0.0.1:3000/api/v1/users

# 获取特定用户
curl http://127.0.0.1:3000/api/v1/users/1

# 管理员端点 (需要认证)
curl -H "Authorization: Bearer admin-secret-token" http://127.0.0.1:3000/admin/stats

# 无认证访问管理员端点 (应返回401)
curl http://127.0.0.1:3000/admin/stats
```

## 代码结构

```
src/
├── main.rs        # 主入口，配置应用和路由
├── handlers.rs    # 业务逻辑处理器
└── middleware.rs  # 自定义中间件实现
```

## 学习要点

1. **中间件设计**: 如何创建和使用自定义中间件
2. **错误处理**: 统一的错误响应格式
3. **参数验证**: 路径参数的解析和验证
4. **认证授权**: 基于Token的简单认证
5. **API设计**: RESTful API的最佳实践
6. **响应格式**: 不同类型响应的构建方法
