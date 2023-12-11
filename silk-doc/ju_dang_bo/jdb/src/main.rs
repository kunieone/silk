use core::panic;
use std::io;

pub enum Action {
    JU,
    DANG(i32),
    BO(i32),
}
pub struct Player {
    pub qi: i32,
    pub name: String,
    pub action: Option<Action>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            qi: 0,
            name,
            action: None,
        }
    }

    pub fn ju(&mut self) {
        self.qi += 1;
        self.action = Some(Action::JU);
    }

    pub fn dang(&mut self, level: i32) {
        if level <= self.qi {
            self.qi -= level - 1;
            self.action = Some(Action::DANG(level));
        } else {
            panic!("dang level is too high");
        }
    }

    pub fn bo(&mut self, level: i32) -> i32 {
        if level <= self.qi {
            self.qi -= level;
            self.action = Some(Action::BO(level));
        } else {
            panic!("bo level is too high");
        }
        level
    }
}

fn judge(player1: &Player, player2: &Player) -> i32 {
    match (&player1.action, &player2.action) {
        (Some(Action::JU) | Some(Action::DANG(_)), Some(Action::JU) | Some(Action::DANG(_))) => {
            0 // Draw
        }
        (Some(Action::BO(level1)), Some(Action::BO(level2))) => {
            if level1 > level2 {
                1 // Player 1 wins
            } else if level1 < level2 {
                2 // Player 2 wins
            } else {
                0
            }
        }
        (Some(Action::DANG(level)), Some(Action::BO(bo_level))) => {
            if bo_level <= level {
                1 // Player 1 wins
            } else if bo_level == level {
                0
            } else {
                2 // Player 2 wins
            }
        }
        (Some(Action::BO(bo_level)), Some(Action::DANG(level))) => {
            if bo_level <= level {
                2 // Player 2 wins
            } else if bo_level == level {
                0
            } else {
                1 // Player 1 wins
            }
        }
        (Some(Action::JU), Some(Action::BO(_))) => 1, // Player 1 wins
        (Some(Action::BO(_)), Some(Action::JU)) => 2, // Player 2 wins
        (None, None) => unreachable!(),
        _ => panic!("Invalid actions"),
    }
}

use rand::{self};

pub struct Robot {
    pub name: String,
    pub rng: rand::rngs::ThreadRng,
}

impl Robot {
    pub fn new(name: &str) -> Self {
        Robot {
            name: name.to_string(),
            rng: rand::thread_rng(),
        }
    }

    pub fn random_action(&mut self, player_qi: i32) -> Action {
        use rand::Rng;

        if player_qi == 0 {
            return Action::JU;
        }
        match self.rng.gen_range(0..=2) {
            0 => Action::JU,
            1 => {
                let level = self.rng.gen_range(1..=player_qi);
                Action::DANG(level)
            }
            2 => {
                let level = self.rng.gen_range(1..=player_qi);
                Action::BO(level)
            }
            _ => unreachable!(),
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// 定义日志类别
pub enum LogCategory {
    状态,
    动作,
    效果,
    提示,
}

fn main() {
    let mut player1 = Player::new("玩家1".to_string());
    let mut player2 = Player::new("玩家2".to_string());
    let mut robot = Robot::new("机器人");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    loop {
        print_colored(
            &mut stdout,
            Color::Cyan,
            LogCategory::提示,
            "玩家1：请输入你的动作 (w/q/e)：",
        );
        let input = get_user_input();
        if input.as_str() == "w" {
            player1.ju();
            print_colored(
                &mut stdout,
                Color::Green,
                LogCategory::动作,
                "玩家1选择了聚。",
            );
        } else if input.starts_with("q") {
            let level = input.len() as i32;
            if level > player1.qi {
                print_colored(
                    &mut stdout,
                    Color::Green,
                    LogCategory::动作,
                    &format!("等级不够!"),
                );
                continue;
            }
            player1.bo(level);
            print_colored(
                &mut stdout,
                Color::Green,
                LogCategory::动作,
                &format!("玩家1选择了{}波。", level_to_chinese(level)),
            );
        } else if input.starts_with("e") {
            let level = input.len() as i32;
            if level > player1.qi {
                print_colored(
                    &mut stdout,
                    Color::Green,
                    LogCategory::动作,
                    &format!("等级不够!"),
                );
                continue;
            }
            player1.dang(level);
            print_colored(
                &mut stdout,
                Color::Green,
                LogCategory::动作,
                &format!("玩家1选择了{}挡。", level_to_chinese(level)),
            );
        }

        let robot_action = robot.random_action(player2.qi);
        match robot_action {
            Action::JU => {
                player2.ju();
                print_colored(
                    &mut stdout,
                    Color::Yellow,
                    LogCategory::动作,
                    "玩家2 (机器人)选择了聚。",
                );
            }
            Action::DANG(level) => {
                player2.dang(level);
                print_colored(
                    &mut stdout,
                    Color::Yellow,
                    LogCategory::动作,
                    &format!("玩家2 (机器人)选择了{}挡。", level_to_chinese(level)),
                );
            }
            Action::BO(level) => {
                player2.bo(level);
                print_colored(
                    &mut stdout,
                    Color::Yellow,
                    LogCategory::动作,
                    &format!("玩家2 (机器人)选择了{}波。", level_to_chinese(level)),
                );
            }
        };

        let result = judge(&player1, &player2);

        match result {
            0 => {
                print_colored(&mut stdout, Color::Green, LogCategory::状态, "游戏继续");
                print_colored(
                    &mut stdout,
                    Color::Yellow,
                    LogCategory::状态,
                    &format!("玩家1 气: {}, 玩家2 气: {}", player1.qi, player2.qi),
                );
            }
            1 => {
                print_colored(&mut stdout, Color::Green, LogCategory::状态, "玩家1 胜利");
                break;
            }
            2 => {
                print_colored(
                    &mut stdout,
                    Color::Green,
                    LogCategory::状态,
                    "玩家2 (机器人) 胜利",
                );
                break;
            }
            _ => panic!("无效结果"),
        }
    }
}

fn print_colored<W: WriteColor>(
    writer: &mut W,
    color: Color,
    category: LogCategory,
    message: &str,
) {
    let category_color = match category {
        LogCategory::状态 => Color::Blue,
        LogCategory::动作 => Color::Magenta,
        LogCategory::效果 => Color::Cyan,
        LogCategory::提示 => Color::Yellow,
    };

    writer
        .set_color(ColorSpec::new().set_fg(Some(category_color)))
        .expect("设置类别颜色失败");
    write!(writer, "[{}]", category_to_str(category)).expect("写入类别失败");
    writer.reset().expect("重置类别颜色失败");

    writer
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .expect("设置颜色失败");
    writeln!(writer, " {}", message).expect("写入彩色消息失败");
    writer.reset().expect("重置颜色失败");
}

fn category_to_str(category: LogCategory) -> &'static str {
    match category {
        LogCategory::状态 => "状态",
        LogCategory::动作 => "动作",
        LogCategory::效果 => "效果",
        LogCategory::提示 => "提示",
    }
}

fn level_to_chinese(level: i32) -> &'static str {
    match level {
        1 => "",
        2 => "中",
        3 => "大",
        4 => "超",
        5 => "极",
        _ => "未知",
    }
}
