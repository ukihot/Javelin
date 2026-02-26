// Utils - View層のユーティリティマクロ
// 責務: 共通フォーマット処理

/// 数値をカンマ区切りでフォーマットするマクロ
#[macro_export]
macro_rules! format_number {
    ($num:expr) => {{
        let num: f64 = $num;
        let num_str = format!("{:.0}", num);
        let mut result = String::new();
        let chars: Vec<char> = num_str.chars().collect();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*ch);
        }
        result
    }};
}

/// 金額をフォーマットするマクロ（ゼロは"---"表示）
#[macro_export]
macro_rules! format_amount {
    ($amount:expr) => {{
        let amount: f64 = $amount;
        if amount == 0.0 {
            "---".to_string()
        } else {
            $crate::format_number!(amount)
        }
    }};
    ($amount:expr, $width:expr) => {{
        let amount: f64 = $amount;
        if amount == 0.0 {
            "---".to_string()
        } else {
            format!("{:>width$}", $crate::format_number!(amount), width = $width)
        }
    }};
}

/// 残高をフォーマットするマクロ（マイナスは括弧表示）
#[macro_export]
macro_rules! format_balance {
    ($balance:expr) => {{
        let balance: f64 = $balance;
        if balance == 0.0 {
            "---".to_string()
        } else if balance > 0.0 {
            $crate::format_number!(balance)
        } else {
            format!("({})", $crate::format_number!(balance.abs()))
        }
    }};
    ($balance:expr, $width:expr) => {{
        let balance: f64 = $balance;
        if balance == 0.0 {
            "---".to_string()
        } else if balance > 0.0 {
            format!("{:>width$}", $crate::format_number!(balance), width = $width)
        } else {
            format!(
                "{:>width$}",
                format!("({})", $crate::format_number!(balance.abs())),
                width = $width
            )
        }
    }};
}

/// テキストを切り詰めるマクロ（レトロな省略表示）
#[macro_export]
macro_rules! truncate_text {
    ($text:expr, $max_len:expr) => {{
        if $text.chars().count() > $max_len {
            let truncated: String = $text.chars().take($max_len - 2).collect();
            format!("{}…", truncated)
        } else {
            $text.to_string()
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_format_number() {
        assert_eq!(format_number!(1000.0), "1,000");
        assert_eq!(format_number!(1000000.0), "1,000,000");
        assert_eq!(format_number!(123456789.0), "123,456,789");
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount!(0.0), "---");
        assert_eq!(format_amount!(1000.0), "1,000");
        assert_eq!(format_amount!(1000.0, 10), "     1,000");
    }

    #[test]
    fn test_format_balance() {
        assert_eq!(format_balance!(0.0), "---");
        assert_eq!(format_balance!(1000.0), "1,000");
        assert_eq!(format_balance!(-1000.0), "(1,000)");
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text!("Hello", 10), "Hello");
        assert_eq!(truncate_text!("Hello World", 8), "Hello …");
    }
}
