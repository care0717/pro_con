#![allow(non_snake_case)]

use std::collections::HashSet;
use tools::*;

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

    let (score, err, _svg) = match &out {
        Ok(out_data) => vis_default(&input, out_data),
        Err(err) => (0, err.clone(), String::new()),
    };

    if err.len() > 0 {
        println!("{}", err);
        println!("Score = {}", 0);
    } else {
        println!("Score = {}", score);
    }

    // Generate interactive visualization
    if let Ok(out_data) = out {
        let interactive_vis = generate_interactive_vis(&input, &out_data);
        let vis = format!("<html><body>{}</body></html>", interactive_vis);
        std::fs::write("vis.html", &vis).unwrap();
    }
}

fn generate_interactive_vis(input: &Input, out: &Output) -> String {
    let mut html = String::new();

    // CSS styles
    html.push_str(
        r#"
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .controls { margin-bottom: 20px; }
        .controls button { margin: 5px; padding: 10px 20px; font-size: 16px; }
        .grid { 
            display: grid; 
            border: 2px solid #333; 
            gap: 1px;
            background-color: #333;
        }
        .cell { 
            width: 20px; height: 20px; 
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            font-weight: bold;
        }
        .empty { background-color: #90EE90; color: #333; }
        .tree { background-color: #8B4513; color: white; }
        .flower { background-color: #FF69B4; color: white; }
        .adventurer { background-color: #FF0000; color: black; font-weight: bold; font-size: 14px; }
        .confirmed { background-color: #E6E6FA; }
        .target { border: 3px solid #FF0000 !important; box-sizing: border-box; }
        .info { margin-top: 20px; padding: 10px; background-color: #f0f0f0; }
        .treant { background-color: #228B22; color: white; }
    </style>
    "#,
    );

    // Controls
    html.push_str(
        r#"
    <div class="controls">
        <button onclick="prevStep()">← Previous</button>
        <button onclick="nextStep()">Next →</button>
        <button onclick="playAnimation()">Play</button>
        <button onclick="pauseAnimation()">Pause</button>
        <span id="stepInfo">Step: 0 / 0</span>
    </div>
    <div class="controls">
        <label for="stepSlider">Step: </label>
        <input type="range" id="stepSlider" min="0" max="0" value="0" style="width: 300px;">
        <span id="stepSliderValue">0</span>
    </div>
    "#,
    );

    // Grid container
    html.push_str(&format!(r#"<div class="grid" id="grid"></div>"#));

    // Info panel
    html.push_str(
        r#"
    <div class="info" id="info">
        <div><strong>★ Adventurer Position:</strong> <span id="adventurerPos" style="color: #FF0000; font-weight: bold;">(0, 0)</span></div>
        <div><strong>🎯 Target:</strong> <span id="targetPos">None</span></div>
        <div><strong>📋 Confirmed Cells:</strong> <span id="confirmedCount">0</span></div>
        <div><strong>👣 Steps Taken:</strong> <span id="stepsTaken">0</span></div>
        <div><strong>🌳 Treants Placed:</strong> <span id="treantCount">0</span></div>
    </div>
    "#,
    );

    // Generate simulation steps
    let simulation_steps = generate_simulation_steps(input, out);

    // JavaScript for interactive functionality
    let simulation_steps_json = serde_json::to_string(&simulation_steps).unwrap();
    let input_b_json = serde_json::to_string(&input.b).unwrap();
    html.push_str(&format!(r#"
    <script>
        const input = {{
            N: {},
            t: [{}, {}],
            b: {}
        }};
        
        const simulationSteps = {};
        
        console.log('simulationSteps:', simulationSteps);
        console.log('simulationSteps.length:', simulationSteps.length);
        
        let currentStep = 0;
        let animationInterval = null;
        
        function initGrid() {{
            const grid = document.getElementById('grid');
            grid.innerHTML = '';
            grid.style.gridTemplateColumns = `repeat(${{input.N}}, 20px)`;
            
            for (let i = 0; i < input.N; i++) {{
                for (let j = 0; j < input.N; j++) {{
                    const cell = document.createElement('div');
                    cell.className = 'cell';
                    cell.id = `cell-${{i}}-${{j}}`;
                    
                    if (input.b[i][j] === 'T') {{
                        cell.className += ' tree';
                        cell.textContent = 'T';
                    }} else {{
                        cell.className += ' empty';
                        cell.textContent = '.';
                    }}
                    
                    if (i === input.t[0] && j === input.t[1]) {{
                        cell.className += ' flower';
                        cell.textContent = 'F';
                    }}
                    
                    grid.appendChild(cell);
                }}
            }}
        }}
        
        function updateStep(step) {{
            console.log('updateStep called with step:', step);
            console.log('simulationSteps.length:', simulationSteps.length);
            
            if (!simulationSteps || simulationSteps.length === 0) {{
                console.error('simulationSteps is empty or undefined');
                return;
            }}
            
            if (step < 0 || step >= simulationSteps.length) {{
                console.log('step out of range:', step, 'length:', simulationSteps.length);
                return;
            }}
            
            currentStep = step;
            
            // Clear previous states
            for (let i = 0; i < input.N; i++) {{
                for (let j = 0; j < input.N; j++) {{
                    const cell = document.getElementById(`cell-${{i}}-${{j}}`);
                    cell.className = cell.className.replace(/ adventurer| confirmed| target| treant/g, '');
                    // Reset text content to original state
                    if (input.b[i][j] === 'T') {{
                        cell.textContent = 'T';
                    }} else if (i === input.t[0] && j === input.t[1]) {{
                        cell.textContent = 'F';
                    }} else {{
                        cell.textContent = '.';
                    }}
                }}
            }}
            
            const stepData = simulationSteps[step];
            
            if (!stepData) {{
                return;
            }}
            
            // Mark adventurer position
            if (stepData.adventurerPos && stepData.adventurerPos.length >= 2) {{
                const adventurerCell = document.getElementById(`cell-${{stepData.adventurerPos[0]}}-${{stepData.adventurerPos[1]}}`);
                if (adventurerCell) {{
                    adventurerCell.className += ' adventurer';
                    adventurerCell.textContent = '★';
                    adventurerCell.title = `Adventurer at (${{stepData.adventurerPos[0]}}, ${{stepData.adventurerPos[1]}})`;
                }}
            }}
            
            // Mark confirmed cells (but not original trees)
            if (stepData.confirmed && Array.isArray(stepData.confirmed)) {{
                stepData.confirmed.forEach(pos => {{
                    if (pos && pos.length >= 2) {{
                        const cell = document.getElementById(`cell-${{pos[0]}}-${{pos[1]}}`);
                        if (cell && !cell.className.includes('adventurer') && !cell.className.includes('tree')) {{
                            cell.className += ' confirmed';
                        }}
                    }}
                }});
            }}
            
            // Mark target
            if (stepData.target && Array.isArray(stepData.target) && stepData.target.length >= 2 && stepData.target[0] !== undefined && stepData.target[1] !== undefined) {{
                const targetCell = document.getElementById(`cell-${{stepData.target[0]}}-${{stepData.target[1]}}`);
                if (targetCell && !targetCell.className.includes('adventurer')) {{
                    targetCell.className += ' target';
                }}
            }}
            
            // Mark treants (only if not adventurer position)
            if (stepData.treants && Array.isArray(stepData.treants)) {{
                stepData.treants.forEach(pos => {{
                    if (pos && pos.length >= 2) {{
                        const cell = document.getElementById(`cell-${{pos[0]}}-${{pos[1]}}`);
                        if (cell && !cell.className.includes('adventurer')) {{
                            cell.className += ' treant';
                            cell.textContent = 'T';
                        }}
                    }}
                }});
            }}
            
            // Update info panel
            document.getElementById('stepInfo').textContent = `Step: ${{step + 1}} / ${{simulationSteps.length}}`;
            
            // Update slider
            const slider = document.getElementById('stepSlider');
            const sliderValue = document.getElementById('stepSliderValue');
            if (slider && sliderValue) {{
                slider.value = step;
                sliderValue.textContent = step;
            }}
            
            if (stepData.adventurerPos && stepData.adventurerPos.length >= 2) {{
                const adventurerPosElement = document.getElementById('adventurerPos');
                adventurerPosElement.textContent = `(${{stepData.adventurerPos[0]}}, ${{stepData.adventurerPos[1]}})`;
                adventurerPosElement.style.color = '#FF0000';
                adventurerPosElement.style.fontWeight = 'bold';
            }} else {{
                document.getElementById('adventurerPos').textContent = 'Unknown';
            }}
            
            if (stepData.target && Array.isArray(stepData.target) && stepData.target.length >= 2) {{
                document.getElementById('targetPos').textContent = `(${{stepData.target[0]}}, ${{stepData.target[1]}})`;
            }} else {{
                document.getElementById('targetPos').textContent = 'None';
            }}
            
            document.getElementById('confirmedCount').textContent = stepData.confirmed ? stepData.confirmed.length : 0;
            document.getElementById('stepsTaken').textContent = step;
            document.getElementById('treantCount').textContent = stepData.treants ? stepData.treants.length : 0;
        }}
        
        function nextStep() {{
            if (currentStep < simulationSteps.length - 1) {{
                updateStep(currentStep + 1);
            }}
        }}
        
        function prevStep() {{
            if (currentStep > 0) {{
                updateStep(currentStep - 1);
            }}
        }}
        
        function playAnimation() {{
            if (animationInterval) return;
            animationInterval = setInterval(() => {{
                if (currentStep < simulationSteps.length - 1) {{
                    nextStep();
                }} else {{
                    pauseAnimation();
                }}
            }}, 1000);
        }}
        
        function pauseAnimation() {{
            if (animationInterval) {{
                clearInterval(animationInterval);
                animationInterval = null;
            }}
        }}
        
        // Initialize slider
        function initSlider() {{
            const slider = document.getElementById('stepSlider');
            const sliderValue = document.getElementById('stepSliderValue');
            
            if (simulationSteps && simulationSteps.length > 0) {{
                slider.max = simulationSteps.length - 1;
                slider.value = 0;
                sliderValue.textContent = '0';
                
                slider.addEventListener('input', function() {{
                    const step = parseInt(this.value);
                    updateStep(step);
                    sliderValue.textContent = step;
                }});
            }}
        }}
        
        // Initialize
        initGrid();
        initSlider();
        if (simulationSteps && simulationSteps.length > 0) {{
            updateStep(0);
        }} else {{
            document.getElementById('stepInfo').textContent = 'No steps available';
        }}
    </script>
    "#, input.N, input.t.0, input.t.1, input_b_json, simulation_steps_json));

    html
}

#[derive(serde::Serialize, Debug)]
struct SimulationStep {
    #[serde(rename = "adventurerPos")]
    adventurer_pos: (usize, usize),
    confirmed: Vec<(usize, usize)>,
    target: Option<(usize, usize)>,
    treants: Vec<(usize, usize)>,
}

fn generate_simulation_steps(input: &Input, out: &Output) -> Vec<SimulationStep> {
    let mut steps = Vec::new();

    // Initial state
    let entrance = (0, input.N / 2);
    steps.push(SimulationStep {
        adventurer_pos: entrance,
        confirmed: vec![entrance],
        target: None,
        treants: Vec::new(),
    });

    // Simulate the actual game using the Sim struct
    let mut sim = Sim::new(input);
    let mut treants = HashSet::new();

    // Process each turn
    for (turn_idx, turn_treants) in out.out.iter().enumerate() {
        // Filter out treants that are already revealed (they can't be placed)
        let mut valid_treants = Vec::new();
        for &(i, j) in turn_treants {
            if !sim.revealed[i * sim.N + j] {
                valid_treants.push((i, j));
                treants.insert((i, j));
            }
        }

        // Execute the turn with only valid treants
        match sim.step(&valid_treants) {
            Ok(_) => {
                // Get current state
                let mut confirmed = HashSet::new();
                for i in 0..input.N {
                    for j in 0..input.N {
                        if sim.revealed[i * input.N + j] {
                            confirmed.insert((i, j));
                        }
                    }
                }

                let step = SimulationStep {
                    adventurer_pos: sim.p,
                    confirmed: confirmed.iter().cloned().collect(),
                    target: Some(sim.target),
                    treants: treants.iter().cloned().collect(),
                };
                steps.push(step);

                // Check if game ended
                if sim.p == sim.t {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Step {} failed: {}", turn_idx, e);
                break;
            }
        }
    }

    eprintln!("Generated {} steps", steps.len());

    steps
}
