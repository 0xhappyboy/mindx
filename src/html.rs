// Character classification and HTML cleaning utilities.
pub fn is_ideographic(c: char) -> bool {
    let u = c as u32;
    (0x4E00..=0x9FFF).contains(&u)        // CJK Unified Ideographs
        || (0x3400..=0x4DBF).contains(&u) // CJK Extension A
        || (0xF900..=0xFAFF).contains(&u) // CJK Compatibility Ideographs
}
// True for any alphabetic character that is not ideographic.
pub fn is_letter(c: char) -> bool {
    c.is_alphabetic() && !is_ideographic(c)
}
// Backwards-compatible alias. Prefer is_ideographic in new code.
pub fn is_cjk(c: char) -> bool {
    is_ideographic(c)
}
pub fn strip_script_style(html: &str) -> String {
    let chars: Vec<char> = html.chars().collect();
    let lower: Vec<char> = html.to_lowercase().chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut in_s = false;
    while i < chars.len() {
        if !in_s && starts_with(&lower, i, "<script") {
            in_s = true;
            i += 7;
            continue;
        }
        if !in_s && starts_with(&lower, i, "<style") {
            in_s = true;
            i += 6;
            continue;
        }
        if in_s {
            if starts_with(&lower, i, "</script>") {
                in_s = false;
                i += 9;
                continue;
            }
            if starts_with(&lower, i, "</style>") {
                in_s = false;
                i += 8;
                continue;
            }
            i += 1;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}
fn starts_with(v: &[char], i: usize, pat: &str) -> bool {
    let p: Vec<char> = pat.chars().collect();
    if i + p.len() > v.len() {
        return false;
    }
    v[i..i + p.len()] == p[..]
}
pub fn strip_tags(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
            out.push(' ');
        } else if c == '>' {
            in_tag = false;
            out.push(' ');
        } else if !in_tag {
            out.push(c);
        }
    }
    out.replace("&nbsp;", " ").replace("&amp;", "&")
}
pub fn split_sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    for c in text.chars() {
        if "。！？.!?\n".contains(c) {
            if !buf.is_empty() {
                out.push(std::mem::take(&mut buf));
            }
        } else {
            buf.push(c);
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}
