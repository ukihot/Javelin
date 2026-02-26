# 画面テンプレート (Templates)

基幹システムとして、汎用的で再利用可能な画面テンプレートを提供します。

## 概要

このディレクトリには、共通の画面パターンを実装した汎用テンプレートが含まれています。
新しいマスタ画面や設定画面を追加する際は、これらのテンプレートを使用することで、
一貫性のあるUIと最小限のコードで実装できます。

## 利用可能なテンプレート

### 1. MasterListTemplate

マスタデータの一覧表示用テンプレート。ページング、選択、ソートなどの機能を提供します。

**用途:**
- 勘定科目マスタ一覧
- 会社マスタ一覧
- 補助科目マスタ一覧
- その他のマスタデータ一覧

**使用方法:**

```rust
use crate::views::layouts::templates::{MasterListItem, MasterListTemplate};

// 1. ViewModelにMasterListItemトレイトを実装
impl MasterListItem for YourItemViewModel {
    fn headers() -> Vec<&'static str> {
        vec!["コード", "名称", "備考"]
    }

    fn column_widths() -> Vec<Constraint> {
        vec![
            Constraint::Length(10),
            Constraint::Min(20),
            Constraint::Length(30),
        ]
    }

    fn to_row(&self) -> Vec<String> {
        vec![
            self.code.clone(),
            self.name.clone(),
            self.note.clone(),
        ]
    }
}

// 2. Pageでテンプレートを使用
pub struct YourMasterPage {
    template: MasterListTemplate<YourItemViewModel>,
}

impl YourMasterPage {
    pub fn new() -> Self {
        Self {
            template: MasterListTemplate::new("マスタ名"),
        }
    }

    pub fn set_data(
        &mut self,
        items: Vec<YourItemViewModel>,
        current_page: usize,
        selected_index: usize,
    ) {
        self.template.set_data(items, current_page, selected_index);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.template.render(frame);
    }
}
```

### 2. SettingsTemplate

設定画面の表示用テンプレート。キー・バリュー形式で設定項目を表示します。

**用途:**
- アプリケーション設定
- ユーザー設定
- システム設定
- その他の設定画面

**使用方法:**

```rust
use crate::views::layouts::templates::{SettingsItem, SettingsTemplate};

// 1. ViewModelにSettingsItemトレイトを実装
impl SettingsItem for YourSettingsViewModel {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("設定項目1".to_string(), self.value1.to_string()),
            ("設定項目2".to_string(), self.value2.to_string()),
            ("設定項目3".to_string(), self.value3.to_string()),
        ]
    }
}

// 2. Pageでテンプレートを使用
pub struct YourSettingsPage {
    template: SettingsTemplate<YourSettingsViewModel>,
}

impl YourSettingsPage {
    pub fn new() -> Self {
        Self {
            template: SettingsTemplate::new("設定名")
                .with_footer("[Esc] 戻る | [Enter] 編集"),
        }
    }

    pub fn set_data(&mut self, view_model: YourSettingsViewModel) {
        self.template.set_data(view_model);
    }

    pub fn render(&self, frame: &mut Frame) {
        self.template.render(frame);
    }
}
```

## テンプレートの利点

1. **一貫性**: すべてのマスタ画面で同じUIパターンを使用
2. **保守性**: テンプレートを修正すれば、すべての画面に反映される
3. **開発効率**: 新しい画面を数行のコードで実装可能
4. **品質**: テストされた共通コードを再利用
5. **拡張性**: 新しいテンプレートを追加して、他のパターンにも対応可能

## 実装例

- `account_master_page_refactored.rs` - MasterListTemplateの使用例
- `application_settings_page_refactored.rs` - SettingsTemplateの使用例
- `subsidiary_account_master_page_refactored.rs` - MasterListTemplateの使用例

## 今後の拡張

以下のテンプレートを追加予定：

- `master_detail_template.rs` - マスタ詳細/編集画面
- `form_template.rs` - 汎用フォーム画面
- `dialog_template.rs` - ダイアログ/モーダル画面
- `wizard_template.rs` - ウィザード形式の画面

## アーキテクチャ原則

- **Clean Architecture**: テンプレートはアダプター層に属し、ドメイン層に依存しない
- **SOLID原則**: 単一責任、開放閉鎖、依存性逆転の原則に従う
- **DRY原則**: 共通コードを重複させない
- **型安全性**: トレイトを使用して、コンパイル時に型チェック
