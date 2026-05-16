#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
IMA 知识库 API 调用脚本
基于 IMA 官方 Skill 包 (v1.1.7) 的 API 文档
用于 TRAE SOLO Skills 集成 IMA 知识库功能

API 文档来源:
  - 笔记 API: .trae/skills/ima-knowledge/notes/references/api.md
  - 知识库 API: .trae/skills/ima-knowledge/knowledge-base/references/api.md
"""

import os
import sys
import json
import urllib.request
import urllib.error

IMA_API_BASE = "https://ima.qq.com"


def get_credentials():
    """获取 API 凭证"""
    client_id = os.environ.get("IMA_OPENAPI_CLIENTID", "")
    api_key = os.environ.get("IMA_OPENAPI_APIKEY", "")

    if not client_id or not api_key:
        print("错误：请设置环境变量 IMA_OPENAPI_CLIENTID 和 IMA_OPENAPI_APIKEY")
        sys.exit(1)

    return client_id, api_key


def call_ima_api(path, body=None):
    """调用 IMA API（统一 POST + JSON）"""
    client_id, api_key = get_credentials()

    url = f"{IMA_API_BASE}/{path}"
    headers = {
        "ima-openapi-clientid": client_id,
        "ima-openapi-apikey": api_key,
        "Content-Type": "application/json"
    }

    data = json.dumps(body).encode("utf-8") if body else b"{}"
    req = urllib.request.Request(url, data=data, headers=headers, method="POST")

    try:
        with urllib.request.urlopen(req, timeout=30) as response:
            result = response.read().decode("utf-8")
            return json.loads(result)
    except urllib.error.HTTPError as e:
        error_body = e.read().decode("utf-8") if e.fp else "No body"
        print(f"API 调用失败：HTTP {e.code}")
        print(f"响应: {error_body[:500]}")
        return None
    except Exception as e:
        print(f"API 调用异常：{type(e).__name__}: {str(e)}")
        return None


# ============================================================
# 笔记相关 API
# ============================================================

def search_note(keyword, search_type=0, start=0, end=20):
    """搜索笔记 (search_type: 0=标题, 1=正文)"""
    body = {
        "search_type": search_type,
        "query_info": {
            "title": keyword if search_type == 0 else "",
            "content": keyword if search_type == 1 else ""
        },
        "start": start,
        "end": end
    }
    result = call_ima_api("openapi/note/v1/search_note", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def list_notebook(cursor="0", limit=20):
    """列出笔记本"""
    body = {"cursor": cursor, "limit": limit}
    result = call_ima_api("openapi/note/v1/list_notebook", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def list_notes(folder_id="", cursor="", limit=20):
    """列出指定笔记本下的笔记"""
    body = {"folder_id": folder_id, "cursor": cursor, "limit": limit}
    result = call_ima_api("openapi/note/v1/list_note", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def get_note_content(note_id):
    """读取笔记内容 (target_content_format: 0=纯文本, 1=Markdown, 2=JSON)"""
    body = {"note_id": note_id, "target_content_format": 0}
    result = call_ima_api("openapi/note/v1/get_doc_content", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def create_note(title, content, folder_id="", folder_name=""):
    """创建新笔记"""
    full_content = f"# {title}\n\n{content}"
    body = {
        "content_format": 1,
        "content": full_content,
        "folder_id": folder_id,
        "folder_name": folder_name
    }
    result = call_ima_api("openapi/note/v1/import_doc", body)
    if result:
        print("笔记创建成功！")
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def append_note(note_id, content):
    """追加内容到已有笔记"""
    body = {"note_id": note_id, "content": content, "content_format": 1}
    result = call_ima_api("openapi/note/v1/append_doc", body)
    if result:
        print("内容追加成功！")
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


# ============================================================
# 知识库相关 API
# ============================================================

def search_knowledge_base(query, cursor="", limit=20):
    """搜索知识库列表"""
    body = {"query": query, "cursor": cursor, "limit": limit}
    result = call_ima_api("openapi/wiki/v1/search_knowledge_base", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def get_knowledge_base(kb_ids):
    """获取知识库信息"""
    body = {"ids": kb_ids}
    result = call_ima_api("openapi/wiki/v1/get_knowledge_base", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def get_knowledge_list(knowledge_base_id, folder_id="", cursor="", limit=20):
    """浏览知识库内容"""
    body = {
        "knowledge_base_id": knowledge_base_id,
        "folder_id": folder_id,
        "cursor": cursor,
        "limit": limit
    }
    result = call_ima_api("openapi/wiki/v1/get_knowledge_list", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def search_knowledge(query, knowledge_base_id, cursor=""):
    """在知识库中搜索内容"""
    body = {
        "query": query,
        "knowledge_base_id": knowledge_base_id,
        "cursor": cursor
    }
    result = call_ima_api("openapi/wiki/v1/search_knowledge", body)
    if result:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def import_urls(knowledge_base_id, folder_id, urls):
    """导入 URL 到知识库"""
    body = {
        "knowledge_base_id": knowledge_base_id,
        "folder_id": folder_id,
        "urls": urls
    }
    result = call_ima_api("openapi/wiki/v1/import_urls", body)
    if result:
        print("URL 导入成功！")
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return result


def main():
    if len(sys.argv) < 2:
        print("用法: python ima_api.py <命令> [参数]")
        print()
        print("笔记命令:")
        print("  search_note <keyword> [type]       - 搜索笔记 (type: 0=标题, 1=内容)")
        print("  list_notebook [cursor] [limit]     - 列出笔记本")
        print("  list_notes [folder_id] [limit]     - 列出笔记")
        print("  get_note <note_id>                 - 读取笔记内容")
        print("  create_note <title> <content> [folder_id] [folder_name]")
        print("  append_note <note_id> <content>    - 追加内容")
        print()
        print("知识库命令:")
        print("  search_kb <query>                  - 搜索知识库列表")
        print("  get_kb <id1,id2,...>               - 获取知识库信息")
        print("  list_kb_content <kb_id> [folder_id] - 浏览知识库内容")
        print("  search_kb_content <query> <kb_id>  - 在知识库中搜索")
        print("  import_urls <kb_id> <folder_id> <url1,url2,...>")
        sys.exit(0)

    command = sys.argv[1]

    if command == "search_note" and len(sys.argv) >= 3:
        stype = int(sys.argv[3]) if len(sys.argv) > 3 else 0
        search_note(sys.argv[2], stype)
    elif command == "list_notebook":
        cursor = sys.argv[2] if len(sys.argv) > 2 else "0"
        limit = int(sys.argv[3]) if len(sys.argv) > 3 else 20
        list_notebook(cursor, limit)
    elif command == "list_notes":
        fid = sys.argv[2] if len(sys.argv) > 2 else ""
        limit = int(sys.argv[3]) if len(sys.argv) > 3 else 20
        list_notes(fid, "", limit)
    elif command == "get_note" and len(sys.argv) >= 3:
        get_note_content(sys.argv[2])
    elif command == "create_note" and len(sys.argv) >= 4:
        fid = sys.argv[4] if len(sys.argv) > 4 else ""
        fname = sys.argv[5] if len(sys.argv) > 5 else ""
        create_note(sys.argv[2], sys.argv[3], fid, fname)
    elif command == "append_note" and len(sys.argv) >= 4:
        append_note(sys.argv[2], sys.argv[3])
    elif command == "search_kb" and len(sys.argv) >= 3:
        search_knowledge_base(sys.argv[2])
    elif command == "get_kb" and len(sys.argv) >= 3:
        get_knowledge_base(sys.argv[2].split(","))
    elif command == "list_kb_content" and len(sys.argv) >= 3:
        fid = sys.argv[3] if len(sys.argv) > 3 else ""
        get_knowledge_list(sys.argv[2], fid)
    elif command == "search_kb_content" and len(sys.argv) >= 4:
        search_knowledge(sys.argv[2], sys.argv[3])
    elif command == "import_urls" and len(sys.argv) >= 5:
        import_urls(sys.argv[2], sys.argv[3], sys.argv[4].split(","))
    else:
        print(f"未知命令或参数不足: {command}")
        print("运行 python ima_api.py 查看帮助")


if __name__ == "__main__":
    main()
