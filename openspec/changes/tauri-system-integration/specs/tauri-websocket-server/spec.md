## ADDED Requirements

### Requirement: Embedded WebSocket TLS server

The system SHALL 在 Rust 侧运行嵌入式 WebSocket 服务器（替代现有的 H3 + CrossWS Node.js 实现），支持 TLS 加密。

#### Scenario: Server starts on application launch
- **WHEN** 应用启动且 server channel 配置启用
- **THEN** WebSocket 服务器在端口 6121（或环境变量指定端口）启动

#### Scenario: TLS encrypted connection
- **WHEN** 客户端通过 `wss://` 连接服务器
- **THEN** 连接使用自签名 TLS 证书加密

#### Scenario: Graceful shutdown
- **WHEN** 应用退出
- **THEN** 服务器关闭所有活跃连接后停止

### Requirement: Self-signed certificate management

The system SHALL 生成和管理自签名 TLS 证书：
- 使用 `rcgen` crate 生成 CA 和服务器证书
- 证书 SAN 包含 `localhost` 和所有本地 IP 地址
- 支持将 CA 证书安装到系统信任存储

#### Scenario: Certificate generation
- **WHEN** 应用首次启动且无现有证书
- **THEN** 自动生成 CA 证书和服务器证书，保存到 userData 目录

#### Scenario: macOS certificate installation
- **WHEN** 用户确认安装证书 on macOS
- **THEN** CA 证书通过 `security add-trusted-cert` 安装到 Keychain

#### Scenario: Certificate with local IPs
- **WHEN** 证书生成时
- **THEN** SAN 包含 `localhost`、`127.0.0.1`、以及所有非虚拟网络接口的 IP 地址
