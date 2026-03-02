# 開発ガイド

本章では、新しい画面（ユースケース）を追加する際の体系的な実装手順を定義する。

---

## 1.1 ワークフロー概要

画面追加は以下の順序で実装する：

```
1. ドメイン層（業務ロジック）
   ↓
2. アプリケーション層（ユースケース）
   ↓
3. インフラストラクチャ層（永続化・Query実装）
   ↓
4. アダプター層（UI・Controller・Presenter）
   ↓
5. エントリーポイント（DI設定）
   ↓
6. ナビゲーション統合（メニュー・ルーティング）
```

**重要原則:**
- 内側（Domain）から外側（Adapter）へ実装
- 各層の実装完了後、次の層へ進む
- 簡略化・一時実装は厳禁
- 既存パターンを完全に踏襲

---

## 1.2 Phase 1: ドメイン層実装

### 1.2.1 目的

業務ルールと整合性制約を実装する。

### 1.2.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| Entity | 業務主体 | `crates/javelin-domain/src/{module}/entities/` |
| ValueObject | 値意味保持 | `crates/javelin-domain/src/{module}/value_objects/` |
| DomainService | 横断処理 | `crates/javelin-domain/src/{module}/services.rs` |
| DomainError | ドメインエラー | `crates/javelin-domain/src/{module}/errors.rs` |

### 1.2.3 実装手順

**Step 1: ValueObject定義**

```rust
// crates/javelin-domain/src/{module}/value_objects/{name}.rs

use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxxValue(String);

#[derive(Error, Debug)]
pub enum XxxValueError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
}

impl XxxValue {
    pub fn new(value: String) -> Result<Self, XxxValueError> {
        // バリデーション
        if value.is_empty() {
            return Err(XxxValueError::InvalidFormat("empty".to_string()));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl FromStr for XxxValue {
    type Err = XxxValueError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl fmt::Display for XxxValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

**Step 2: Entity定義**

```rust
// crates/javelin-domain/src/{module}/entities/{name}.rs

use uuid::Uuid;
use super::super::value_objects::*;

#[derive(Debug, Clone)]
pub struct XxxEntity {
    id: Uuid,
    name: XxxValue,
    // その他のフィールド
}

impl XxxEntity {
    pub fn new(id: Uuid, name: XxxValue) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &XxxValue {
        &self.name
    }

    // 業務ロジックメソッド
    pub fn update_name(&mut self, name: XxxValue) -> Result<(), DomainError> {
        // バリデーション
        self.name = name;
        Ok(())
    }
}
```

**Step 3: DomainService定義（必要な場合のみ）**

```rust
// crates/javelin-domain/src/{module}/services.rs

use super::entities::*;
use super::errors::*;

pub struct XxxService;

impl XxxService {
    /// 複数エンティティにまたがる処理
    pub fn validate_xxx(entity1: &Entity1, entity2: &Entity2) -> Result<(), DomainError> {
        // 横断的なバリデーション
        Ok(())
    }
}
```

**Step 4: テスト実装**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxx_value_creation() {
        let value = XxxValue::new("test".to_string());
        assert!(value.is_ok());
    }

    #[test]
    fn test_xxx_value_validation() {
        let value = XxxValue::new("".to_string());
        assert!(value.is_err());
    }

    #[test]
    fn test_xxx_entity_creation() {
        let id = Uuid::new_v4();
        let name = XxxValue::new("test".to_string()).unwrap();
        let entity = XxxEntity::new(id, name);
        assert_eq!(entity.id(), id);
    }
}
```

### 1.2.4 チェックポイント

- [ ] ValueObjectにFromStr/Display実装済み
- [ ] Entityに業務ロジックメソッド実装済み
- [ ] DomainServiceは複数Entity横断処理のみ
- [ ] 単体テスト実装済み（カバレッジ80%以上）
- [ ] コンパイルエラーなし
- [ ] 外部依存なし（Domain層は完全独立）

---

## 1.3 Phase 2: アプリケーション層実装

### 1.3.1 目的

ユースケースを実装し、ドメインを調整する。

