use thiserror::Error;

#[derive(Error, Debug)]
pub enum Detail {
  // ローカルファイルのオープンに失敗
  #[error("Failed to open local file {file}; {message}")]
  FailedToOpenLocalFile { file: String, message: String },

  // ストレージの内容が BHT ではない
  #[error("The contents of storage are not for BHT: {message}")]
  FileIsNotContentsOfLMTHTree { message: &'static str },

  // 互換性のないファイルバージョン
  #[error("BHT storage version is incompatible: {0}.{1}")]
  IncompatibleVersion(u8, u8),

  // ペイロードのサイズが大きすぎる
  #[error("Payload size is too large: {size}")]
  TooLargePayload { size: usize },

  // ストレージ破損に対する一般メッセージ
  #[error("DAMAGED STORAGE: {0}")]
  DamagedStorage(String),

  // シーク先の位置が不正
  #[error("DAMAGED STORAGE: incorrect seek position; {message}")]
  IncorrectSeekPosition { message: String },

  // エントリ先頭へのオフセットが間違っている
  #[error("DAMAGED STORAGE: incorrect entry-head offset field; recorded as {expected}, but actually {actual}")]
  IncorrectEntryHeadOffset { expected: u32, actual: u64 },

  // チェックサム検査に失敗
  #[error("DAMAGED STORAGE: checksum verification failed for {length} bytes starting at {at}; expected {expected} but got {actual}")]
  ChecksumVerificationFailed { at: u64, length: u32, expected: u64, actual: u64 },

  // ノードの読み出し位置が不正
  #[error("DAMAGED STORAGE: the read start position is not a correct node boundary")]
  IncorrectNodeBoundary { at: u64 },

  // 内部状態とストレージ上のデータが矛盾している
  #[error("INCONSISTENCY STATE: between the internally state and the data in storage; {message}")]
  InternalStateInconsistency { message: String },

  #[error("I/O error: {source}")]
  Io {
    #[from]
    source: std::io::Error,
  },

  #[error("{source}")]
  Otherwise {
    #[from]
    source: Box<dyn std::error::Error>,
  },
}
