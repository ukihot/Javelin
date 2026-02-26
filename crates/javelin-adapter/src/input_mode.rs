// InputMode - 入力モード管理
// 責務: Vimライクな2モード（変更/非変更）の状態管理

/// 入力モード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    /// 非変更モード（Normal）- hjklで移動、iで変更モードへ
    #[default]
    Normal,
    /// 変更モード（Modify）- 文字入力可能、jjで非変更モードへ
    Modify,
}

/// 変更モード時の入力タイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifyInputType {
    /// ダイレクト書き込み - キーボード入力を直接受け付け
    Direct,
    /// オーバーレイリスト - 画面の8割をオーバーレイして選択肢を表示
    OverlayList,
    /// カレンダー - 日付入力（YYYY-MM-DD形式、2000年代のみ、バリデーション付き）
    Calendar,
    /// 数値のみ - 数字とピリオドのみ入力可能
    NumberOnly,
    /// 二値切り替え - スペースキーでtrue/falseを切り替え
    BooleanToggle,
}

impl ModifyInputType {
    /// 入力タイプの表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            ModifyInputType::Direct => "直接入力",
            ModifyInputType::OverlayList => "リスト選択",
            ModifyInputType::Calendar => "カレンダー",
            ModifyInputType::NumberOnly => "数値入力",
            ModifyInputType::BooleanToggle => "切替",
        }
    }

    /// 文字が入力可能かどうかを判定
    pub fn is_char_allowed(&self, ch: char) -> bool {
        match self {
            ModifyInputType::Direct => true,
            ModifyInputType::OverlayList => false, // リスト選択のみ
            ModifyInputType::Calendar => ch.is_ascii_digit(), // 数字のみ入力可能
            ModifyInputType::NumberOnly => {
                // 0-9の数字のみ許可（カンマは自動表示するため入力不可）
                ch.is_ascii_digit()
            }
            ModifyInputType::BooleanToggle => ch == ' ', // スペースキーのみ
        }
    }

    /// 日付入力のバリデーション（Calendar用）
    /// 入力: 8桁の数字文字列（YYYYMMDD）
    /// 戻り値: (バリデーション成功, エラーメッセージ)
    pub fn validate_date_input(input: &str) -> (bool, Option<&'static str>) {
        // 8桁チェック
        if input.len() != 8 {
            return (false, Some("8桁で入力してください"));
        }

        // 数字のみチェック
        if !input.chars().all(|c| c.is_ascii_digit()) {
            return (false, Some("数字のみ入力可能です"));
        }

        // 年月日を抽出
        let year: u32 = match input[0..4].parse() {
            Ok(y) => y,
            Err(_) => return (false, Some("年が不正です")),
        };
        let month: u32 = match input[4..6].parse() {
            Ok(m) => m,
            Err(_) => return (false, Some("月が不正です")),
        };
        let day: u32 = match input[6..8].parse() {
            Ok(d) => d,
            Err(_) => return (false, Some("日が不正です")),
        };

        // 年チェック（2000年代のみ）
        if !(2000..3000).contains(&year) {
            return (false, Some("2000年代の日付を入力してください"));
        }

        // 月チェック（1-12）
        if !(1..=12).contains(&month) {
            return (false, Some("月は01-12の範囲で入力してください"));
        }

        // 日チェック
        let max_day = days_in_month(year, month);
        if day < 1 || day > max_day {
            return (false, Some("日が月の範囲を超えています"));
        }

        (true, None)
    }

    /// 日付入力を表示用にフォーマット（YYYY-MM-DD）
    pub fn format_date_input(input: &str) -> String {
        let len = input.len();

        if len <= 4 {
            // 4桁以下: そのまま表示
            input.to_string()
        } else if len <= 6 {
            // 5-6桁: YYYY-MM
            format!("{}-{}", &input[0..4], &input[4..])
        } else {
            // 7-8桁: YYYY-MM-DD
            format!("{}-{}-{}", &input[0..4], &input[4..6], &input[6..])
        }
    }
}