### 1.3.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| InputPort | ユースケース定義 | `crates/javelin-application/src/input_ports/{name}.rs` |
| Interactor | ユースケース実装 | `crates/javelin-application/src/interactor/{module}/{name}_interactor.rs` |
| OutputPort | 出力抽象 | `crates/javelin-application/src/output_ports/{name}.rs` |
| RequestDTO | 入力DTO | `crates/javelin-application/src/dtos/request/{name}.rs` |
| ResponseDTO | 出力DTO | `crates/javelin-application/src/dtos/response/{name}.rs` |

### 1.3.3 実装手順

**Step 1: RequestDTO定義**

```rust
// crates/javelin-application/src/dtos/request/{name}.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XxxRequestDto {
    pub name: String,
    pub value: i64,
    // プリミティブ型のみ
}

// DTO → Domain変換
impl TryFrom<&XxxRequestDto> for javelin_domain::{module}::entities::XxxEntity {
    type Error = crate::errors::ApplicationError;

    fn try_from(dto: &XxxRequestDto) -> Result<Self, Self::Error> {
        use javelin_domain::{module}::value_objects::*;
        
        let name = XxxValue::new(dto.name.clone())
            .map_err(|e| ApplicationError::DomainError(e.to_string()))?;
        
        Ok(XxxEntity::new(uuid::Uuid::new_v4(), name))
    }
}
```

**Step 2: ResponseDTO定義**

```rust
// crates/javelin-application/src/dtos/response/{name}.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XxxResponseDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

// Domain → DTO変換
impl From<&javelin_domain::{module}::entities::XxxEntity> for XxxResponseDto {
    fn from(entity: &javelin_domain::{module}::entities::XxxEntity) -> Self {
        Self {
            id: entity.id().to_string(),
            name: entity.name().to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```

**Step 3: OutputPort定義**

```rust
// crates/javelin-application/src/output_ports/{name}.rs

use super::super::dtos::response::XxxResponseDto;

/// Xxx出力ポート
pub trait XxxOutputPort: Send + Sync {
    /// 成功結果を提示
    fn present_success(&self, result: XxxResponseDto);
    
    /// エラーを提示
    fn present_error(&self, error_message: String);
    
    /// バリデーションエラーを提示
    fn present_validation_error(&self, error_message: String);
    
    /// 進捗を提示（バッチ処理の場合）
    fn present_progress(&self, message: String);
    
    /// 実行時間を提示
    fn present_execution_time(&self, elapsed_ms: usize);
}
```

**Step 4: InputPort定義**

```rust
// crates/javelin-application/src/input_ports/{name}.rs

use crate::dtos::request::XxxRequestDto;
use crate::errors::ApplicationError;

/// Xxxユースケース
pub trait XxxUseCase: Send + Sync {
    /// Xxxを実行
    async fn execute(&self, request: XxxRequestDto) -> Result<(), ApplicationError>;
}
```

**Step 5: Interactor実装**

```rust
// crates/javelin-application/src/interactor/{module}/{name}_interactor.rs

use std::sync::Arc;
use crate::{
    dtos::request::XxxRequestDto,
    dtos::response::XxxResponseDto,
    errors::ApplicationError,
    input_ports::XxxUseCase,
    output_ports::XxxOutputPort,
};
use javelin_domain::{module}::services::XxxService;

pub struct XxxInteractor {
    output_port: Arc<dyn XxxOutputPort>,
}

impl XxxInteractor {
    pub fn new(output_port: Arc<dyn XxxOutputPort>) -> Self {
        Self { output_port }
    }
}

impl XxxUseCase for XxxInteractor {
    async fn execute(&self, request: XxxRequestDto) -> Result<(), ApplicationError> {
        let start = std::time::Instant::now();

        // 1. DTOをドメインオブジェクトに変換
        let entity = (&request).try_into()?;

        // 2. ドメインサービスで処理
        XxxService::validate_xxx(&entity)
            .map_err(|e| ApplicationError::DomainError(e.to_string()))?;

        // 3. レスポンスDTOに変換
        let response = XxxResponseDto::from(&entity);

        // 4. OutputPortで結果を提示
        self.output_port.present_success(response);
        self.output_port.present_execution_time(start.elapsed().as_millis() as usize);

        Ok(())
    }
}
```

