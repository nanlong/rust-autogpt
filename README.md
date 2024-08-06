# Rust AutoGPT

- Auto Gippity

Managing Agent (管理):

    - 将用户需求转化为任务目标

Solutions Architect (架构师):

    - 设定项目的开发范围
        - 是否需要CRUD
        - 是否需要用户登录
        - 是否需要访问第三方API
    - 提供第三方API

Backend Developer (程序员):

    - 生成具体项目代码
    - 优化或改进现有的代码
    - 测试并修复代码
    - 为接口生成JSON Schema

## 开发 Agent 的主要模块

- 提示工程，为 Agent 设定人设、任务目标、处理流程、结果输出等
- 向量数据库，为 Agent 增加长期记忆功能
- 功能函数，为 Agent 增加感知能力和行动能力
- 当要求返回 json 格式时，最好是将需求描述成一个 Function，增加 Input、Function、Important、Output、Example 等字段，方便 Agent 生成 json 格式的需求描述

## 工具

    - [Prompt生成](https://console.anthropic.com)
