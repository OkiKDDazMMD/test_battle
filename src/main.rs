
mod battle_space;
use battle_space::*;

fn main() {

// 1. データの準備
    let initial_points: Vec<Vec2> = vec![
        Vec2::new(0.0, 0.0),  // 0番目 (ターゲット)
        Vec2::new(1.0, 2.0),  // 1番目
        Vec2::new(5.0, 5.0),  // 2番目
        Vec2::new(0.5, 0.5),  // 3番目 (0番目に近いはず)
    ];

    let target_index = 0; // ターゲットとなる配列番号

    // 2. BattleSpaceの生成と処理の実行
    let mut battle_space = BattleSpace::new(initial_points);
    
    // 距離行列の計算を実行 (struct内部で状態が更新される)
    battle_space.calculate_distance_matrix();

    // 最近傍点の探索
    let result = battle_space.find_nearest_point(target_index);

    // 3. 結果の出力
    match result {
        Some((nearest_idx, min_dist)) => {
            println!("--- 🎯 最適なターゲット探索結果 ---");
            println!("🏠 ターゲット点 (点 {})", target_index);
            println!("  座標: ({}, {})", 
                battle_space.points[target_index].x, battle_space.points[target_index].y);
            println!("---");
            println!("🔍 最も近い点 (点 {})", nearest_idx);
            println!("  座標: ({}, {})", 
                battle_space.points[nearest_idx].x, battle_space.points[nearest_idx].y);
            println!("  距離: {:.4}", min_dist);
        },
        None => {
            println!("⚠️ 比較対象の点がありませんでした (またはターゲットインデックスが不正です)。");
        },
    }

}
