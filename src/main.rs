use simulation_mahjong::game::Game;

fn main() {
    let p = vec![
        (0, "プレイヤー1"),
        (1, "プレイヤー2"),
        (2, "プレイヤー3"),
        (3, "プレイヤー4"),
        (4, "プレイヤー5"),
        (5, "プレイヤー6"),
        (6, "プレイヤー7"),
        (7, "プレイヤー8"),
        (8, "プレイヤー9"),
        (9, "プレイヤー10"),
        (10, "プレイヤー11"),
        (11, "プレイヤー12"),
        (12, "プレイヤー13"),
        (13, "プレイヤー14"),
    ];

    let mut v = vec![];
    let mut game = Game::new(p);

    // 1000回×1000半荘を行う
    for _ in 0..1000 {
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
