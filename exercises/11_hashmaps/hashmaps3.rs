// 给定一个足球比赛的得分列表（每行一个）。每行的格式为
// “<球队1名称>,<球队2名称>,<球队1进球数>,<球队2进球数>”
// 示例：“England,France,4,2”（英格兰进4球，法国进2球）。
//
// 你需要创建一个得分表，其中包含球队名称、球队总进球数
// 和球队总失球数。

use std::collections::HashMap;

// 用于存储球队进球详情的结构体。
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // 球队名称是键，其关联的结构体是值。
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // 注意：我们使用 `unwrap` 是因为我们还没有处理错误处理。
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: 使用提取的详细信息填充得分表。
        // 请记住，球队1的进球数将是球队2的失球数。
        // 同样，球队2的进球数将是球队1的失球数。
        let team_1_scores = scores.entry(team_1_name).or_default();
        team_1_scores.goals_scored += team_1_score;
        team_1_scores.goals_conceded += team_2_score;

        let team_2_scores = scores.entry(team_2_name).or_default();
        team_2_scores.goals_scored += team_2_score;
        team_2_scores.goals_conceded += team_1_score;
    }

    scores
}

fn main() {
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}