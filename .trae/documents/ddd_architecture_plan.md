# AxumKit DDD 架构改造方案

## 1. 仓库分析

通过分析当前 AxumKit 项目结构，发现其采用传统的分层架构：
- API 层 (src/api/)
- 服务层 (src/service/)
- 仓库层 (src/repository/)
- 实体层 (src/entity/)

主要功能模块包括：
- 认证 (auth)
- 用户 (user)
- OAuth (oauth)
- 搜索 (search)
- 动作日志 (action_logs)
- 审核 (moderation)
- 事件流 (eventstream)
- 健康检查 (health)

当前项目已将所有 `axumkit-<module>` 类型的 crate 重命名为 `kit-<module>`，包括：
- `kit-dto`
- `kit-entity`
- `kit-server`
- `kit-config`
- `kit-constants`
- `kit-errors`
- `kit-worker`

## 2. DDD 架构改造方案

### 2.1 核心领域划分

根据业务功能和领域边界，将项目拆分为以下独立领域模块：

| 领域模块 | 描述 | 核心功能 |
|---------|------|----------|
| `kit-auth` | 认证领域 | 登录、注销、注册、密码管理、TOTP 2FA |
| `kit-user` | 用户领域 | 用户资料管理、角色管理、用户封禁 |
| `kit-oauth` | 第三方认证领域 | 谷歌、GitHub 登录和关联 |
| `kit-search` | 搜索领域 | 用户搜索功能 |
| `kit-action-log` | 动作日志领域 | 记录系统动作和事件 |
| `kit-moderation` | 审核领域 | 内容审核和管理 |
| `kit-health` | 健康检查领域 | 系统健康状态检查 |

### 2.2 领域模块结构

每个领域模块采用以下结构：

```
crate/
└── kit-<domain>/
    ├── src/
    │   ├── domain/        # 领域模型和业务规则
    │   │   ├── model/      # 实体、值对象
    │   │   ├── service/    # 领域服务
    │   │   └── repository/ # 领域仓库接口
    │   ├── application/    # 应用服务
    │   ├── api/            # API 层
    │   │   ├── handler/    # 请求处理器
    │   │   ├── route/      # 路由定义
    │   │   └── dto/        # 请求/响应数据结构
    │   ├── infrastructure/ # 基础设施
    │   │   ├── repository/ # 仓库实现
    │   │   ├── config/     # 领域配置
    │   │   └── adapter/    # 外部系统适配器
    │   └── lib.rs          # 模块入口
    └── Cargo.toml
```

### 2.3 共享模块

保留以下共享模块：

| 共享模块 | 描述 |
|---------|------|
| `kit-config` | 全局配置管理 |
| `kit-constants` | 共享常量定义 |
| `kit-errors` | 错误处理系统 |
| `kit-worker` | 后台任务处理 |

## 3. 领域模块详细设计

### 3.1 Auth 领域

**核心概念**：
- 用户认证
- 会话管理
- 密码管理
- TOTP 2FA

