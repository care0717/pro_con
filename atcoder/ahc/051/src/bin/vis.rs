#![allow(non_snake_case)]

use tools::*;

// エラーがあってもパースできる部分だけ取得する関数
fn parse_output_partial(input: &Input, f: &str) -> Result<Output, String> {
    let mut f = f.split_whitespace();
    let mut ds = vec![];
    for _ in 0..input.N {
        match read(f.next(), 0..input.N) {
            Ok(d) => ds.push(d),
            Err(_) => ds.push(0), // エラー時はデフォルト値
        }
    }
    let s = read(f.next(), 0..input.N + input.M).unwrap_or(0);
    let mut cs = vec![];
    for _ in 0..input.M {
        let k = read(f.next(), -1..(input.K as i32)).unwrap_or(-1);
        let k = if k < 0 { !0 } else { k as usize };
        let (v1, v2) = if k == !0 {
            (!0, !0)
        } else {
            (
                read(f.next(), 0..input.N + input.M).unwrap_or(0),
                read(f.next(), 0..input.N + input.M).unwrap_or(0),
            )
        };
        cs.push((k, v1, v2));
    }
    Ok(Output { ds, s, cs })
}

fn main() {
    if std::env::args().len() != 3 {
        eprintln!(
            "Usage: {} <input> <output>",
            std::env::args().nth(0).unwrap()
        );
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
        Err(_) => {
            // エラーがあってもパースできた部分は使用可能にする
            match parse_output_partial(&input, &output) {
                Ok(partial_out) => {
                    serde_json::to_string(&partial_out).unwrap_or_else(|_| "{}".to_string())
                }
                Err(_) => "{}".to_string(),
            }
        }
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
            <div class="control-group">
                <label for="nodeSelector">ノード接続情報:</label>
                <select id="nodeSelector" onchange="showNodeInfo()">
                    <option value="">ノードを選択...</option>
                </select>
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
            <div id="nodeInfo"></div>
        </div>
        
        <div class="svg-container" id="svgContainer">
            {}
        </div>
    </div>
    
    <script>
        const inputData = {};
        const outputData = {};
        let probabilities = {{}};
        
        console.log('Script loaded, parsing JSON data...');
        
        // JSONデータを設定
        try {{
            Object.assign(inputData, {2});
            Object.assign(outputData, {3});
            console.log('JSON data assigned successfully');
            console.log('inputData after assignment:', inputData);
            console.log('outputData after assignment:', outputData);
        }} catch (error) {{
            console.error('Error parsing JSON data:', error);
        }}
        
        function calculateProbabilities(input, output) {{
            console.log('calculateProbabilities called');
            console.log('Input data:', input);
            console.log('Output data:', output);
            
            const probs = {{}};
            const N = input.N;
            const M = input.M;
            
            console.log(`N: ${{N}}, M: ${{M}}, start node: ${{output.s}}`);
            
            // 各ゴミ種類の確率を計算
            for (let wasteType = 0; wasteType < N; wasteType++) {{
                probs[wasteType] = {{}};
                
                // 全ノードの確率を初期化
                for (let i = 0; i < N + M; i++) {{
                    probs[wasteType][i] = 0.0;
                }}
                
                // 開始ノードの確率を1.0に設定
                probs[wasteType][output.s] = 1.0;
                console.log(`Set start node ${{output.s}} probability to 1.0 for waste type ${{wasteType}}`);
                
                // BFS的にネットワークを辿って確率を伝播
                const processed = new Set();
                const toProcess = [output.s];
                
                while (toProcess.length > 0) {{
                    const node = toProcess.shift();
                    if (processed.has(node)) continue;
                    
                    const nodeProb = probs[wasteType][node];
                    if (nodeProb === 0) continue;
                    
                    processed.add(node);
                    console.log(`Processing node ${{node}} for waste type ${{wasteType}}, prob: ${{nodeProb}}`);
                    
                    if (node >= N) {{ // 分別器ノード
                        const sepIndex = node - N;
                        if (sepIndex < output.cs.length && output.cs[sepIndex][0] !== 4294967295) {{ // !0 in Rust is max u32
                            const sortType = output.cs[sepIndex][0];
                            const v1 = output.cs[sepIndex][1];
                            const v2 = output.cs[sepIndex][2];
                            
                            if (sortType < input.ps.length && wasteType < input.ps[sortType].length) {{
                                const prob1 = input.ps[sortType][wasteType];
                                const prob2 = 1.0 - prob1;
                                
                                // 確率を伝播
                                probs[wasteType][v1] += nodeProb * prob1;
                                probs[wasteType][v2] += nodeProb * prob2;
                                
                                console.log(`Separator ${{node}} (type ${{sortType}}) -> v1: ${{v1}} (prob: ${{prob1 * nodeProb}}), v2: ${{v2}} (prob: ${{prob2 * nodeProb}})`);
                                
                                // 次の処理対象に追加
                                if (!processed.has(v1)) toProcess.push(v1);
                                if (!processed.has(v2)) toProcess.push(v2);
                            }}
                        }}
                    }}
                }}
            }}
            
            console.log('Final probabilities:', probs);
            return probs;
        }}
        
        function getColor(probability) {{
            // 確率0-1を0-100に変換して整数にする
            const p = Math.max(0, Math.min(1, probability));
            
            let r, g, b;
            
            if (p <= 0.5) {{
                // 0.0-0.5: 青 → 緑
                const t = p * 2; // 0-1に正規化
                r = Math.round(0 * (1 - t) + 0 * t);
                g = Math.round(0 * (1 - t) + 255 * t);
                b = Math.round(255 * (1 - t) + 0 * t);
            }} else {{
                // 0.5-1.0: 緑 → 赤
                const t = (p - 0.5) * 2; // 0-1に正規化
                r = Math.round(0 * (1 - t) + 255 * t);
                g = Math.round(255 * (1 - t) + 0 * t);
                b = Math.round(0 * (1 - t) + 0 * t);
            }}
            
            return `rgb(${{r}}, ${{g}}, ${{b}})`;
        }}
        
        function calculateEdgeProbability(wasteType, separatorType, output, startNode, endNode) {{
            console.log(`calculateEdgeProbability: wasteType=${{wasteType}}, sepType=${{separatorType}}, output=${{output}}, start=${{startNode}}, end=${{endNode}}`);
            
            // 始点への到達確率を取得
            const startProb = probabilities[wasteType] && probabilities[wasteType][startNode] ? probabilities[wasteType][startNode] : 0;
            console.log(`Start node ${{startNode}} probability: ${{startProb}}`);
            
            if (startProb === 0) {{
                console.log('Start probability is 0, returning 0');
                return 0;
            }}
            
            // 分別器からの分岐確率を計算
            if (separatorType >= 0 && separatorType < inputData.ps.length && wasteType < inputData.N) {{
                let branchProb;
                if (output === "out1") {{
                    branchProb = inputData.ps[separatorType][wasteType];
                }} else if (output === "out2") {{
                    branchProb = 1.0 - inputData.ps[separatorType][wasteType];
                }} else {{
                    console.log('Invalid output type:', output);
                    return 0;
                }}
                
                // 実際の線分通過確率 = 始点到達確率 × 分岐確率
                const edgeProb = startProb * branchProb;
                console.log(`Edge probability: ${{startProb}} * ${{branchProb}} = ${{edgeProb}}`);
                return edgeProb;
            }}
            
            console.log(`Invalid separator type: ${{separatorType}}`);
            return 0;
        }}
        
        function updateVisualization() {{
            console.log('updateVisualization called');
            const wasteType = parseInt(document.getElementById('wasteTypeSlider').value);
            const valueDisplay = document.getElementById('wasteTypeValue');
            const probInfo = document.getElementById('probInfo');
            
            console.log('Selected waste type:', wasteType);
            console.log('Input data available:', !!inputData);
            console.log('Output data available:', !!outputData);
            console.log('Probabilities available:', !!probabilities);
            
            if (wasteType === -1) {{
                valueDisplay.textContent = '全体表示';
                probInfo.textContent = '';
                resetVisualization();
                return;
            }}
            
            valueDisplay.textContent = `ゴミ種類 ${{wasteType}}`;
            
            // データの存在確認
            if (!outputData || !outputData.ds || !probabilities) {{
                console.error('Required data not available');
                probInfo.textContent = 'データが利用できません';
                return;
            }}
            
            // 対応する処理装置を取得
            const correctProcessor = outputData.ds[wasteType];
            const successProb = probabilities[wasteType] && probabilities[wasteType][correctProcessor] ? probabilities[wasteType][correctProcessor] : 0;
            probInfo.innerHTML = `<strong>ゴミ種類 ${{wasteType}}</strong> → 処理装置 ${{correctProcessor}} (成功確率: ${{(successProb * 100).toFixed(1)}}%)`;
            
            console.log('Calling updateSVGColors with wasteType:', wasteType);
            // SVG要素を更新
            updateSVGColors(wasteType);
            highlightProcessor(correctProcessor);
        }}
        
        function resetVisualization() {{
            // 全ての線を元の色（灰色）に戻す
            const groups = document.querySelectorAll('g');
            
            groups.forEach(group => {{
                const titleElement = group.querySelector('title');
                const lineElement = group.querySelector('line');
                
                if (titleElement && lineElement && titleElement.textContent.includes('edge:')) {{
                    lineElement.style.stroke = 'gray';
                    lineElement.style.strokeWidth = '2'; // 元の太さに戻す
                }}
            }});
            
            // 全ての処理装置の強調を解除
            document.querySelectorAll('.processor-node').forEach(circle => {{
                circle.classList.remove('highlighted');
                circle.style.stroke = 'darkgreen';
            }});
        }}
        
        function updateSVGColors(wasteType) {{
            console.log('updateSVGColors called with wasteType:', wasteType);
            
            // SVG内の全てのgroup要素を取得
            const groups = document.querySelectorAll('g');
            console.log(`Found ${{groups.length}} groups`);
            
            let elementsUpdated = 0;
            
            groups.forEach(group => {{
                const titleElement = group.querySelector('title');
                const lineElement = group.querySelector('line');
                
                if (!titleElement || !lineElement) {{
                    return; // タイトルまたは線がない場合はスキップ
                }}
                
                const titleText = titleElement.textContent;
                console.log('Found title:', titleText);
                
                // エッジ（線）の場合のみ処理
                if (titleText.includes('edge:')) {{
                    let prob = 0;
                    
                    if (titleText.includes('inlet -')) {{
                        // 搬入口からの線分
                        prob = 1.0;
                        console.log('Inlet case: prob = 1.0');
                    }} else {{
                        // 分別器からの線分 - titleから詳細情報を抽出
                        const match = titleText.match(/edge: (\d+) - (\d+) \| sep_type: (\d+)/);
                        console.log(`Regex match result:`, match);
                        if (match) {{
                            const startNode = parseInt(match[1]);
                            const endNode = parseInt(match[2]);
                            const separatorType = parseInt(match[3]);
                            
                            // data属性から出力タイプを取得
                            const output = lineElement.getAttribute('data-output');
                            console.log(`Output type from data attribute:`, output);
                            
                            // 線分の実際の通過確率を計算
                            prob = calculateEdgeProbability(wasteType, separatorType, output, startNode, endNode);
                        }} else {{
                            console.log(`No match for title: ${{titleText}}`);
                        }}
                    }}
                    
                    const color = getColor(prob);
                    lineElement.style.stroke = color;
                    lineElement.style.strokeWidth = '3'; // 少し太くして見やすく
                    
                    elementsUpdated++;
                    console.log(`Updated edge: ${{titleText}} -> prob: ${{prob.toFixed(3)}} -> color: ${{color}}`);
                }}
            }});
            
            console.log(`Total elements updated: ${{elementsUpdated}}`);
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
        
        function showNodeInfo() {{
            const nodeSelector = document.getElementById('nodeSelector');
            const nodeInfo = document.getElementById('nodeInfo');
            const selectedNodeId = parseInt(nodeSelector.value);
            
            if (isNaN(selectedNodeId)) {{
                nodeInfo.innerHTML = '';
                return;
            }}
            
            let info = '';
            const N = inputData.N;
            const M = inputData.M;
            
            if (selectedNodeId < N) {{
                // 処理装置
                info = `<h3>処理装置 ${{selectedNodeId}}</h3>`;
                info += `<p>座標: (${{inputData.pos[selectedNodeId][0]}}, ${{inputData.pos[selectedNodeId][1]}})</p>`;
                info += `<p>処理するゴミ: ${{outputData.ds.indexOf(selectedNodeId)}}</p>`;
                
                // この処理装置への入力辺を探す
                let incomingEdges = [];
                if (outputData.s === selectedNodeId) {{
                    incomingEdges.push('搬入口から直接');
                }}
                for (let i = 0; i < M; i++) {{
                    if (outputData.cs[i][0] !== 4294967295) {{
                        if (outputData.cs[i][1] === selectedNodeId) {{
                            incomingEdges.push(`分別器${{N + i}}の出力1から`);
                        }}
                        if (outputData.cs[i][2] === selectedNodeId) {{
                            incomingEdges.push(`分別器${{N + i}}の出力2から`);
                        }}
                    }}
                }}
                
                if (incomingEdges.length > 0) {{
                    info += `<p><strong>入力辺:</strong><br>${{incomingEdges.join('<br>')}}</p>`;
                }} else {{
                    info += `<p><strong>入力辺:</strong> なし</p>`;
                }}
                
            }} else if (selectedNodeId < N + M) {{
                // 分別器
                const sepIndex = selectedNodeId - N;
                info = `<h3>分別器 ${{selectedNodeId}} (インデックス: ${{sepIndex}})</h3>`;
                info += `<p>座標: (${{inputData.pos[selectedNodeId][0]}}, ${{inputData.pos[selectedNodeId][1]}})</p>`;
                
                if (outputData.cs[sepIndex][0] === 4294967295) {{
                    info += `<p><strong>状態:</strong> 設置されていない</p>`;
                }} else {{
                    const sortType = outputData.cs[sepIndex][0];
                    const v1 = outputData.cs[sepIndex][1];
                    const v2 = outputData.cs[sepIndex][2];
                    
                    info += `<p><strong>分別器種類:</strong> ${{sortType}}</p>`;
                    info += `<p><strong>出力1:</strong> ノード${{v1}} ${{v1 < N ? '(処理装置)' : '(分別器)'}}</p>`;
                    info += `<p><strong>出力2:</strong> ノード${{v2}} ${{v2 < N ? '(処理装置)' : '(分別器)'}}</p>`;
                    
                    // 分別確率を表示
                    info += `<p><strong>分別確率 (ゴミ種類別):</strong></p>`;
                    for (let wasteType = 0; wasteType < N; wasteType++) {{
                        const prob1 = inputData.ps[sortType][wasteType];
                        const prob2 = 1.0 - prob1;
                        info += `<p style="margin-left: 20px;">ゴミ${{wasteType}}: 出力1へ ${{(prob1*100).toFixed(1)}}%, 出力2へ ${{(prob2*100).toFixed(1)}}%</p>`;
                    }}
                }}
                
                // この分別器への入力辺を探す
                let incomingEdges = [];
                if (outputData.s === selectedNodeId) {{
                    incomingEdges.push('搬入口から直接');
                }}
                for (let i = 0; i < M; i++) {{
                    if (outputData.cs[i][0] !== 4294967295) {{
                        if (outputData.cs[i][1] === selectedNodeId) {{
                            incomingEdges.push(`分別器${{N + i}}の出力1から`);
                        }}
                        if (outputData.cs[i][2] === selectedNodeId) {{
                            incomingEdges.push(`分別器${{N + i}}の出力2から`);
                        }}
                    }}
                }}
                
                if (incomingEdges.length > 0) {{
                    info += `<p><strong>入力辺:</strong><br>${{incomingEdges.join('<br>')}}</p>`;
                }} else {{
                    info += `<p><strong>入力辺:</strong> なし</p>`;
                }}
            }}
            
            nodeInfo.innerHTML = info;
        }}
        
        function populateNodeSelector() {{
            const nodeSelector = document.getElementById('nodeSelector');
            const N = inputData.N;
            const M = inputData.M;
            
            // 処理装置を追加
            for (let i = 0; i < N; i++) {{
                const option = document.createElement('option');
                option.value = i;
                option.textContent = `処理装置 ${{i}}`;
                nodeSelector.appendChild(option);
            }}
            
            // 分別器を追加
            for (let i = 0; i < M; i++) {{
                const option = document.createElement('option');
                option.value = N + i;
                const status = outputData.cs[i][0] === 4294967295 ? ' (未設置)' : '';
                option.textContent = `分別器 ${{N + i}}${{status}}`;
                nodeSelector.appendChild(option);
            }}
        }}

        // 初期化
        document.addEventListener('DOMContentLoaded', function() {{
            console.log('DOMContentLoaded event fired');
            console.log('inputData keys:', Object.keys(inputData));
            console.log('outputData keys:', Object.keys(outputData));
            
            if (Object.keys(inputData).length > 0 && Object.keys(outputData).length > 0) {{
                console.log('Starting probability calculation...');
                probabilities = calculateProbabilities(inputData, outputData);
                console.log('Probability calculation completed');
                
                const slider = document.getElementById('wasteTypeSlider');
                slider.max = inputData.N - 1;
                
                populateNodeSelector();
                updateVisualization();
            }} else {{
                console.error('inputData or outputData is empty!');
                console.log('inputData:', inputData);
                console.log('outputData:', outputData);
            }}
        }});
    </script>
</body>
</html>"#,
        score, svg, input_json, output_json
    );
    std::fs::write("vis.html", &vis).unwrap();
}
