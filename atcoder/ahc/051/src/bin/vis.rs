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
        
        function calculateEdgeProbability(wasteType, startNode, endNode) {{
            console.log(`calculateEdgeProbability: wasteType=${{wasteType}}, start=${{startNode}}, end=${{endNode}}`);
            
            // 始点への到達確率
            const startProb = probabilities[wasteType] && probabilities[wasteType][startNode] ? probabilities[wasteType][startNode] : 0;
            console.log(`Start probability: ${{startProb}}`);
            
            if (startProb === 0) {{
                console.log('Start probability is 0, returning 0');
                return 0;
            }}
            
            // 分別器の設定を取得して、特定の出力への確率を計算
            if (startNode >= inputData.N && outputData.cs) {{
                const sepIdx = startNode - inputData.N;
                console.log(`Separator index: ${{sepIdx}}`);
                
                if (sepIdx < outputData.cs.length) {{
                    const sepConfig = outputData.cs[sepIdx];
                    console.log(`Separator config:`, sepConfig);
                    
                    if (sepConfig && sepConfig.length >= 3 && sepConfig[0] !== 4294967295) {{
                        const sortType = sepConfig[0];
                        const out1 = sepConfig[1];
                        const out2 = sepConfig[2];
                        
                        console.log(`Sort type: ${{sortType}}, out1: ${{out1}}, out2: ${{out2}}`);
                        
                        // この分別器からendNodeへの確率を計算
                        if (endNode === out1) {{
                            // 出力1への確率
                            const prob1 = inputData.ps[sortType][wasteType];
                            const result = startProb * prob1;
                            console.log(`Output 1: prob1=${{prob1}}, result=${{result}}`);
                            return result;
                        }} else if (endNode === out2) {{
                            // 出力2への確率
                            const prob2 = 1.0 - inputData.ps[sortType][wasteType];
                            const result = startProb * prob2;
                            console.log(`Output 2: prob2=${{prob2}}, result=${{result}}`);
                            return result;
                        }}
                    }}
                }}
            }}
            
            // 分別器でない場合や設定が不明な場合は始点確率をそのまま使用
            console.log(`Fallback: returning start probability ${{startProb}}`);
            return startProb;
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
            // 全ての線を灰色に戻す
            const edgeSelectors = ['line', 'path', '.edge-line', '.edge', '[stroke]'];
            
            edgeSelectors.forEach(selector => {{
                document.querySelectorAll(selector).forEach(element => {{
                    // ノード（円）は除外
                    if (element.tagName.toLowerCase() !== 'circle') {{
                        element.style.stroke = 'gray';
                        if (element.tagName.toLowerCase() === 'path') {{
                            element.style.fill = 'gray';
                        }}
                    }}
                }});
            }});
            
            // 全ての処理装置の強調を解除
            document.querySelectorAll('.processor-node').forEach(circle => {{
                circle.classList.remove('highlighted');
                circle.style.stroke = 'darkgreen';
            }});
        }}
        
        function updateSVGColors(wasteType) {{
            console.log('updateSVGColors called with wasteType:', wasteType);
            
            // 全ての線（line要素とpath要素）の色を確率に応じて更新
            const edgeSelectors = ['line', 'path', '.edge-line', '.edge', '[stroke="gray"]'];
            
            let totalElementsFound = 0;
            let elementsUpdated = 0;
            
            edgeSelectors.forEach(selector => {{
                const elements = document.querySelectorAll(selector);
                console.log(`Found ${{elements.length}} elements with selector: ${{selector}}`);
                totalElementsFound += elements.length;
                
                elements.forEach(element => {{
                    // 親要素またはその近くのtitle要素を探す
                    let titleElement = null;
                    let currentElement = element;
                    
                    // title要素を探す（親要素、兄弟要素、子要素を含む）
                    for (let i = 0; i < 3; i++) {{
                        titleElement = currentElement.querySelector('title') || 
                                      currentElement.parentElement?.querySelector('title');
                        if (titleElement) break;
                        currentElement = currentElement.parentElement;
                        if (!currentElement) break;
                    }}
                    
                    if (!titleElement) {{
                        console.log('No title element found for element:', element);
                        return;
                    }}
                    
                    const titleText = titleElement.textContent;
                    console.log('Found title:', titleText);
                    
                    // タイトルから始点と終点を抽出
                    const match = titleText.match(/(\\d+) - (\\d+)|inlet - (\\d+)/);
                    if (match) {{
                        let prob = 0;
                        if (match[3]) {{ // inlet case (搬入口からの線分)
                            // 搬入口からは確率1.0で出発
                            prob = 1.0;
                            console.log('Inlet case: prob = 1.0');
                        }} else {{ // separator case (分別器からの線分)
                            const startNode = parseInt(match[1]);
                            const endNode = parseInt(match[2]);
                            
                            // 線分の実際の通過確率を計算
                            prob = calculateEdgeProbability(wasteType, startNode, endNode);
                        }}
                        
                        const color = getColor(prob);
                        element.style.stroke = color;
                        
                        // 矢印の場合はfillも変更
                        if (element.tagName.toLowerCase() === 'path') {{
                            element.style.fill = color;
                        }}
                        
                        elementsUpdated++;
                        console.log(`Updated edge: ${{titleText}} -> prob: ${{prob.toFixed(3)}} -> color: ${{color}}`);
                    }} else {{
                        console.log('No match found for title:', titleText);
                    }}
                }});
            }});
            
            console.log(`Total elements found: ${{totalElementsFound}}, Updated: ${{elementsUpdated}}`);
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
            if (Object.keys(inputData).length > 0 && Object.keys(outputData).length > 0) {{
                probabilities = calculateProbabilities(inputData, outputData);
                
                const slider = document.getElementById('wasteTypeSlider');
                slider.max = inputData.N - 1;
                
                populateNodeSelector();
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