**模块结构**：
```
kit-auth/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   ├── user.rs        # 用户实体
│   │   │   ├── session.rs      # 会话实体
│   │   │   └── totp.rs         # TOTP 相关值对象
│   │   ├── service/
│   │   │   ├── auth_service.rs     # 认证领域服务
│   │   │   └── session_service.rs  # 会话管理服务
│   │   └── repository/
│   │       ├── user_repository.rs  # 用户仓库接口
│   │       └── session_repository.rs # 会话仓库接口
│   ├── application/
│   │   ├── login_service.rs     # 登录应用服务
│   │   ├── signup_service.rs    # 注册应用服务
│   │   ├── password_service.rs  # 密码管理服务
│   │   └── totp_service.rs      # TOTP 应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   ├── login.rs         # 登录处理器
│   │   │   ├── logout.rs        # 注销处理器
│   │   │   ├── signup.rs        # 注册处理器
│   │   │   └── totp/
│   │   ├── route/
│   │   │   └── auth_routes.rs   # 认证路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   ├── user_repository_impl.rs  # 用户仓库实现
│   │   │   └── session_repository_impl.rs # 会话仓库实现
│   │   └── config/
│   │       └── auth_config.rs   # 认证配置
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.2 User 领域

**核心概念**：
- 用户资料
- 用户角色
- 用户封禁
- 用户图片管理

**模块结构**：
```
kit-user/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   ├── user.rs          # 用户实体
│   │   │   ├── user_role.rs      # 用户角色实体
│   │   │   └── user_ban.rs       # 用户封禁实体
│   │   ├── service/
│   │   │   ├── profile_service.rs     # 资料管理服务
│   │   │   └── user_management_service.rs # 用户管理服务
│   │   └── repository/
│   │       ├── user_repository.rs      # 用户仓库接口
│   │       ├── user_role_repository.rs # 用户角色仓库接口
│   │       └── user_ban_repository.rs  # 用户封禁仓库接口
│   ├── application/
│   │   ├── profile_service.rs     # 资料管理应用服务
│   │   └── user_management_service.rs # 用户管理应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   ├── profile.rs        # 资料处理器
│   │   │   └── management/
│   │   ├── route/
│   │   │   └── user_routes.rs    # 用户路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   ├── user_repository_impl.rs      # 用户仓库实现
│   │   │   ├── user_role_repository_impl.rs # 用户角色仓库实现
│   │   │   └── user_ban_repository_impl.rs  # 用户封禁仓库实现
│   │   └── config/
│   │       └── user_config.rs     # 用户配置
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.3 OAuth 领域

**核心概念**：
- 第三方认证
- OAuth 连接管理
- 提供商集成

**模块结构**：
```
kit-oauth/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   ├── oauth_connection.rs  # OAuth 连接实体
│   │   │   └── oauth_provider.rs    # OAuth 提供商实体
│   │   ├── service/
│   │   │   └── oauth_service.rs     # OAuth 领域服务
│   │   └── repository/
│   │       └── oauth_repository.rs  # OAuth 仓库接口
│   ├── application/
│   │   ├── google_service.rs    # 谷歌 OAuth 服务
│   │   ├── github_service.rs    # GitHub OAuth 服务
│   │   └── oauth_service.rs     # OAuth 应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   ├── google.rs        # 谷歌 OAuth 处理器
│   │   │   ├── github.rs        # GitHub OAuth 处理器
│   │   │   └── connection.rs    # 连接管理处理器
│   │   ├── route/
│   │   │   └── oauth_routes.rs  # OAuth 路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   └── oauth_repository_impl.rs  # OAuth 仓库实现
│   │   ├── adapter/
│   │   │   ├── google_adapter.rs  # 谷歌适配器
│   │   │   └── github_adapter.rs  # GitHub 适配器
│   │   └── config/
│   │       └── oauth_config.rs   # OAuth 配置
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.4 Search 领域

**核心概念**：
- 全文搜索
- 索引管理
- 搜索结果

**模块结构**：
```
kit-search/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   └── search_result.rs  # 搜索结果实体
│   │   ├── service/
│   │   │   └── search_service.rs # 搜索领域服务
│   │   └── repository/
│   │       └── search_repository.rs # 搜索仓库接口
│   ├── application/
│   │   └── search_service.rs     # 搜索应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   └── search.rs         # 搜索处理器
│   │   ├── route/
│   │   │   └── search_routes.rs  # 搜索路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   └── search_repository_impl.rs # 搜索仓库实现
│   │   └── adapter/
│   │       └── meilisearch_adapter.rs # MeiliSearch 适配器
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.5 Action Log 领域

**核心概念**：
- 系统动作记录
- 事件流
- 动作查询

**模块结构**：
```
kit-action-log/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   └── action_log.rs    # 动作日志实体
│   │   ├── service/
│   │   │   └── action_log_service.rs # 动作日志领域服务
│   │   └── repository/
│   │       └── action_log_repository.rs # 动作日志仓库接口
│   ├── application/
│   │   └── action_log_service.rs     # 动作日志应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   └── action_log.rs     # 动作日志处理器
│   │   ├── route/
│   │   │   └── action_log_routes.rs # 动作日志路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   └── action_log_repository_impl.rs # 动作日志仓库实现
│   │   └── adapter/
│   │       └── eventstream_adapter.rs # 事件流适配器
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.6 Moderation 领域

**核心概念**：
- 内容审核
- 审核日志
- 审核规则

**模块结构**：
```
kit-moderation/
├── src/
│   ├── domain/
│   │   ├── model/
│   │   │   └── moderation_log.rs # 审核日志实体
│   │   ├── service/
│   │   │   └── moderation_service.rs # 审核领域服务
│   │   └── repository/
│   │       └── moderation_repository.rs # 审核仓库接口
│   ├── application/
│   │   └── moderation_service.rs     # 审核应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   └── moderation.rs     # 审核处理器
│   │   ├── route/
│   │   │   └── moderation_routes.rs # 审核路由
│   │   └── dto/
│   │       ├── request/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   └── moderation_repository_impl.rs # 审核仓库实现
│   │   └── config/
│   │       └── moderation_config.rs # 审核配置
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

