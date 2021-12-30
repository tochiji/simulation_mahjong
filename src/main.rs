use simulation_mahjong::game::Game;

fn main() {
    let p = vec![
        (0, "プレイヤー1", 75),
        (1, "プレイヤー2", 70),
        (2, "プレイヤー3", 55),
        (3, "プレイヤー4", 45),
        (4, "プレイヤー5", 40),
        (5, "プレイヤー6", 30),
        (6, "プレイヤー7", 25),
        (7, "プレイヤー8", 25),
        (8, "プレイヤー9", 12),
        (9, "プレイヤー10", 12),
        (10, "プレイヤー11", 7),
        (11, "プレイヤー12", 3),
        (12, "プレイヤー13", 3),
        (13, "プレイヤー14", 2),
    ];

    let mut v = vec![];
    let mut game = Game::new(p);

    // 10000回×1000半荘を行う
    for _ in 0..10000 {
        game.play_hanchans(1000);

        // 1000半荘繰り返した結果の最大ポイントを格納する
        v.push(game.get_max_point());

        game.reset_points();
    }
    v.sort();

    // 最大ポイントを全て表示
    for max_point in v {
        println!("{}", max_point);
    }
}
