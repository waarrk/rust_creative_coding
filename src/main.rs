use nannou::prelude::*;

// メイン関数
fn main() {
    nannou::app(model).update(update).run();
}

// ノード構造体の定義
struct Node {
    pub angle: f32,   // 角度
    pub radius: f32,  // 半径
    pub speed: f32,   // 回転速度
    pub center: Vec2, // 中心座標
}

impl Node {
    // 新しいノードを作成するための関数
    fn new(radius: f32, speed: f32, center: Vec2) -> Self {
        Node {
            angle: random_f32() * 2.0 * PI, // ランダムな初期角度
            radius,
            speed,
            center,
        }
    }

    // ノードの更新関数
    fn update(&mut self) {
        self.angle += self.speed; // 角度を速度に応じて更新
        if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI; // 角度が2πを超えた場合リセット
        } else if self.angle < 0.0 {
            self.angle += 2.0 * PI; // 角度が0未満の場合リセット
        }
    }

    // ノードの現在位置を計算する関数
    fn position(&self) -> Vec2 {
        vec2(
            self.center.x + self.radius * self.angle.cos(),
            self.center.y + self.radius * self.angle.sin(),
        )
    }
}

// モデル構造体の定義
struct Model {
    nodes: Vec<Node>,  // ノードのベクター
    node_count: usize, // ノードの数
}

// アプリケーションの初期化関数
fn model(app: &App) -> Model {
    app.new_window()
        .size(1280, 720) // ウィンドウサイズを設定
        .view(view) // 描画関数を設定
        .key_released(key_released) // キーリリースイベントを設定
        .build()
        .unwrap();

    let node_count = 20;
    let center = vec2(0.0, 0.0); // 中心座標を設定
    let nodes = create_nodes(node_count, center); // ノードを作成

    Model { nodes, node_count } // モデルを返す
}

// ノードを生成する関数
fn create_nodes(node_count: usize, center: Vec2) -> Vec<Node> {
    (0..node_count)
        .map(|_| {
            let radius = random_range(50.0, 900.0); // ランダムな半径を設定
            let speed = random_range(0.005, 0.03); // ランダムな速度を設定
            let speed = if random_f32() < 0.5 { -speed } else { speed }; // 速度の方向をランダムに決定
            Node::new(radius, speed, center) // 新しいノードを作成
        })
        .collect()
}

// 更新関数
fn update(_app: &App, model: &mut Model, _update: Update) {
    for node in model.nodes.iter_mut() {
        node.update(); // 各ノードを更新
    }
}

// 描画関数
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 || app.keys.down.contains(&Key::R) {
        draw.background().color(WHITE); // 最初のフレームやRキーが押された場合背景を白にする
    } else {
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(1.0, 1.0, 1.0, 0.07); // 半透明の矩形を描画してトレイル効果を作る
    }

    // ノードの位置を記憶するためのベクター
    let mut trail_positions = Vec::new();

    for node in model.nodes.iter() {
        let pos = node.position(); // 各ノードの現在位置を取得
        trail_positions.push(pos); // トレイルの位置を記憶
    }

    // トレイルの点どうしを直線で結んで滑らかに描画
    if trail_positions.len() > 1 {
        for i in 0..trail_positions.len() - 1 {
            let p1 = trail_positions[i];
            let p2 = trail_positions[i + 1];
            draw.line()
                .start(p1)
                .end(p2)
                .weight(2.0)
                .color(rgb(29u8, 31u8, 47u8));
        }
    }

    // ノードを描画
    for pos in trail_positions.iter() {
        draw.ellipse()
            .x_y(pos.x, pos.y)
            .radius(5.0) // ノードの位置に小さな円を描画
            .color(rgb(9u8, 15u8, 25u8));
    }

    draw.to_frame(app, &frame).unwrap(); // 描画をフレームに送る
}

// キーリリースイベントハンドラ
fn key_released(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            let center = vec2(0.0, 0.0);
            model.nodes = create_nodes(model.node_count, center); // Rキーが押されたらノードを再生成
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png"); // Sキーが押されたらスクリーンショットを保存
        }
        _other_key => {}
    }
}
