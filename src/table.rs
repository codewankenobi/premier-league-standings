pub mod table {
    use prettytable::Table;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LeagueTable {
        filters: Filters,
        area: Area,
        competition: Competition,
        season: Season,
        standings: Vec<Standing>,
    }

    impl std::fmt::Display for LeagueTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", get_table_as_string(&self))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Filters {
        season: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Area {
        id: u32,
        name: String,
        code: String,
        flag: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Competition {
        id: u32,
        name: String,
        code: String,
        #[serde(rename = "type")]
        competition_type: String,
        emblem: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Season {
        id: u32,
        #[serde(rename = "startDate")]
        start_date: String,
        #[serde(rename = "endDate")]
        end_date: String,
        #[serde(rename = "currentMatchday")]
        current_matchday: u32,
        winner: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Standing {
        stage: String,
        #[serde(rename = "type")]
        standing_type: String,
        group: Option<String>,
        table: Vec<TableRow>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TableRow {
        position: i32,
        team: Team,
        #[serde(rename = "playedGames")]
        played_games: i32,
        form: String,
        won: i32,
        draw: i32,
        lost: i32,
        points: i32,
        #[serde(rename = "goalsFor")]
        goals_for: i32,
        #[serde(rename = "goalsAgainst")]
        goals_against: i32,
        #[serde(rename = "goalDifference")]
        goal_difference: i32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Team {
        id: i32,
        name: String,
        #[serde(rename = "shortName")]
        short_name: String,
        tla: String,
        crest: String,
    }

    fn get_table_as_string(response: &LeagueTable) -> String {
        let mut table = Table::new();
        table.add_row(row![
            "#", "Club", "Played", "Won", "Drawn", "Lost", "GF", "GA", "GD", "Points"
        ]);

        for row in &response.standings[0].table {
            table.add_row(row![
                row.position,
                row.team.short_name,
                row.played_games,
                row.won,
                row.draw,
                row.lost,
                row.goals_for,
                row.goals_against,
                row.goal_difference,
                row.points
            ]);
        }

        table.to_string()
    }
}
