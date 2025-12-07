// ----------------------------------------------------------------------
// 1. データ構造 (変更なし)
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // 2点間のユークリッド距離を計算するメソッド
    pub fn distance(&self, other: &Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// ----------------------------------------------------------------------
// 2. 処理構造: BattleSpace structの定義とimpl
// ----------------------------------------------------------------------
pub struct BattleSpace {
    pub points: Vec<Vec2>,
    distance_matrix: Option<Vec<Vec<f64>>>, // 距離行列は初回計算後に保存する
}

impl BattleSpace {
    /// 新しいBattleSpaceを作成し、点群を設定する
    pub fn new(points: Vec<Vec2>) -> Self {
        Self {
            points,
            distance_matrix: None, // 初期化時は行列は計算されていない
        }
    }

    /// 距離行列を計算し、struct内部に保存する
    pub fn calculate_distance_matrix(&mut self) {
        let point_count = self.points.len();
        let mut dist_matrix: Vec<Vec<f64>> = vec![vec![0.0; point_count]; point_count];

        for i in 0..point_count {
            for j in 0..point_count {
                if i == j {
                    dist_matrix[i][j] = 0.0;
                } else {
                    // pointsフィールドのデータを用いて距離を計算
                    let dist = self.points[i].distance(&self.points[j]);
                    dist_matrix[i][j] = dist;
                }
            }
        }
        // 計算結果をフィールドに保存
        self.distance_matrix = Some(dist_matrix);
    }

    /// ターゲットとなる点に最も近い点のインデックスと距離を見つける
    /// 距離行列が計算済みであることを前提とする
    /// 戻り値: (最も近い点のインデックス, 最小距離)
    pub fn find_nearest_point(&self, target_index: usize) -> Option<(usize, f64)> {
        // 距離行列がまだ計算されていない場合はエラー（またはNone）を返す
        let dist_matrix = self.distance_matrix.as_ref()?;

        // ターゲットのインデックスが範囲外の場合はNoneを返す
        if target_index >= dist_matrix.len() {
            return None;
        }

        let target_distances = &dist_matrix[target_index];
        let mut nearest_index: Option<usize> = None;
        let mut min_dist = f64::MAX;

        for (index, &dist) in target_distances.iter().enumerate() {
            // 自分自身(target_index)はスキップ
            if index == target_index {
                continue;
            }

            if dist < min_dist {
                min_dist = dist;
                nearest_index = Some(index);
            }
        }

        // 最も近い点が見つかった場合、インデックスと距離のタプルを返す
        nearest_index.map(|idx| (idx, min_dist))
    }
}