/// 指定された年月の日数を返す（うるう年考慮）
fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// うるう年判定
fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// 仕訳編集区分
///
/// 月次決算確報作成規程の仕訳行為区分に基づく編集モード
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JournalEntryEditMode {
    /// 新規起票仕訳 - 経済事象の第一次認識として記録される仕訳
    #[default]
    NewEntry,
    /// 取消仕訳 - 既存仕訳の効力を無効化する目的で計上される仕訳
    Cancellation,
    /// 反対仕訳 - 既存残高または期間帰属を反転させるための仕訳
    Reversal,
    /// 追加仕訳 - 計上不足または後日判明事項を補正する仕訳
    Additional,
    /// 再分類仕訳 - 測定額を変更せず表示区分のみ変更する仕訳
    Reclassification,
    /// 洗替仕訳 - 既存評価額を一旦消去し再評価する仕訳
    Replacement,
}

impl JournalEntryEditMode {
    /// 既存伝票の参照が必要かどうか
    pub fn requires_reference(&self) -> bool {
        !matches!(self, JournalEntryEditMode::NewEntry)
    }

    /// 表示名を取得
    pub fn display_name(&self) -> &str {
        match self {
            JournalEntryEditMode::NewEntry => "1.新規起票",
            JournalEntryEditMode::Cancellation => "2.取消",
            JournalEntryEditMode::Reversal => "3.反対",
            JournalEntryEditMode::Additional => "4.追加",
            JournalEntryEditMode::Reclassification => "5.再分類",
            JournalEntryEditMode::Replacement => "6.洗替",
        }
    }

    /// 次のモードへ切り替え
    pub fn next(&self) -> Self {
        match self {
            JournalEntryEditMode::NewEntry => JournalEntryEditMode::Cancellation,
            JournalEntryEditMode::Cancellation => JournalEntryEditMode::Reversal,
            JournalEntryEditMode::Reversal => JournalEntryEditMode::Additional,
            JournalEntryEditMode::Additional => JournalEntryEditMode::Reclassification,
            JournalEntryEditMode::Reclassification => JournalEntryEditMode::Replacement,
            JournalEntryEditMode::Replacement => JournalEntryEditMode::NewEntry,
        }
    }

    /// 前のモードへ切り替え
    pub fn previous(&self) -> Self {
        match self {
            JournalEntryEditMode::NewEntry => JournalEntryEditMode::Replacement,
            JournalEntryEditMode::Cancellation => JournalEntryEditMode::NewEntry,
            JournalEntryEditMode::Reversal => JournalEntryEditMode::Cancellation,
            JournalEntryEditMode::Additional => JournalEntryEditMode::Reversal,
            JournalEntryEditMode::Reclassification => JournalEntryEditMode::Additional,
            JournalEntryEditMode::Replacement => JournalEntryEditMode::Reclassification,
        }
    }

    /// すべてのモードを取得
    pub fn all() -> Vec<Self> {
        vec![
            JournalEntryEditMode::NewEntry,
            JournalEntryEditMode::Cancellation,
            JournalEntryEditMode::Reversal,
            JournalEntryEditMode::Additional,
            JournalEntryEditMode::Reclassification,
            JournalEntryEditMode::Replacement,
        ]
    }
}

impl InputMode {
    /// 変更モードかどうか
    pub fn is_modify(&self) -> bool {
        matches!(self, InputMode::Modify)
    }

    /// 非変更モードかどうか
    pub fn is_normal(&self) -> bool {
        matches!(self, InputMode::Normal)
    }

    /// モードを切り替え
    pub fn toggle(&mut self) {
        *self = match self {
            InputMode::Normal => InputMode::Modify,
            InputMode::Modify => InputMode::Normal,
        };
    }

    /// 変更モードに切り替え
    pub fn enter_modify(&mut self) {
        *self = InputMode::Modify;
    }

    /// 非変更モードに切り替え
    pub fn enter_normal(&mut self) {
        *self = InputMode::Normal;
    }