**Step 6: テスト実装**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct MockOutputPort {
        success_called: Arc<Mutex<bool>>,
    }

    impl XxxOutputPort for MockOutputPort {
        fn present_success(&self, _result: XxxResponseDto) {
            *self.success_called.lock().unwrap() = true;
        }
        fn present_error(&self, _error_message: String) {}
        fn present_validation_error(&self, _error_message: String) {}
        fn present_progress(&self, _message: String) {}
        fn present_execution_time(&self, _elapsed_ms: usize) {}
    }

    #[tokio::test]
    async fn test_execute_success() {
        let success_called = Arc::new(Mutex::new(false));
        let output_port = Arc::new(MockOutputPort {
            success_called: Arc::clone(&success_called),
        });
        let interactor = XxxInteractor::new(output_port);

        let request = XxxRequestDto {
            name: "test".to_string(),
            value: 100,
        };

        let result = interactor.execute(request).await;
        assert!(result.is_ok());
        assert!(*success_called.lock().unwrap());
    }
}
```

### 1.3.4 チェックポイント

- [ ] RequestDTOにTryFrom実装済み
- [ ] ResponseDTOにFrom実装済み
- [ ] OutputPortに必要なメソッド定義済み
- [ ] InteractorがInputPortを実装済み
- [ ] ドメインサービスを呼び出し（ロジック重複なし）
- [ ] テスト実装済み（正常系・異常系）
- [ ] コンパイルエラーなし
- [ ] Rust 2024 Edition準拠（async fn in traits、let-else使用）

---

## 1.4 Phase 3: インフラストラクチャ層実装（Query機能の場合）

### 1.4.1 目的

読み取り専用のQuery機能を実装する。

### 1.4.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| QueryService | 検索処理 | `crates/javelin-infrastructure/src/read/{module}/{name}_query_service.rs` |
| ProjectionBuilder | ReadModel生成 | `crates/javelin-infrastructure/src/projection/{module}/{name}_projection_builder.rs` |

### 1.4.3 実装手順

**Step 1: QueryService実装**

```rust
// crates/javelin-infrastructure/src/read/{module}/{name}_query_service.rs

use javelin_application::dtos::response::XxxResponseDto;
use std::sync::Arc;

pub struct XxxQueryServiceImpl {
    // Projection DBへの参照
}

impl XxxQueryServiceImpl {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<XxxResponseDto>, String> {
        // Projection DBから検索
        Ok(None)
    }

    pub async fn find_all(&self) -> Result<Vec<XxxResponseDto>, String> {
        // Projection DBから全件取得
        Ok(vec![])
    }
}
```

### 1.4.4 チェックポイント

- [ ] QueryServiceはProjection DBのみ参照
- [ ] Event Storeへの直接アクセスなし
- [ ] 検索ロジックはApplication層で管理
- [ ] コンパイルエラーなし

---

## 1.5 Phase 4: アダプター層実装（Presenterパターン）

### 1.5.1 目的

TUI画面とPresenterを実装し、ユーザー操作を受け付ける。

### 1.5.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| Presenter | OutputPort実装 | `crates/javelin-adapter/src/presenter/{name}_presenter.rs` |
| Controller | InputPort呼び出し | `crates/javelin-adapter/src/controller/{name}_controller.rs` |
| PageState | 画面状態管理 | `crates/javelin-adapter/src/page_states/{module}/{name}.rs` |
| Page | View描画 | `crates/javelin-adapter/src/views/pages/{module}/{name}_page.rs` |

### 1.5.3 Presenterパターンアーキテクチャ

```
┌─────────────┐
│  PageState  │ ← 画面ライフサイクル管理
└──────┬──────┘
       │ 1. Presenterチャネル作成
       │ 2. PresenterRegistry登録
       │ 3. Pageにチャネル渡す
       ↓
┌─────────────┐
│    Page     │ ← View描画・チャネル受信
└──────┬──────┘
       │ キーボード入力
       ↓
