# 共享模块重命名计划

## 1. 仓库分析

通过分析当前 AxumKit 项目结构，发现以下共享模块需要重命名：

| 当前模块名 | 目标模块名 | 描述 |
|-----------|-----------|------|
| `axumkit-config` | `kit-config` | 全局配置管理 |
| `axumkit-constants` | `kit-constants` | 共享常量定义 |
| `axumkit-errors` | `kit-errors` | 错误处理系统 |
| `axumkit-worker` | `kit-worker` | 后台任务处理 |

此外，需要创建一个新的 `kit-core` 模块，用于存放核心功能和工具。

## 2. 重命名计划

### 2.1 重命名步骤

1. **创建 kit-core 模块**
   - 创建 `crates/kit-core` 目录结构
   - 从 `axumkit-server/src/utils` 中迁移核心工具函数
   - 配置 `Cargo.toml` 文件

2. **重命名 axumkit-config 为 kit-config**
   - 重命名目录：`crates/axumkit-config` → `crates/kit-config`
   - 更新 `Cargo.toml` 中的包名
   - 更新模块内的引用路径

3. **重命名 axumkit-constants 为 kit-constants**
   - 重命名目录：`crates/axumkit-constants` → `crates/kit-constants`
   - 更新 `Cargo.toml` 中的包名
   - 更新模块内的引用路径

4. **重命名 axumkit-errors 为 kit-errors**
   - 重命名目录：`crates/axumkit-errors` → `crates/kit-errors`
   - 更新 `Cargo.toml` 中的包名
   - 更新模块内的引用路径

5. **重命名 axumkit-worker 为 kit-worker**
   - 重命名目录：`crates/axumkit-worker` → `crates/kit-worker`
   - 更新 `Cargo.toml` 中的包名
   - 更新模块内的引用路径

6. **更新依赖关系**
   - 更新所有模块的 `Cargo.toml` 文件中的依赖引用
   - 更新代码中的 `use` 语句

7. **测试和验证**
   - 运行 `cargo check` 检查编译错误
   - 运行 `cargo test` 验证功能正常

### 2.2 具体文件修改

#### 2.2.1 创建 kit-core 模块

**文件结构**：
```
crates/kit-core/
├── src/
│   ├── crypto/
│   │   ├── backup_code.rs
│   │   ├── mod.rs
│   │   ├── password.rs
│   │   └── token.rs
│   ├── extract/
│   │   ├── extract_ip_address.rs
│   │   ├── extract_user_agent.rs
│   │   └── mod.rs
│   ├── image_processor/
│   │   ├── image_processor.rs
│   │   ├── image_validator.rs
│   │   └── mod.rs
│   ├── diff.rs
│   ├── logger.rs
│   ├── mod.rs
│   ├── r2_url.rs
│   ├── redis_cache.rs
│   ├── redis_keys.rs
│   ├── session_helper.rs
│   └── uuid.rs
└── Cargo.toml
```

**Cargo.toml**：
```toml
[package]
name = "kit-core"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.5", features = ["v4", "serde"] }
argon2 = "0.5"
rand = "0.8"
base64 = "0.21"
image = "0.24"
```

#### 2.2.2 重命名模块目录

| 原目录 | 目标目录 |
|--------|----------|
| `crates/axumkit-config` | `crates/kit-config` |
| `crates/axumkit-constants` | `crates/kit-constants` |
| `crates/axumkit-errors` | `crates/kit-errors` |
| `crates/axumkit-worker` | `crates/kit-worker` |

#### 2.2.3 更新 Cargo.toml 文件

**kit-config/Cargo.toml**：
```toml
[package]
name = "kit-config"
# 其他配置保持不变
```

**kit-constants/Cargo.toml**：
```toml
[package]
name = "kit-constants"
# 其他配置保持不变
```

**kit-errors/Cargo.toml**：
```toml
[package]
name = "kit-errors"
# 其他配置保持不变
```

**kit-worker/Cargo.toml**：
```toml
[package]
name = "kit-worker"
# 其他配置保持不变
```

#### 2.2.4 更新依赖引用

需要更新以下文件中的依赖引用：

1. **Cargo.toml 文件**：
   - `crates/axumkit-server/Cargo.toml`
   - `crates/axumkit-dto/Cargo.toml`
   - `crates/axumkit-entity/Cargo.toml`
   - `crates/migration/Cargo.toml`
   - `crates/e2e/Cargo.toml`

2. **代码文件**：
   - 所有使用 `axumkit-*` 模块的代码文件
   - 将 `use axumkit_*` 改为 `use kit_*`

## 3. 依赖关系处理

### 3.1 模块间依赖

| 模块 | 依赖模块 |
|------|----------|
| `kit-config` | - |
| `kit-constants` | - |
| `kit-errors` | - |
| `kit-core` | - |
| `kit-worker` | `kit-config`, `kit-errors`, `kit-core` |
| `kit-server` (原 axumkit-server) | `kit-config`, `kit-constants`, `kit-errors`, `kit-core` |

### 3.2 依赖更新策略

1. **先更新独立模块**：`kit-config`, `kit-constants`, `kit-errors`
2. **创建 kit-core 模块**
3. **更新 kit-worker 模块**
4. **最后更新 kit-server 模块**

## 4. 风险和注意事项

1. **依赖关系风险**：重命名模块可能导致依赖关系混乱，需要仔细检查所有引用
2. **编译错误**：可能会出现大量编译错误，需要逐一修复
3. **测试失败**：重命名后可能会导致测试失败，需要更新测试代码
4. **IDE 支持**：IDE 可能需要重新索引项目，影响开发体验
5. **版本控制**：重命名操作可能会影响版本控制历史

## 5. 执行策略

1. **备份**：在执行重命名操作前，确保代码已提交到版本控制系统
2. **分步执行**：按照计划分步执行，每完成一步就运行 `cargo check` 验证
3. **批量修改**：使用工具批量修改引用路径，减少手动操作
4. **测试验证**：每完成一个模块的重命名，就运行相关测试验证功能
5. **文档更新**：更新项目文档中的模块引用

## 6. 预期结果

完成重命名后，项目结构将如下所示：

```
crates/
├── kit-config/        # 全局配置管理
├── kit-constants/     # 共享常量定义
├── kit-errors/        # 错误处理系统
├── kit-core/          # 核心功能和工具
├── kit-worker/        # 后台任务处理
├── axumkit-dto/       # 待后续重命名为 kit-dto
├── axumkit-entity/     # 待后续重命名为 kit-entity
├── axumkit-server/     # 待后续重命名为 kit-server
├── e2e/               # 测试模块
├── migration/          # 数据库迁移
└── xtask/             # 工具任务
```

## 7. 后续步骤

完成共享模块重命名后，将继续按照 DDD 架构改造方案，对其他模块进行重命名和重构，最终实现完整的 DDD 架构。