    /// モード名を取得
    pub fn name(&self) -> &str {
        match self {
            InputMode::Normal => "NORMAL",
            InputMode::Modify => "MODIFY",
        }
    }
}

/// jjエスケープ検出器
/// 変更モードで"jj"を入力すると非変更モードに戻る
#[derive(Debug, Default)]
pub struct JjEscapeDetector {
    last_char: Option<char>,
}

impl JjEscapeDetector {
    pub fn new() -> Self {
        Self { last_char: None }
    }

    /// 文字を処理し、jjが検出されたかを返す
    /// 戻り値: (jjが検出されたか, 入力すべき文字)
    ///
    /// 注意: j以外の文字が来た時、保留中のjがある場合は
    /// has_pending_j()で確認してflush_pending()で取得する必要がある
    pub fn process(&mut self, ch: char) -> (bool, Option<char>) {
        if ch == 'j' {
            if self.last_char == Some('j') {
                // jjが検出された
                self.last_char = None;
                (true, None)
            } else {
                // 最初のj
                self.last_char = Some('j');
                (false, None) // まだ入力しない
            }
        } else {
            // j以外の文字 - 保留中のjはそのまま残し、今回の文字だけ返す
            // 呼び出し側がhas_pending_j()とflush_pending()で処理する
            (false, Some(ch))
        }
    }

    /// リセット（モード切り替え時に呼ぶ）
    pub fn reset(&mut self) {
        self.last_char = None;
    }

    /// 保留中のjがあるか
    pub fn has_pending_j(&self) -> bool {
        self.last_char == Some('j')
    }

