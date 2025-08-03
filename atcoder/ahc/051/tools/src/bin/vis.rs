#![allow(non_snake_case)]

use tools::*;

fn main() {
    if std::env::args().len() != 3 {
        eprintln!("Usage: {} <input> <output>", std::env::args().nth(0).unwrap());
        return;
    }
    let in_file = std::env::args().nth(1).unwrap();
    let out_file = std::env::args().nth(2).unwrap();
    let input = std::fs::read_to_string(&in_file).unwrap_or_else(|_| {
        eprintln!("no such file: {}", in_file);
        std::process::exit(1)
    });
    let output = std::fs::read_to_string(&out_file).unwrap_or_else(|_| {
        eprintln!("no such file: {}", out_file);
        std::process::exit(1)
    });
    let input = parse_input(&input);
    let out = parse_output(&input, &output);
    let (score, err, svg) = match &out {
        Ok(out) => vis_default(&input, &out),
        Err(err) => (0, err.clone(), String::new()),
    };
    let input_json = serde_json::to_string(&input).unwrap_or_else(|_| "{}".to_string());
    let output_json = match out {
        Ok(ref out) => serde_json::to_string(&out).unwrap_or_else(|_| "{}".to_string()),
        Err(_) => "{}".to_string(),
    };
    if err.len() > 0 {
        println!("{}", err);
        println!("Score = {}", 0);
    } else {
        println!("Score = {}", score);
    }
    let vis = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Waste Processing Visualizer</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }}
        .container {{ max-width: 1200px; margin: 0 auto; background-color: white; padding: 20px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .controls {{ margin-bottom: 20px; padding: 15px; background-color: #f9f9f9; border-radius: 8px; }}
        .control-group {{ margin-bottom: 15px; }}
        label {{ display: block; font-weight: bold; margin-bottom: 5px; color: #333; }}
        .slider-container {{ display: flex; align-items: center; gap: 15px; }}
        .slider {{ flex: 1; height: 8px; border-radius: 5px; background: #ddd; outline: none; }}
        .slider::-webkit-slider-thumb {{ appearance: none; width: 20px; height: 20px; border-radius: 50%; background: #4CAF50; cursor: pointer; }}
        .slider::-moz-range-thumb {{ width: 20px; height: 20px; border-radius: 50%; background: #4CAF50; cursor: pointer; border: none; }}
        .value-display {{ min-width: 100px; font-weight: bold; color: #4CAF50; }}
        .info {{ background-color: #e3f2fd; padding: 15px; border-radius: 8px; margin-bottom: 20px; border-left: 4px solid #2196F3; }}
        .legend {{ display: flex; align-items: center; margin-bottom: 10px; gap: 10px; }}
        .color-box {{ width: 25px; height: 15px; border: 1px solid #333; border-radius: 3px; }}
        .svg-container {{ border: 2px solid #ddd; border-radius: 8px; overflow: hidden; }}
        .highlighted {{ stroke-width: 4 !important; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🗂️ 廃棄物処理システム ビジュアライザ</h1>
        
        <div class="controls">
            <div class="control-group">
                <label for="wasteTypeSlider">ゴミ種類選択:</label>
                <div class="slider-container">
                    <span>全体表示</span>
                    <input type="range" id="wasteTypeSlider" class="slider" min="-1" max="19" value="-1" oninput="updateVisualization()">
                    <span class="value-display" id="wasteTypeValue">全体表示</span>
                </div>
            </div>
        </div>
        
        <div class="info">
            <div class="legend">
                <span>通過確率:</span>
                <div class="color-box" style="background: linear-gradient(to right, #1e90ff, #ff6347);"></div>
                <span>低 (青) → 高 (赤)</span>
            </div>
            <div id="scoreInfo">スコア: {}</div>
            <div id="probInfo"></div>
        </div>
        
        <div class="svg-container" id="svgContainer">
            {}
        </div>
    </div>
    
    <script>
        const inputData = {};
        const outputData = {};
        let probabilities = {{}};
        
        function calculateProbabilities(input, output) {{
            const probs = {{}};
            const N = input.N;
            const M = input.M;
            
            // 各ゴミ種類の確率を計算
            for (let wasteType = 0; wasteType < N; wasteType++) {{
                probs[wasteType] = {{}};
                
                // 全ノードの確率を初期化
                for (let i = 0; i < N + M; i++) {{
                    probs[wasteType][i] = 0.0;
                }}
                
                // 開始ノードの確率を1.0に設定
                probs[wasteType][output.s] = 1.0;
                
                // トポロジカルソートに基づいて確率を伝播
                const visited = new Set();
                const queue = [output.s];
                
                while (queue.length > 0) {{
                    const node = queue.shift();
                    if (visited.has(node)) continue;
                    visited.add(node);
                    
                    if (node >= N) {{ // 分別器ノード
                        const sepIndex = node - N;
                        if (output.cs[sepIndex][0] !== 4294967295) {{ // !0 in Rust is max u32
                            const sortType = output.cs[sepIndex][0];
                            const v1 = output.cs[sepIndex][1];
                            const v2 = output.cs[sepIndex][2];
                            
                            const prob1 = input.ps[sortType][wasteType];
                            const prob2 = 1.0 - prob1;
                            
                            probs[wasteType][v1] += probs[wasteType][node] * prob1;
                            probs[wasteType][v2] += probs[wasteType][node] * prob2;
                            
                            queue.push(v1);
                            queue.push(v2);
                        }}
                    }}
                }}
            }}
            
            return probs;
        }}
        
        function getColor(probability) {{
            const r = Math.round(30 + (255 - 30) * probability);
            const g = Math.round(144 + (30 - 144) * probability);
            const b = Math.round(255 + (30 - 255) * probability);
            return `rgb(${{r}}, ${{g}}, ${{b}})`;
        }}
        
        function updateVisualization() {{
            const wasteType = parseInt(document.getElementById('wasteTypeSlider').value);
            const valueDisplay = document.getElementById('wasteTypeValue');
            const probInfo = document.getElementById('probInfo');
            
            if (wasteType === -1) {{
                valueDisplay.textContent = '全体表示';
                probInfo.textContent = '';
                resetVisualization();
                return;
            }}
            
            valueDisplay.textContent = `ゴミ種類 ${{wasteType}}`;
            
            // 対応する処理装置を取得
            const correctProcessor = outputData.ds[wasteType];
            const successProb = probabilities[wasteType][correctProcessor] || 0;
            probInfo.innerHTML = `<strong>ゴミ種類 ${{wasteType}}</strong> → 処理装置 ${{correctProcessor}} (成功確率: ${{(successProb * 100).toFixed(1)}}%)`;
            
            // SVG要素を更新
            updateSVGColors(wasteType);
            highlightProcessor(correctProcessor);
        }}
        
        function resetVisualization() {{
            // 全ての線を灰色に戻す
            document.querySelectorAll('.edge-line').forEach(line => {{
                line.style.stroke = 'gray';
            }});
            
            // 全ての処理装置の強調を解除
            document.querySelectorAll('.processor-node').forEach(circle => {{
                circle.classList.remove('highlighted');
                circle.style.stroke = 'darkgreen';
            }});
        }}
        
        function updateSVGColors(wasteType) {{
            // 全ての線の色を確率に応じて更新
            document.querySelectorAll('.edge-line').forEach(line => {{
                const group = line.parentElement;
                const title = group.querySelector('title').textContent;
                
                // タイトルから始点と終点を抽出
                const match = title.match(/(\\d+) - (\\d+)|inlet - (\\d+)/);
                if (match) {{
                    let prob = 0;
                    if (match[3]) {{ // inlet case
                        const endNode = parseInt(match[3]);
                        prob = probabilities[wasteType][endNode] || 0;
                    }} else {{ // separator case
                        const endNode = parseInt(match[2]);
                        prob = probabilities[wasteType][endNode] || 0;
                    }}
                    
                    line.style.stroke = getColor(prob);
                }}
            }});
        }}
        
        function highlightProcessor(processorId) {{
            // 全ての処理装置の強調を解除
            document.querySelectorAll('.processor-node').forEach(circle => {{
                circle.classList.remove('highlighted');
                circle.style.stroke = 'darkgreen';
            }});
            
            // 対応する処理装置を強調
            const processorGroups = document.querySelectorAll('g');
            processorGroups.forEach(group => {{
                const title = group.querySelector('title');
                if (title && title.textContent.includes(`vertex: ${{processorId}} (processor`)) {{
                    const circle = group.querySelector('.processor-node');
                    if (circle) {{
                        circle.classList.add('highlighted');
                        circle.style.stroke = '#ff6347';
                        circle.style.strokeWidth = '4';
                    }}
                }}
            }});
        }}
        
        // 初期化
        document.addEventListener('DOMContentLoaded', function() {{
            if (Object.keys(inputData).length > 0 && Object.keys(outputData).length > 0) {{
                probabilities = calculateProbabilities(inputData, outputData);
                
                const slider = document.getElementById('wasteTypeSlider');
                slider.max = inputData.N - 1;
                
                updateVisualization();
            }}
        }});
    </script>
</body>
</html>"#,
        score, svg, input_json, output_json
    );
    std::fs::write("vis.html", &vis).unwrap();
}