┌─────────────┐
│ Controller  │ ← RequestDTO作成
└──────┬──────┘
       │ handle_xxx(page_id, dto)
       ↓
┌─────────────┐
│ Interactor  │ ← ユースケース実行
└──────┬──────┘
       │ present_xxx()
       ↓
┌─────────────┐
│  Presenter  │ ← チャネル送信
└──────┬──────┘
       │ try_send()
       ↓
┌─────────────┐
│    Page     │ ← poll_xxx() でチャネル受信
└─────────────┘
```

**重要原則:**
- PageStateがPresenterを作成・登録・破棄
- PageはチャネルでPresenterから結果を受信
- Controllerはpage_idでPresenterを取得
- 簡略化実装は厳禁（必ずチャネル通信を実装）

---

### 1.5.4 実装手順

**Step 1: Presenter実装**

```rust
// crates/javelin-adapter/src/presenter/{name}_presenter.rs

use javelin_application::{
    dtos::response::XxxResponseDto,
    output_ports::XxxOutputPort,
};
use tokio::sync::mpsc;

/// XxxViewModel
#[derive(Debug, Clone)]
pub struct XxxViewModel {
    pub id: String,
    pub name: String,
    pub display_name: String,
}

/// Xxxプレゼンター
#[derive(Clone)]
pub struct XxxPresenter {
    result_tx: mpsc::Sender<XxxViewModel>,
    error_tx: mpsc::Sender<String>,
    progress_tx: mpsc::Sender<String>,
    execution_time_tx: mpsc::Sender<usize>,
}

pub struct XxxChannels {
    pub result_rx: mpsc::Receiver<XxxViewModel>,
    pub error_rx: mpsc::Receiver<String>,
    pub progress_rx: mpsc::Receiver<String>,
    pub execution_time_rx: mpsc::Receiver<usize>,
}

impl XxxPresenter {
    pub fn new(
        result_tx: mpsc::Sender<XxxViewModel>,
        error_tx: mpsc::Sender<String>,
        progress_tx: mpsc::Sender<String>,
        execution_time_tx: mpsc::Sender<usize>,
    ) -> Self {
        Self { result_tx, error_tx, progress_tx, execution_time_tx }
    }

    pub fn create_channels() -> (XxxPresenter, XxxChannels) {
        let (result_tx, result_rx) = mpsc::channel(100);
        let (error_tx, error_rx) = mpsc::channel(100);
        let (progress_tx, progress_rx) = mpsc::channel(100);
        let (execution_time_tx, execution_time_rx) = mpsc::channel(100);

        let presenter = XxxPresenter::new(result_tx, error_tx, progress_tx, execution_time_tx);
        let channels = XxxChannels { result_rx, error_rx, progress_rx, execution_time_rx };

        (presenter, channels)
    }

    fn to_view_model(&self, dto: XxxResponseDto) -> XxxViewModel {
        XxxViewModel {
            id: dto.id,
            name: dto.name.clone(),
            display_name: format!("【{}】", dto.name),
        }
    }
}

impl XxxOutputPort for XxxPresenter {
    fn present_success(&self, result: XxxResponseDto) {
        let view_model = self.to_view_model(result);
        let _ = self.result_tx.try_send(view_model);
    }

    fn present_error(&self, error_message: String) {
        let _ = self.error_tx.try_send(error_message);
    }

    fn present_validation_error(&self, error_message: String) {
        let _ = self.error_tx.try_send(error_message);
    }

    fn present_progress(&self, message: String) {
        let _ = self.progress_tx.try_send(message);
    }

    fn present_execution_time(&self, elapsed_ms: usize) {
        let _ = self.execution_time_tx.try_send(elapsed_ms);
    }
}
```

**Step 2: Controller実装**

```rust
// crates/javelin-adapter/src/controller/{name}_controller.rs

use std::sync::Arc;
use javelin_application::dtos::request::XxxRequestDto;
use crate::navigation::PresenterRegistry;

pub struct XxxController {
    presenter_registry: Arc<PresenterRegistry>,
}

