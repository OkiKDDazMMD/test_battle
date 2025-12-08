use rayon::prelude::*;

// ----------------------------------------------------------------------
// 1. データ構造 (変更なし)
// ----------------------------------------------------------------------
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn move_point(&mut self, x: f32, y: f32) {
        self.x = self.x+x;
        self.y = self.y+y;
    }

    // getter メソッド
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }    

    // 2点間のユークリッド距離を計算するメソッド
    pub fn distance2powi(&self, other: &Vec2) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

// ----------------------------------------------------------------------
// 2. 処理構造: BattleSpace structの定義とimpl
// ----------------------------------------------------------------------
pub struct BattleSpace {
    pub points: Vec<Vec2>,
    distance_matrix: Vec<f32>, // 距離行列は初回計算後に保存する
}

impl BattleSpace {
    /// 新しいBattleSpaceを作成し、点群を設定する
    pub fn new(points: Vec<Vec2>) -> Self {
        let n = points.len();
        let matrix = vec![-1.0; n * n]; // -1.0で初期化されたn×n行列

        Self {
            points,
            distance_matrix: matrix, // 初期化時は行列は計算されていない
        }
    }

    pub fn move_point(&mut self, x: f32, y: f32, index: usize) {
        if index < self.points.len() {
            self.points[index].move_point(x, y);
        }
    }

    /// 距離行列の一次元配列内でのインデックスを計算するヘルパー関数
    fn index(&self, i: usize, j: usize, count: usize) -> usize {
        i * count + j
    }

    /// 距離行列を計算し、struct内部に保存する
    pub fn calculate_distance_matrix(&mut self) {
         // --- 追加：配列を並列に初期化 ---
        self.distance_matrix
            .par_iter_mut()
            .for_each(|element| *element = -1.0);

        let point_count = self.points.len();

        for i in 0..point_count {
            for j in 0..point_count {
                let index = self.index(i, j, point_count);

                if i == j {
                    self.distance_matrix[index] = 0.0;
                } else if self.distance_matrix[index] < 0.0 {
                    // pointsフィールドのデータを用いて距離を計算
                    let dist = self.points[i].distance2powi(&self.points[j]);
                    self.distance_matrix[index] = dist;
                    let reverse_index = self.index(j, i, point_count);
                    if self.distance_matrix[reverse_index] < 0.0 {
                        self.distance_matrix[reverse_index] = dist;
                    }
                }
            }
        }
    }

    /// ターゲットとなる点に最も近い点のインデックスと距離を見つける
    /// 距離行列が計算済みであることを前提とする
    /// 戻り値: (最も近い点のインデックス, 最小距離)
    pub fn find_nearest_point(&self, target_index: usize) -> Option<(usize, f32)> {
        let point_count = self.points.len();

        // ターゲットのインデックスが範囲外の場合はNoneを返す
        if target_index >= point_count {
            return None;
        }

        let mut nearest_index: Option<usize> = None;
        let mut min_dist_sq = f32::MAX;

        for j in 0..point_count {
            // 自分自身(target_index)はスキップ
            if j == target_index {
                continue;
            }

            // distance_matrix[target_index][j] にアクセス
            let index = self.index(target_index, j, point_count);
            let dist_sq = self.distance_matrix[index];

            // 注意: `calculate_distance_matrix_squared`が実行されていない場合、
            // ここで dist_sq が -1.0 となり、ロジックが破綻する可能性があるため、
            // 本来は呼び出し元で計算済みであることを保証する必要があります。
            // (このコードでは簡略化のため-1.0チェックは省略します)

            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                nearest_index = Some(j);
            }
        }

        // 最も近い点が見つかった場合、インデックスと距離の二乗のタプルを返す
        nearest_index.map(|idx| (idx, min_dist_sq))
    }
}