    /// 保留中のjをフラッシュ（確定）
    pub fn flush_pending(&mut self) -> Option<char> {
        let result = self.last_char;
        self.last_char = None;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_mode_toggle() {
        let mut mode = InputMode::Normal;
        assert!(mode.is_normal());

        mode.toggle();
        assert!(mode.is_modify());

        mode.toggle();
        assert!(mode.is_normal());
    }

    #[test]
    fn test_modify_input_type_char_allowed() {
        // Direct: すべての文字が許可される
        assert!(ModifyInputType::Direct.is_char_allowed('a'));
        assert!(ModifyInputType::Direct.is_char_allowed('1'));
        assert!(ModifyInputType::Direct.is_char_allowed('あ'));

        // OverlayList: 文字入力は許可されない
        assert!(!ModifyInputType::OverlayList.is_char_allowed('a'));
        assert!(!ModifyInputType::OverlayList.is_char_allowed('1'));

        // Calendar: 数字のみ許可
        assert!(!ModifyInputType::Calendar.is_char_allowed('a'));
        assert!(ModifyInputType::Calendar.is_char_allowed('1'));
        assert!(!ModifyInputType::Calendar.is_char_allowed('-'));

        // NumberOnly: 0-9の数字のみ許可（カンマ、ピリオド、マイナスは不可）
        assert!(ModifyInputType::NumberOnly.is_char_allowed('0'));
        assert!(ModifyInputType::NumberOnly.is_char_allowed('9'));
        assert!(!ModifyInputType::NumberOnly.is_char_allowed('.'));
        assert!(!ModifyInputType::NumberOnly.is_char_allowed('-'));
        assert!(!ModifyInputType::NumberOnly.is_char_allowed(','));
        assert!(!ModifyInputType::NumberOnly.is_char_allowed('a'));
        assert!(!ModifyInputType::NumberOnly.is_char_allowed('あ'));
    }

    #[test]
    fn test_validate_date_input() {
        // 正常な日付
        assert!(ModifyInputType::validate_date_input("20240315").0);
        assert!(ModifyInputType::validate_date_input("20000101").0);
        assert!(ModifyInputType::validate_date_input("29991231").0);

        // うるう年
        assert!(ModifyInputType::validate_date_input("20240229").0); // 2024年はうるう年
        assert!(!ModifyInputType::validate_date_input("20230229").0); // 2023年は平年
        assert!(ModifyInputType::validate_date_input("20000229").0); // 2000年はうるう年
        assert!(!ModifyInputType::validate_date_input("21000229").0); // 2100年は平年

        // 桁数エラー
        assert!(!ModifyInputType::validate_date_input("2024031").0);
        assert!(!ModifyInputType::validate_date_input("202403155").0);

        // 数字以外
        assert!(!ModifyInputType::validate_date_input("2024-03-15").0);
        assert!(!ModifyInputType::validate_date_input("abcd0315").0);

        // 年の範囲外
        assert!(!ModifyInputType::validate_date_input("19991231").0);
        assert!(!ModifyInputType::validate_date_input("30000101").0);

        // 月の範囲外
        assert!(!ModifyInputType::validate_date_input("20240015").0);
        assert!(!ModifyInputType::validate_date_input("20241315").0);

        // 日の範囲外
        assert!(!ModifyInputType::validate_date_input("20240100").0);
        assert!(!ModifyInputType::validate_date_input("20240132").0);
        assert!(!ModifyInputType::validate_date_input("20240431").0); // 4月は30日まで
        assert!(!ModifyInputType::validate_date_input("20230229").0); // 平年の2月は28日まで
    }

    #[test]
    fn test_format_date_input() {
        assert_eq!(ModifyInputType::format_date_input("2"), "2");
        assert_eq!(ModifyInputType::format_date_input("20"), "20");
        assert_eq!(ModifyInputType::format_date_input("202"), "202");
        assert_eq!(ModifyInputType::format_date_input("2024"), "2024");
        assert_eq!(ModifyInputType::format_date_input("20240"), "2024-0");
        assert_eq!(ModifyInputType::format_date_input("202403"), "2024-03");
        assert_eq!(ModifyInputType::format_date_input("2024031"), "2024-03-1");
        assert_eq!(ModifyInputType::format_date_input("20240315"), "2024-03-15");
        assert_eq!(ModifyInputType::format_date_input("20260220"), "2026-02-20");
    }

    #[test]
    fn test_days_in_month() {
        // 31日の月
        assert_eq!(days_in_month(2024, 1), 31);
        assert_eq!(days_in_month(2024, 3), 31);
        assert_eq!(days_in_month(2024, 5), 31);
        assert_eq!(days_in_month(2024, 7), 31);
        assert_eq!(days_in_month(2024, 8), 31);
        assert_eq!(days_in_month(2024, 10), 31);
        assert_eq!(days_in_month(2024, 12), 31);

        // 30日の月
        assert_eq!(days_in_month(2024, 4), 30);
        assert_eq!(days_in_month(2024, 6), 30);
        assert_eq!(days_in_month(2024, 9), 30);
        assert_eq!(days_in_month(2024, 11), 30);

        // 2月（うるう年）
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2000, 2), 29);

        // 2月（平年）
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(2100, 2), 28);
    }

    #[test]
    fn test_is_leap_year() {
        // うるう年
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));

        // 平年
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(2200));
        assert!(!is_leap_year(2300));
    }

    #[test]
    fn test_jj_escape_detector() {
        let mut detector = JjEscapeDetector::new();

        // 通常の文字
        let (escaped, ch) = detector.process('a');
        assert!(!escaped);
        assert_eq!(ch, Some('a'));

        // 最初のj
        let (escaped, ch) = detector.process('j');
        assert!(!escaped);
        assert_eq!(ch, None);

        // 2番目のj（jjエスケープ）
        let (escaped, ch) = detector.process('j');
        assert!(escaped);
        assert_eq!(ch, None);

        // jの後に別の文字
        let (escaped, ch) = detector.process('j');
        assert!(!escaped);
        assert_eq!(ch, None);

        let (escaped, ch) = detector.process('a');
        assert!(!escaped);
        assert_eq!(ch, Some('a'));

        // 保留中のjをフラッシュ
        assert!(detector.has_pending_j());
        let flushed = detector.flush_pending();
        assert_eq!(flushed, Some('j'));
    }
}