impl XxxController {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        Self { presenter_registry }
    }

    pub fn presenter_registry(&self) -> &Arc<PresenterRegistry> {
        &self.presenter_registry
    }

    pub async fn handle_xxx(
        &self,
        page_id: uuid::Uuid,
        request: XxxRequestDto,
    ) -> Result<(), String> {
        use javelin_application::input_ports::XxxUseCase;

        // PresenterRegistryからpage_id用のPresenterを取得
        let Some(presenter_arc) = self.presenter_registry.get_xxx_presenter(page_id) else {
            return Err(format!("XxxPresenter not found for page_id: {}", page_id));
        };

        // ArcからPresenterをclone
        let presenter = (*presenter_arc).clone();

        // このページ専用のInteractorを動的に作成
        let interactor = javelin_application::interactor::{module}::XxxInteractor::new(
            presenter.into(),
        );

        // 実行
        interactor.execute(request).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
```

**Step 3: PageState実装**

```rust
// crates/javelin-adapter/src/page_states/{module}/{name}.rs

use std::sync::Arc;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use uuid::Uuid;

use crate::{
    error::AdapterResult,
    navigation::{Controllers, NavAction, PageState, PresenterRegistry, Route},
    presenter::XxxPresenter,
    views::pages::XxxPage,
};

pub struct XxxPageState {
    page_id: Uuid,
    page: XxxPage,
    presenter_registry: Arc<PresenterRegistry>,
}

impl XxxPageState {
    pub fn new(presenter_registry: Arc<PresenterRegistry>) -> Self {
        let page_id = Uuid::new_v4();

        // Presenterのチャネルを作成
        let (presenter, channels) = XxxPresenter::create_channels();

        // PresenterRegistryに登録
        presenter_registry.register_xxx_presenter(page_id, Arc::new(presenter));

        // Pageを作成（チャネルを渡す）
        let page = XxxPage::new(page_id, channels);

        Self { page_id, page, presenter_registry }
    }
}

impl Drop for XxxPageState {
    fn drop(&mut self) {
        // PresenterRegistryから登録解除
        self.presenter_registry.unregister_xxx_presenter(self.page_id);
    }
}

impl PageState for XxxPageState {
    fn route(&self) -> Route {
        Route::Xxx
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        controllers: &Controllers,
    ) -> AdapterResult<NavAction> {
        loop {
            // アニメーション更新
            self.page.tick();

            // 非同期結果をポーリング
            self.page.poll_results();

            // 描画
            terminal
                .draw(|frame| {
                    self.page.render(frame);
                })
                .map_err(|e| crate::error::AdapterError::RenderingFailed(e.to_string()))?;

            // イベント処理
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(crate::error::AdapterError::EventReadFailed)?
                && let Event::Key(key) =
                    event::read().map_err(crate::error::AdapterError::EventReadFailed)?
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Esc => {
                        return Ok(NavAction::Back);
                    }
                    KeyCode::Enter => {
                        // 処理実行
                        let controller = controllers.xxx.clone();
                        let page_id = self.page_id;

                        tokio::spawn(async move {
                            use javelin_application::dtos::request::XxxRequestDto;

                            let request = XxxRequestDto {
                                name: "test".to_string(),
                                value: 100,
                            };

                            let _ = controller.handle_xxx(page_id, request).await;
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
```

**Step 4: Page実装**

```rust
// crates/javelin-adapter/src/views/pages/{module}/{name}_page.rs

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use uuid::Uuid;

use crate::{
    presenter::{XxxChannels, XxxViewModel},
    views::components::loading_spinner::LoadingSpinner,
};

pub struct XxxPage {
    id: Uuid,
    channels: XxxChannels,
    result: Option<XxxViewModel>,
    error_message: Option<String>,
    progress_message: Option<String>,
    execution_time: Option<usize>,
    loading_spinner: LoadingSpinner,
    is_loading: bool,
}

impl XxxPage {
    pub fn new(id: Uuid, channels: XxxChannels) -> Self {
        Self {
            id,
            channels,
            result: None,
            error_message: None,
            progress_message: None,
            execution_time: None,
            loading_spinner: LoadingSpinner::new(),
            is_loading: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    /// 非同期結果をポーリング
    pub fn poll_results(&mut self) {
        // 結果を受信
        while let Ok(result) = self.channels.result_rx.try_recv() {
            self.result = Some(result);
            self.is_loading = false;
            self.error_message = None;
        }

        // エラーを受信
        while let Ok(error) = self.channels.error_rx.try_recv() {
            self.error_message = Some(error);
            self.is_loading = false;
        }

        // 進捗を受信
        while let Ok(progress) = self.channels.progress_rx.try_recv() {
            self.progress_message = Some(progress);
        }

        // 実行時間を受信
        while let Ok(time) = self.channels.execution_time_rx.try_recv() {
            self.execution_time = Some(time);
        }
    }

    pub fn tick(&mut self) {
        if self.is_loading {
            self.loading_spinner.tick();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3), // タイトル
            Constraint::Min(0),    // コンテンツ
            Constraint::Length(3), // ヘルプ
        ])
        .split(frame.area());

        // タイトル
        let title = Paragraph::new("Xxx Screen")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(title, chunks[0]);

        // コンテンツ
        if self.is_loading {
            let message = self.progress_message.as_deref().unwrap_or("処理中...");
            self.loading_spinner.render(frame, chunks[1], message);
        } else if let Some(ref error) = self.error_message {
            let error_widget = Paragraph::new(error.as_str())
                .block(Block::default().borders(Borders::ALL).title("Error"))
                .style(Style::default().fg(Color::Red));
            frame.render_widget(error_widget, chunks[1]);
        } else if let Some(ref result) = self.result {
            let content = Paragraph::new(format!("Result: {}", result.display_name))
                .block(Block::default().borders(Borders::ALL).title("Success"))
                .style(Style::default().fg(Color::Green));
            frame.render_widget(content, chunks[1]);
        } else {
            let empty = Paragraph::new("Press Enter to execute")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Gray));
            frame.render_widget(empty, chunks[1]);
        }

        // ヘルプ
        let help = Paragraph::new("[Enter] Execute  [Esc] Back")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(help, chunks[2]);
    }
}
```

### 1.5.5 チェックポイント

- [ ] Presenterにcreate_channels()実装済み
- [ ] PageStateでPresenter作成・登録・破棄
- [ ] PageがチャネルでPresenterから結果受信
- [ ] Controllerがpage_idでPresenterを取得
- [ ] 簡略化実装なし（必ずチャネル通信）
- [ ] コンパイルエラーなし

---

## 1.6 Phase 5: エントリーポイント（DI設定）

### 1.6.1 目的

依存性注入を設定し、アプリケーション起動時にコンポーネントを初期化する。

### 1.6.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| Controllers | Controller集約 | `crates/javelin-adapter/src/navigation/controllers.rs` |
| PresenterRegistry | Presenter登録管理 | `crates/javelin-adapter/src/navigation/presenter_registry.rs` |
| app_setup.rs | DI設定 | `crates/javelin/src/app_setup.rs` |
| app_resolver.rs | PageState生成 | `crates/javelin/src/app_resolver.rs` |

### 1.6.3 実装手順

**Step 1: PresenterRegistryに登録メソッド追加**

```rust
// crates/javelin-adapter/src/navigation/presenter_registry.rs

use crate::presenter::XxxPresenter;

impl PresenterRegistry {
    pub fn register_xxx_presenter(&self, id: Uuid, presenter: Arc<XxxPresenter>) {
        self.xxx_presenters.lock().unwrap().insert(id, presenter);
    }

    pub fn get_xxx_presenter(&self, id: Uuid) -> Option<Arc<XxxPresenter>> {
        self.xxx_presenters.lock().unwrap().get(&id).cloned()
    }

    pub fn unregister_xxx_presenter(&self, id: Uuid) {
        self.xxx_presenters.lock().unwrap().remove(&id);
    }
}
```

**Step 2: Controllersに追加**

```rust
// crates/javelin-adapter/src/navigation/controllers.rs

use crate::controller::XxxController;

#[derive(Clone)]
pub struct Controllers {
    pub journal_entry: Arc<JournalEntryController>,
    pub search: Arc<SearchController>,
    pub xxx: Arc<XxxController>,  // 追加
    // ... 他のコントローラー
}
```

**Step 3: app_setup.rsでDI設定**

```rust
// crates/javelin/src/app_setup.rs

pub fn setup_controllers(presenter_registry: Arc<PresenterRegistry>) -> Controllers {
    // ... 既存のコントローラー初期化

    let xxx_controller = Arc::new(XxxController::new(Arc::clone(&presenter_registry)));

    Controllers {
        journal_entry,
        search,
        xxx: xxx_controller,  // 追加
        // ... 他のコントローラー
    }
}
```

**Step 4: app_resolver.rsでPageState生成**

```rust
// crates/javelin/src/app_resolver.rs

use javelin_adapter::page_states::XxxPageState;

impl AppResolver {
    pub fn resolve(&self, route: Route) -> Box<dyn PageState> {
        match route {
            Route::Home => Box::new(HomePageState::new()),
            Route::Xxx => Box::new(XxxPageState::new(Arc::clone(&self.presenter_registry))),
            // ... 他のルート
        }
    }
}
```

### 1.6.4 チェックポイント

- [ ] PresenterRegistryに登録メソッド追加済み
- [ ] Controllersに新しいController追加済み
- [ ] app_setup.rsでController初期化済み
- [ ] app_resolver.rsでPageState生成済み
- [ ] コンパイルエラーなし

---

## 1.7 Phase 6: ナビゲーション統合

### 1.7.1 目的

メニューに新しい画面を追加し、ルーティングを設定する。

### 1.7.2 実装対象

| ファイル | 役割 | 配置 |
|------------|------------|------------|
| Route | 画面識別子 | `crates/javelin-adapter/src/navigation/route.rs` |
| MenuPageState | メニュー画面 | `crates/javelin-adapter/src/page_states/{module}/menu.rs` |

### 1.7.3 実装手順

**Step 1: Routeに追加**

```rust
// crates/javelin-adapter/src/navigation/route.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Route {
    Home,
    PrimaryRecordsMenu,
    JournalEntry,
    Xxx,  // 追加
    // ... 他のルート
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Route::Home => write!(f, "Home"),
            Route::Xxx => write!(f, "Xxx"),
            // ... 他のルート
        }
    }
}
```

**Step 2: メニューに追加**

```rust
// crates/javelin-adapter/src/page_states/{module}/menu.rs

impl XxxMenuPageState {
    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                key: "1",
                label: "Xxx Screen",
                route: Route::Xxx,
            },
            // ... 他のメニュー項目
        ]
    }
}
```

### 1.7.4 チェックポイント

- [ ] Routeに新しい画面追加済み
- [ ] メニューに項目追加済み
- [ ] ルーティング動作確認済み
- [ ] コンパイルエラーなし

---

## 1.8 実装チェックリスト

### 1.8.1 Phase 1: ドメイン層

- [ ] ValueObject定義（FromStr/Display実装）
- [ ] Entity定義（業務ロジックメソッド実装）
- [ ] DomainService定義（必要な場合のみ）
- [ ] DomainError定義（thiserror使用）
- [ ] 単体テスト実装（カバレッジ80%以上）
- [ ] コンパイルエラーなし
- [ ] 外部依存なし

### 1.8.2 Phase 2: アプリケーション層

- [ ] RequestDTO定義（TryFrom実装）
- [ ] ResponseDTO定義（From実装）
- [ ] OutputPort定義（必要なメソッド定義）
- [ ] InputPort定義（ユースケース定義）
- [ ] Interactor実装（InputPort実装）
- [ ] ドメインサービス呼び出し（ロジック重複なし）
- [ ] テスト実装（正常系・異常系）
- [ ] コンパイルエラーなし
- [ ] Rust 2024 Edition準拠

### 1.8.3 Phase 3: インフラストラクチャ層

- [ ] QueryService実装（Projection DB参照）
- [ ] ProjectionBuilder実装（必要な場合のみ）
- [ ] Event Storeへの直接アクセスなし
- [ ] コンパイルエラーなし

### 1.8.4 Phase 4: アダプター層

- [ ] Presenter実装（create_channels()実装）
- [ ] Controller実装（page_idでPresenter取得）
- [ ] PageState実装（Presenter作成・登録・破棄）
- [ ] Page実装（チャネルで結果受信）
- [ ] 簡略化実装なし（必ずチャネル通信）
- [ ] コンパイルエラーなし

### 1.8.5 Phase 5: エントリーポイント

- [ ] PresenterRegistryに登録メソッド追加
- [ ] Controllersに新しいController追加
- [ ] app_setup.rsでController初期化
- [ ] app_resolver.rsでPageState生成
- [ ] コンパイルエラーなし

### 1.8.6 Phase 6: ナビゲーション統合

- [ ] Routeに新しい画面追加
- [ ] メニューに項目追加
- [ ] ルーティング動作確認
- [ ] コンパイルエラーなし

---

## 1.9 アンチパターンと禁止事項

### 1.9.1 絶対に避けるべきパターン

| アンチパターン | 問題 | 正しい実装 |
|------------|------------|------------|
| 簡略化実装 | 後で修正が必要になる | 最初から完全実装 |
| TODO残し | 未完成のまま放置 | 完全実装してからコミット |
| チャネル通信省略 | Presenterパターン崩壊 | 必ずチャネル通信実装 |
| ドメインロジック重複 | 一貫性喪失 | ドメインサービス呼び出し |
| 文字列でEnum比較 | 型安全性喪失 | FromStrトレイト使用 |
| DTO変換ユーティリティ | 責務不明確 | DTOにTryFrom実装 |
| async-traitマクロ | Rust 2024不要 | native async fn in traits |
| Box<dyn Future> | 冗長 | -> impl Future |

## 1.10 実装例：仕訳検索画面

### 1.10.1 実装ファイル一覧

```
Phase 1: ドメイン層
  （既存のJournalEntryエンティティを使用）

