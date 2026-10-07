use crate::core::MindX;
mod associate;
mod core;
mod decay;
mod equivalence;
mod example;
mod hebbian;
mod html;
mod induction;
mod input;
mod number;
mod query;
mod rule_def;
mod stats;
mod tokenize;
mod vector;
mod word_discovery;
pub fn demo() {
    println!("{}", MindX::input("规则 乘法 表示 x * 乘"));
}
#[cfg(test)]
mod tests {
    use super::*;
    // MindX uses a thread-local singleton. Running each test in its own
    // thread gives every test a fresh InnerMind.
    fn fresh<F: FnOnce() + Send + 'static>(f: F) {
        std::thread::spawn(f).join().unwrap();
    }
    // ---------- Rule induction ----------
    #[test]
    fn show_rule_induction_multiplication() {
        fresh(|| {
            println!("--- rule induction: multiplication ---");
            println!("{}", MindX::input("规则 乘法 表示 x * 乘"));
            println!("{}", MindX::input("3 x 7 = 21"));
            println!("{}", MindX::input("5 * 7 = 35"));
            println!("{}", MindX::input("2 乘 7 = 14"));
            println!("12 x 13 => {}", MindX::input("12 x 13"));
        });
    }
    #[test]
    fn show_rule_induction_addition() {
        fresh(|| {
            println!("--- rule induction: addition ---");
            println!("{}", MindX::input("规则 加法 表示 + 加"));
            println!("{}", MindX::input("3 + 7 = 10"));
            println!("{}", MindX::input("5 加 5 = 10"));
            println!("{}", MindX::input("2 + 8 = 10"));
            println!("12 + 13 => {}", MindX::input("12 + 13"));
        });
    }
    #[test]
    fn show_computation_outside_examples() {
        fresh(|| {
            println!("--- computation outside examples ---");
            MindX::input("规则 乘法 表示 x * 乘");
            MindX::input("3 x 7 = 21");
            MindX::input("5 * 7 = 35");
            MindX::input("2 乘 7 = 14");
            println!("7 x 8    => {}", MindX::input("7 x 8"));
            println!("9 * 9    => {}", MindX::input("9 * 9"));
            println!("11 乘 11 => {}", MindX::input("11 乘 11"));
        });
    }
    // ---------- Equivalence ----------
    #[test]
    fn show_equivalence_declaration() {
        fresh(|| {
            println!("--- equivalence declaration ---");
            println!("{}", MindX::input("x 和 * 是同一个东西"));
        });
    }
    // ---------- Word discovery ----------
    #[test]
    fn show_word_discovery_from_repetition() {
        fresh(|| {
            println!("--- word discovery from repetition ---");
            for _ in 0..5 {
                MindX::input("苹果 是 水果");
            }
            println!("query apple => {}", MindX::input("苹果"));
        });
    }
    // ---------- Association ----------
    #[test]
    fn show_association_unknown_input() {
        fresh(|| {
            println!("--- association: unknown input ---");
            println!("{}", MindX::input("zzz_unknown_symbol_zzz"));
        });
    }
    #[test]
    fn show_empty_input() {
        fresh(|| {
            println!("--- empty input ---");
            println!("[{}]", MindX::input(""));
        });
    }
    // ---------- HTML ----------
    #[test]
    fn show_html_strip_script_and_style() {
        println!("--- html strip script/style ---");
        let html =
            "<html><head><style>body{}</style><script>1+1</script></head><body>hello</body></html>";
        println!("{}", html::strip_script_style(html));
    }
    #[test]
    fn show_html_strip_tags() {
        println!("--- html strip tags ---");
        let html = "<p>hello <b>world</b></p>";
        println!("{}", html::strip_tags(html));
    }
    #[test]
    fn show_html_split_sentences() {
        println!("--- html split sentences ---");
        let sents = html::split_sentences("a. b! c?");
        println!("{:?}", sents);
    }
    // ---------- Character classification ----------
    #[test]
    fn show_character_classification() {
        println!("--- character classification ---");
        println!("is_ideographic('中') = {}", html::is_ideographic('中'));
        println!("is_ideographic('漢') = {}", html::is_ideographic('漢'));
        println!("is_letter('a')       = {}", html::is_letter('a'));
        println!("is_letter('я')       = {}", html::is_letter('я'));
        println!("is_letter('α')       = {}", html::is_letter('α'));
        println!("is_letter('中')      = {}", html::is_letter('中'));
        println!("is_ideographic('a')  = {}", html::is_ideographic('a'));
    }
    // ---------- Tokenization ----------
    #[test]
    fn show_tokenize_latin() {
        let mind = input::InnerMind::new();
        let tokens = tokenize::tokenize(&mind, "hello world");
        println!("--- tokenize latin ---");
        println!("{:?}", tokens);
    }
    #[test]
    fn show_tokenize_math_symbols() {
        let mind = input::InnerMind::new();
        let tokens = tokenize::tokenize(&mind, "3 * 7");
        println!("--- tokenize math symbols ---");
        println!("{:?}", tokens);
    }
    #[test]
    fn show_tokenize_ideographic_single() {
        let mind = input::InnerMind::new();
        let tokens = tokenize::tokenize(&mind, "苹果");
        println!("--- tokenize ideographic (no discovery yet) ---");
        println!("{:?}", tokens);
    }
    #[test]
    fn show_tokenize_cyrillic() {
        let mind = input::InnerMind::new();
        let tokens = tokenize::tokenize(&mind, "привет мир");
        println!("--- tokenize cyrillic ---");
        println!("{:?}", tokens);
    }
    // ---------- Vector ----------
    #[test]
    fn show_atom_deterministic() {
        let mind = input::InnerMind::new();
        let a = vector::atom(&mind, "hello");
        let b = vector::atom(&mind, "hello");
        let c = vector::atom(&mind, "world");
        println!("--- atom determinism ---");
        println!("atom(hello) == atom(hello): {}", a == b);
        println!("atom(hello) == atom(world): {}", a == c);
    }
    #[test]
    fn show_cosine_self() {
        let mind = input::InnerMind::new();
        let a = vector::atom(&mind, "x");
        let s = associate::cosine(&a, &a);
        println!("--- cosine(x, x) ---");
        println!("{}", s);
    }
    // ---------- Program synthesis ----------
    #[test]
    fn show_program_eval_mul() {
        use rule_def::Program;
        let p = Program::Mul(Box::new(Program::Var(0)), Box::new(Program::Var(1)));
        println!("--- program eval: mul ---");
        println!("{:?}", induction::eval(&p, &[3.0, 4.0]));
    }
    #[test]
    fn show_program_eval_div_by_zero() {
        use rule_def::Program;
        let p = Program::Div(Box::new(Program::Const(1.0)), Box::new(Program::Const(0.0)));
        println!("--- program eval: div by zero ---");
        println!("{:?}", induction::eval(&p, &[]));
    }
    // ---------- Decay ----------
    #[test]
    fn show_decay_removes_tiny_weights() {
        let mut mind = input::InnerMind::new();
        mind.w[0].insert(1, 1e-7);
        println!("--- decay: tiny weight ---");
        println!("before: contains key 1 = {}", mind.w[0].contains_key(&1));
        decay::decay_all(&mut mind);
        println!("after:  contains key 1 = {}", mind.w[0].contains_key(&1));
    }
    // ---------- Stats ----------
    #[test]
    fn show_stats() {
        let mind = input::InnerMind::new();
        println!("--- stats ---");
        println!("{}", stats::stats(&mind));
    }
    // ---------- Stress: multilingual input ----------
    #[test]
    fn show_stress_10_rounds() {
        fresh(|| {
            println!("--- stress: multilingual input ---");
            // Sentences from many languages. No language is privileged.
            let pool = [
                // Chinese
                "苹果 是 水果",
                "苹果 公司 是 科技 公司",
                "香蕉 是 水果",
                "水果 富含 维生素",
                "维生素 对 身体 有益",
                "科技 公司 有 股价",
                "股价 上涨 说明 公司 表现 好",
                "狗 是 动物",
                "猫 是 动物",
                "动物 需要 食物",
                "食物 提供 能量",
                "水 是 生命 必需",
                "太阳 提供 光 和 热",
                "地球 围绕 太阳 转",
                "月亮 围绕 地球 转",
                // English
                "apple is a fruit",
                "apple is a tech company",
                "banana is a fruit",
                "fruit is rich in vitamins",
                "vitamins are good for health",
                "tech companies have stock prices",
                "stock price rises when company performs well",
                "dog is an animal",
                "cat is an animal",
                "animals need food",
                "food provides energy",
                "water is essential for life",
                "the sun provides light and heat",
                "the earth revolves around the sun",
                "the moon revolves around the earth",
                // Russian
                "яблоко это фрукт",
                "банан это фрукт",
                "фрукты богаты витаминами",
                "витамины полезны для здоровья",
                "собака это животное",
                "кошка это животное",
                "животные нуждаются в еде",
                // Greek
                "μήλο είναι φρούτο",
                "μπανάνα είναι φρούτο",
                "φρούτα είναι πλούσια σε βιταμίνες",
                "σκύλος είναι ζώο",
                "γάτα είναι ζώο",
                // Japanese
                "りんご は 果物 です",
                "バナナ は 果物 です",
                "果物 は ビタミン が 豊富 です",
                "犬 は 動物 です",
                "猫 は 動物 です",
                // Korean
                "사과 는 과일 이다",
                "바나나 는 과일 이다",
                "과일 은 비타민 이 풍부 하다",
                "개 는 동물 이다",
                "고양이 는 동물 이다",
                // Arabic
                "تفاحة هي فاكهة",
                "موز هو فاكهة",
                "الفواكه غنية بالفيتامينات",
                "كلب هو حيوان",
                "قطة هي حيوان",
                // Spanish
                "manzana es una fruta",
                "plátano es una fruta",
                "las frutas son ricas en vitaminas",
                "perro es un animal",
                "gato es un animal",
                // French
                "pomme est un fruit",
                "banane est un fruit",
                "les fruits sont riches en vitamines",
                "chien est un animal",
                "chat est un animal",
                // German
                "apfel ist eine frucht",
                "banane ist eine frucht",
                "früchte sind reich an vitaminen",
                "hund ist ein tier",
                "katze ist ein tier",
            ];
            // Feed 10 rounds, calling discover_words once per round.
            for _round in 0..10 {
                for s in pool.iter() {
                    MindX::feed(s);
                }
                core::INSTANCE.with(|m| {
                    word_discovery::discover_words(&mut m.borrow_mut());
                });
            }
            println!("fed: 10 rounds x {} sentences (multilingual)", pool.len());
            // Ask questions in several languages.
            let questions = [
                // Chinese
                "苹果",
                "苹果 公司",
                "水果",
                "维生素",
                "狗",
                "猫",
                // English
                "apple",
                "fruit",
                "dog",
                "cat",
                // Russian
                "яблоко",
                "фрукт",
                // Greek
                "μήλο",
                "φρούτο",
                // Japanese
                "りんご",
                "果物",
                // Korean
                "사과",
                "과일",
                // Arabic
                "تفاحة",
                "فاكهة",
                // Spanish
                "manzana",
                "fruta",
                // French
                "pomme",
                "fruit",
                // German
                "apfel",
                "frucht",
            ];
            println!("--- query results ---");
            for q in &questions {
                println!("{} => {}", q, MindX::query(q));
            }
            println!("--- state ---");
            core::INSTANCE.with(|m| {
                let mind = m.borrow();
                println!("{}", stats::stats(&mind));
            });
        });
    }
}
