/// OutputPort - ユースケース結果の出力
pub trait OutputPort: Send + Sync {
    type Output;

    fn present(&self, output: Self::Output);
}
