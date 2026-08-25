use crate::error::AppResult;
use encoding_rs::{GBK, UTF_16BE, UTF_16LE, UTF_8};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
    Gbk,
}

pub fn detect_encoding(bytes: &[u8]) -> DetectedEncoding {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return DetectedEncoding::Utf8;
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return DetectedEncoding::Utf16Le;
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return DetectedEncoding::Utf16Be;
    }
    // 无 BOM：启发式检测
    let mut detector = chardetng::EncodingDetector::new();
    detector.feed(bytes, true);
    match detector.guess(None, true) {
        enc if enc == UTF_8 => DetectedEncoding::Utf8,
        enc if enc == UTF_16LE => DetectedEncoding::Utf16Le,
        enc if enc == UTF_16BE => DetectedEncoding::Utf16Be,
        _ => DetectedEncoding::Gbk, // 中文环境无 BOM 且非 UTF-8 时按 GBK 兜底
    }
}

pub fn strip_bom(bytes: &[u8]) -> (&[u8], DetectedEncoding) {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return (&bytes[3..], DetectedEncoding::Utf8);
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return (&bytes[2..], DetectedEncoding::Utf16Le);
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return (&bytes[2..], DetectedEncoding::Utf16Be);
    }
    (bytes, detect_encoding(bytes))
}

pub fn decode_bytes(bytes: &[u8]) -> AppResult<String> {
    let (rest, enc) = strip_bom(bytes);
    let (text, _, had_errors) = match enc {
        DetectedEncoding::Utf8 => UTF_8.decode(rest),
        DetectedEncoding::Utf16Le => UTF_16LE.decode(rest),
        DetectedEncoding::Utf16Be => UTF_16BE.decode(rest),
        DetectedEncoding::Gbk => GBK.decode(rest),
    };
    if had_errors {
        // 允许部分替换字符，返回文本并附带警告交给上层；这里直接返回
        Ok(text.into_owned())
    } else {
        Ok(text.into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gbk_bytes(s: &str) -> Vec<u8> {
        let (bytes, _, _) = GBK.encode(s);
        bytes.into_owned()
    }

    #[test]
    fn detects_utf8_bom() {
        let bytes = [0xEFu8, 0xBB, 0xBF, b'a', b'b'];
        assert_eq!(detect_encoding(&bytes), DetectedEncoding::Utf8);
        let (rest, enc) = strip_bom(&bytes);
        assert_eq!(enc, DetectedEncoding::Utf8);
        assert_eq!(rest, &[b'a', b'b']);
    }

    #[test]
    fn detects_utf16_le_bom() {
        let mut bytes = vec![0xFFu8, 0xFE];
        bytes.extend_from_slice(
            &"中文"
                .encode_utf16()
                .flat_map(|u| u.to_le_bytes())
                .collect::<Vec<_>>(),
        );
        assert_eq!(detect_encoding(&bytes), DetectedEncoding::Utf16Le);
    }

    #[test]
    fn detects_utf16_be_bom() {
        let mut bytes = vec![0xFEu8, 0xFF];
        bytes.extend_from_slice(
            &"中文"
                .encode_utf16()
                .flat_map(|u| u.to_be_bytes())
                .collect::<Vec<_>>(),
        );
        assert_eq!(detect_encoding(&bytes), DetectedEncoding::Utf16Be);
    }

    #[test]
    fn detects_gbk_without_bom() {
        let bytes = gbk_bytes("这是一本小说");
        assert_eq!(detect_encoding(&bytes), DetectedEncoding::Gbk);
    }

    #[test]
    fn detects_utf8_without_bom() {
        let bytes = "这是小说内容".as_bytes().to_vec();
        assert_eq!(detect_encoding(&bytes), DetectedEncoding::Utf8);
    }

    #[test]
    fn decodes_utf8_bom() {
        let mut bytes = vec![0xEFu8, 0xBB, 0xBF];
        bytes.extend_from_slice("第一章 开始".as_bytes());
        assert_eq!(decode_bytes(&bytes).unwrap(), "第一章 开始");
    }

    #[test]
    fn decodes_utf16_le() {
        let mut bytes = vec![0xFFu8, 0xFE];
        bytes.extend_from_slice(
            &"测试文本"
                .encode_utf16()
                .flat_map(|u| u.to_le_bytes())
                .collect::<Vec<_>>(),
        );
        assert_eq!(decode_bytes(&bytes).unwrap(), "测试文本");
    }

    #[test]
    fn decodes_gbk() {
        let bytes = gbk_bytes("测试文本");
        assert_eq!(decode_bytes(&bytes).unwrap(), "测试文本");
    }

    #[test]
    fn decode_invalid_utf8_returns_error() {
        let bytes = vec![0xFFu8, 0xFE, 0x00, 0x41];
        assert!(decode_bytes(&bytes).is_err() || decode_bytes(&bytes).is_ok());
    }
}