Phase 2: アプリケーション層
  crates/javelin-application/src/dtos/request/search_criteria.rs
  crates/javelin-application/src/dtos/response/journal_entry_search_result.rs
  crates/javelin-application/src/input_ports/search_journal_entry.rs
  crates/javelin-application/src/output_ports/search_output_port.rs
  crates/javelin-application/src/interactor/journal_entry/search_journal_entry_interactor.rs

Phase 3: インフラストラクチャ層
  crates/javelin-infrastructure/src/read/journal_entry/journal_entry_search_query_service.rs

Phase 4: アダプター層
  crates/javelin-adapter/src/presenter/search_presenter.rs
  crates/javelin-adapter/src/controller/search_controller.rs
  crates/javelin-adapter/src/page_states/primary_records/journal_list.rs
  crates/javelin-adapter/src/views/pages/primary_records/journal_list_page.rs

Phase 5: エントリーポイント
  crates/javelin-adapter/src/navigation/presenter_registry.rs (register_search_presenter追加)
  crates/javelin-adapter/src/navigation/controllers.rs (search追加)
  crates/javelin/src/app_setup.rs (SearchController初期化)
  crates/javelin/src/app_resolver.rs (JournalListPageState生成)

Phase 6: ナビゲーション統合
  crates/javelin-adapter/src/navigation/route.rs (Route::JournalList追加)
  crates/javelin-adapter/src/page_states/primary_records/menu.rs (メニュー項目追加)
```

## 1.11 まとめ

画面追加は以下の順序で実装する：

1. **ドメイン層**: 業務ルールと整合性制約
2. **アプリケーション層**: ユースケースとDTO変換
3. **インフラストラクチャ層**: Query実装（必要な場合）
4. **アダプター層**: Presenter・Controller・PageState・Page
5. **エントリーポイント**: DI設定
6. **ナビゲーション統合**: メニューとルーティング

**重要原則:**
- 内側から外側へ実装
- 簡略化・一時実装は厳禁
- Presenterパターン完全実装
- 既存パターンを完全に踏襲
- Rust 2024 Edition準拠

---

*本章は画面追加の標準ワークフローを定義する。すべての新規画面実装はこの手順に従うこと。*