### 3.7 Health 领域

**核心概念**：
- 系统健康检查
- 服务状态监控

**模块结构**：
```
kit-health/
├── src/
│   ├── domain/
│   │   ├── service/
│   │   │   └── health_service.rs # 健康检查领域服务
│   │   └── repository/
│   │       └── health_repository.rs # 健康检查仓库接口
│   ├── application/
│   │   └── health_service.rs     # 健康检查应用服务
│   ├── api/
│   │   ├── handler/
│   │   │   └── health_check.rs   # 健康检查处理器
│   │   ├── route/
│   │   │   └── health_routes.rs  # 健康检查路由
│   │   └── dto/
│   │       └── response/
│   ├── infrastructure/
│   │   ├── repository/
│   │   │   └── health_repository_impl.rs # 健康检查仓库实现
│   │   └── adapter/
│   │       └── service_checker.rs # 服务检查适配器
│   └── lib.rs          # 模块入口
└── Cargo.toml
```

## 4. 依赖关系

### 4.1 领域间依赖

| 领域 | 依赖领域 | 依赖类型 |
|------|---------|----------|
| `kit-auth` | `kit-user` | 强依赖 |
| `kit-oauth` | `kit-auth`, `kit-user` | 强依赖 |
| `kit-user` | - | 无依赖 |
| `kit-search` | `kit-user` | 弱依赖 |
| `kit-action-log` | - | 无依赖 |
| `kit-moderation` | `kit-user` | 弱依赖 |
| `kit-health` | - | 无依赖 |

### 4.2 共享模块依赖

所有领域模块都依赖于以下共享模块：
- `kit-config`
- `kit-constants`
- `kit-errors`

## 5. 改造步骤

1. **创建领域模块目录结构**
   - 为每个领域创建独立的 crate
   - 建立领域内部的目录结构

2. **迁移领域模型**
   - 将实体从 `kit-entity` 迁移到对应领域
   - 定义领域特定的业务规则

3. **实现领域服务**
   - 实现领域服务和仓库接口
   - 实现基础设施层的仓库实现

4. **实现应用服务**
   - 实现应用服务，协调领域服务
   - 处理业务用例

5. **实现 API 层**
   - 实现请求处理器
   - 定义路由
   - 创建请求/响应 DTO

6. **集成测试**
   - 为每个领域编写集成测试
   - 确保领域间的协作正常

7. **重构主应用**
   - 更新 `kit-server` 以使用新的领域模块
   - 移除旧的分层结构

## 6. 优势

1. **领域边界清晰**：每个领域模块独立管理自己的业务逻辑
2. **可维护性提高**：代码组织更加清晰，便于理解和维护
3. **可测试性增强**：领域模块可以独立测试
4. **扩展性更好**：新功能可以在对应领域中添加，不影响其他领域
5. **团队协作效率提升**：不同团队可以负责不同领域的开发

## 7. 风险和注意事项

1. **依赖管理**：需要仔细管理领域间的依赖关系，避免循环依赖
2. **性能影响**：模块化可能会增加一些运行时开销，需要监控和优化
3. **迁移复杂性**：从现有架构迁移到 DDD 架构需要仔细规划
4. **学习成本**：团队需要理解 DDD 概念和实践

## 8. 总结

通过 DDD 架构改造，AxumKit 项目将获得更清晰的领域边界、更好的可维护性和可扩展性。每个领域模块将包含自己的领域模型、服务、API 处理程序和路由，实现真正的业务模块化。