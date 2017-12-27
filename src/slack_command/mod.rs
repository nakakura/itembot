use regex::{ Regex, Captures };

/// ・monoコマンドかどうかの判別
/// ・ホワイトスペースの除去
/// ・コーテーションの除去
/// ・少なくとも1つ以上のパラメータがあるか
/// ・コマンドではなく引数が与えられているか
/// までは正規表現でチェック
lazy_static! {
  static ref REGEX_QUOTATION: Regex = {
    Regex::new(r###"['“"](.+?)[“"']"###).unwrap()
  };

  static ref REGEX_WHITE_SPACE: Regex = {
    Regex::new(r"\s+").unwrap()
  };

  static ref REGEX_NUM_COMMAND: Regex = {
    Regex::new(r"^mono\s(\S+)\s(0|[1-9]\d*)$").unwrap()
  };

  static ref REGEX_BOT_COMMAND: Regex = {
    Regex::new(r"^mono\s(\S+)\s(.*)$").unwrap()
  };
}

/// コーテーションの間のスペースをアンダーバーに変える
/// これはMySQLに投入するための前処理を兼ねる
fn replace_quotation_part(str: &str) -> String {
    (&REGEX_QUOTATION).replace_all(str,|caps: &Captures| {
        (&REGEX_WHITE_SPACE).replace_all(&caps[1].trim(), "_").to_string()
    }).to_string()
}

fn extract_number(str: &str) -> Option<Vec<String>> {
    let caps = (&REGEX_NUM_COMMAND).captures(str);
    caps.map(|c| {
        vec!(c[1].to_string(), c[2].to_string())
    })
}

/// slackから受け取った文字列をパラメータ文字列として処理
fn extract_params(str: &str) -> Option<Vec<String>> {
    let caps = (&REGEX_BOT_COMMAND).captures(str);
    caps.map(|c| {
        vec!(c[1].to_string(), c[2].to_string())
    })
}

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct SlackCommand {
    pub user: String,
    pub channel: String,
    pub plugin: String,
    pub command: String,
    pub params: Vec<String>,
    pub number: Option<usize>,
}

impl SlackCommand {
    pub fn create_command(usr: &str, channel: &str, message: &str) -> Option<Self> {
        let formatted_string = replace_quotation_part(message);
        if let Some(vec) = extract_number(&formatted_string) {
            Some(SlackCommand {
                user: usr.to_string(),
                channel: channel.to_string(),
                plugin: vec[0].clone(),
                command: "number".to_string(),
                params: vec!(),
                number: Some(vec[1].parse::<usize>().unwrap()),
            })
        } else if let Some(params) = extract_params(&formatted_string) {
            let mut p: Vec<&str> = (&REGEX_WHITE_SPACE).split(&params[1]).collect();
            let vec = p.split_off(1).into_iter().map(|s| s.to_string()).collect();
            Some(SlackCommand {
                user: usr.to_string(),
                channel: channel.to_string(),
                plugin: params[0].clone(),
                command: p[0].to_string(),
                params: vec,
                number: None,
            })
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_quotation_whitespace() {
        let result = replace_quotation_part("aa hogehoge \"tab    english  sentence \" “全角スペース　日本語の　文章\"");
        assert_eq!(result, "aa hogehoge tab_english_sentence 全角スペース_日本語の_文章");
    }

    #[test]
    fn test_remove_quotation_whitespace_odd() {
        let result = replace_quotation_part("foo hogehoge \"tab    english  sentence \" “全角スペース　日本語の　文章\" \"");
        assert_eq!(result, "foo hogehoge tab_english_sentence 全角スペース_日本語の_文章 \"");
    }

    #[test]
    fn test_remove_quotation_whitespace_none() {
        let result = replace_quotation_part("foo hogehoge ");
        assert_eq!(result, "foo hogehoge ");
    }

    #[test]
    fn test_extract_params() {
        let result = extract_params("mono hoge1 hoge2 hoge3");
        assert_eq!(result, Some(vec!("hoge1".to_string(), "hoge2 hoge3".to_string())));
    }

    #[test]
    fn test_extract_params_too_short() {
        let result = extract_params("mono ");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_params_no_command() {
        let result = extract_params("hoge moge");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_num() {
        let result = extract_number("mono item 1");
        assert_eq!(result, Some(vec!("item".to_string(), "1".to_string())));
    }

    #[test]
    fn test_extract_0() {
        let result = extract_number("mono item 0");
        assert_eq!(result, Some(vec!("item".to_string(), "0".to_string())));
    }

    #[test]
    fn test_extract_negative() {
        let result = extract_number("mono -1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_no_number() {
        let result = extract_number("mono xx xx");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_too_short() {
        let result = extract_number("mono ");
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_invalid_command() {
        let result = extract_number("xxx ");
        assert_eq!(result, None);
    }

    #[test]
    fn test_create_slackpost_num() {
        let result = SlackCommand::create_command("user1", "channel1", "mono item 1").unwrap();
        assert_eq!(result.user, "user1".to_string());
        assert_eq!(result.channel, "channel1".to_string());
        assert_eq!(result.plugin, "item".to_string());
        assert_eq!(result.params, Vec::<String>::new());
        assert_eq!(result.command, "number".to_string());
        assert_eq!(result.number, Some(1));
    }

    #[test]
    fn test_create_slackpost_num_extra_args() {
        let result = SlackCommand::create_command("user1", "channel1", "mono item 1afdafa").unwrap();
        assert_eq!(result.user, "user1".to_string());
        assert_eq!(result.channel, "channel1".to_string());
        assert_eq!(result.plugin, "item".to_string());
        assert_eq!(result.params, Vec::<String>::new());
        assert_eq!(result.command, "1afdafa".to_string());
        assert_eq!(result.number, None);
    }

    #[test]
    fn test_create_slackpost_num_extra_args2() {
        let result = SlackCommand::create_command("user1", "channel1", "mono item 1 afdafa").unwrap();
        assert_eq!(result.user, "user1".to_string());
        assert_eq!(result.channel, "channel1".to_string());
        assert_eq!(result.plugin, "item".to_string());
        assert_eq!(result.params, vec!("afdafa".to_string()));
        assert_eq!(result.command, "1".to_string());
        assert_eq!(result.number, None);
    }

    #[test]
    fn test_create_slackpost_non_num() {
        let result = SlackCommand::create_command("user1", "channel1", "mono x y").unwrap();
        assert_eq!(result.user, "user1".to_string());
        assert_eq!(result.channel, "channel1".to_string());
        assert_eq!(result.plugin, "x".to_string());
        assert_eq!(result.command, "y".to_string());
        assert_eq!(result.params, Vec::<String>::new());
        assert_eq!(result.number, None);
    }

    #[test]
    fn test_create_slackpost_too_short() {
        let result = SlackCommand::create_command("user1", "channel1", "mono 1");
        assert_eq!(result, None);
    }
}