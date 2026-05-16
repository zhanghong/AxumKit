
---
name: "ima-knowledge"
description: "管理 IMA 知识库和笔记，支持笔记搜索、创建、追加、读取，以及知识库内容浏览和搜索。当用户询问项目知识、架构、约定或需要引用存储的知识时调用。"
---

# IMA 知识库

本技能为 AxumKit 项目提供 IMA 知识库和笔记管理功能，帮助存储、组织和检索项目特定知识、架构模式、开发约定和最佳实践。

## 触发条件

- 用户使用 `/ima-search` 命令搜索知识库或笔记
- 用户使用 `/ima-note` 命令创建或追加笔记
- 用户使用 `/ima-save` 命令保存内容到知识库
- 用户询问项目架构、约定或最佳实践时

## 指令

### 搜索笔记
```
python .trae/skills/ima-knowledge/ima_api.py search_note "搜索关键词" [搜索类型]
```
搜索类型: 0=标题搜索(默认), 1=正文搜索

### 列出笔记本
```
python .trae/skills/ima-knowledge/ima_api.py list_notebook
```

### 列出笔记
```
python .trae/skills/ima-knowledge/ima_api.py list_notes [folder_id]
```

### 读取笔记内容
```
python .trae/skills/ima-knowledge/ima_api.py get_note "笔记ID"
```

### 创建笔记
```
python .trae/skills/ima-knowledge/ima_api.py create_note "标题" "内容" [folder_id] [folder_name]
```

### 追加内容到笔记
```
python .trae/skills/ima-knowledge/ima_api.py append_note "笔记ID" "追加内容"
```

### 搜索知识库列表
```
python .trae/skills/ima-knowledge/ima_api.py search_kb "搜索关键词"
```

### 浏览知识库内容
```
python .trae/skills/ima-knowledge/ima_api.py list_kb_content "知识库ID" [folder_id]
```

### 在知识库中搜索
```
python .trae/skills/ima-knowledge/ima_api.py search_kb_content "搜索关键词" "知识库ID"
```

## 环境配置

需要配置以下环境变量：
- `IMA_OPENAPI_CLIENTID`: IMA 客户端 ID
- `IMA_OPENAPI_APIKEY`: IMA API 密钥

Windows PowerShell 永久配置方式：
```powershell
[System.Environment]::SetEnvironmentVariable('IMA_OPENAPI_CLIENTID', '你的ClientID', 'User')
[System.Environment]::SetEnvironmentVariable('IMA_OPENAPI_APIKEY', '你的APIKey', 'User')
```

## 核心知识领域

- AxumKit 项目架构
- 开发模式和约定
- 数据库和 ORM 使用
- 认证与会话管理
- API 设计模式
- 配置管理

## 使用指南

1. 首先检查 `.trae/documents/` 目录中是否有现有知识
2. 参考 CLAUDE.md 文件获取项目特定约定
3. 提供准确的项目特定信息
4. 尽可能链接到相关代码文件
5. 将项目文档迁移到 IMA 知识库
