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
        .adventurer { background-color: #FFD700; color: black; }
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
    "#,
    );

    // Grid container
    html.push_str(&format!(r#"<div class="grid" id="grid"></div>"#));

    // Info panel
    html.push_str(
        r#"
    <div class="info" id="info">
        <div>Adventurer Position: <span id="adventurerPos">(0, 0)</span></div>
        <div>Target: <span id="targetPos">None</span></div>
        <div>Confirmed Cells: <span id="confirmedCount">0</span></div>
        <div>Steps Taken: <span id="stepsTaken">0</span></div>
        <div>Treants Placed: <span id="treantCount">0</span></div>
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
            if (step < 0 || step >= simulationSteps.length) return;
            
            currentStep = step;
            
            // Clear previous states
            for (let i = 0; i < input.N; i++) {{
                for (let j = 0; j < input.N; j++) {{
                    const cell = document.getElementById(`cell-${{i}}-${{j}}`);
                    cell.className = cell.className.replace(/ adventurer| confirmed| target| treant/g, '');
                }}
            }}
            
            const stepData = simulationSteps[step];
            
            // Mark adventurer position
            const adventurerCell = document.getElementById(`cell-${{stepData.adventurerPos[0]}}-${{stepData.adventurerPos[1]}}`);
            adventurerCell.className += ' adventurer';
            adventurerCell.textContent = 'A';
            
            // Mark confirmed cells
            stepData.confirmed.forEach(pos => {{
                const cell = document.getElementById(`cell-${{pos[0]}}-${{pos[1]}}`);
                cell.className += ' confirmed';
            }});
            
            // Mark target
            if (stepData.target && stepData.target[0] !== undefined && stepData.target[1] !== undefined) {{
                const targetCell = document.getElementById(`cell-${{stepData.target[0]}}-${{stepData.target[1]}}`);
                targetCell.className += ' target';
            }}
            
            // Mark treants
            stepData.treants.forEach(pos => {{
                const cell = document.getElementById(`cell-${{pos[0]}}-${{pos[1]}}`);
                cell.className += ' treant';
                cell.textContent = 'T';
            }});
            
            // Update info panel
            document.getElementById('stepInfo').textContent = `Step: ${{step + 1}} / ${{simulationSteps.length}}`;
            document.getElementById('adventurerPos').textContent = `(${{stepData.adventurerPos[0]}}, ${{stepData.adventurerPos[1]}})`;
            document.getElementById('targetPos').textContent = stepData.target ? `(${{stepData.target[0]}}, ${{stepData.target[1]}})` : 'None';
            document.getElementById('confirmedCount').textContent = stepData.confirmed.length;
            document.getElementById('stepsTaken').textContent = step;
            document.getElementById('treantCount').textContent = stepData.treants.length;
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
        
        // Initialize
        initGrid();
        updateStep(0);
    </script>
    "#, input.N, input.t.0, input.t.1, input_b_json, simulation_steps_json));

    html
}

#[derive(serde::Serialize, Debug)]
struct SimulationStep {
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

    // Only simulate if there are treant placements
    if !out.out.is_empty() {
        // Get all treants from the first turn (only turn with placements)
        let first_turn_treants = &out.out[0];
        let mut treants = HashSet::new();
        for &(i, j) in first_turn_treants {
            treants.insert((i, j));
        }

        // Show the state after treant placement
        steps.push(SimulationStep {
            adventurer_pos: entrance,
            confirmed: vec![entrance],
            target: None,
            treants: treants.iter().cloned().collect(),
        });

        // For demonstration, show a few more steps with the adventurer moving
        // This is a simplified simulation since we don't have the full game state
        let mut current_pos = entrance;
        let mut confirmed = HashSet::new();
        confirmed.insert(entrance);

        // Simple pathfinding towards the flower
        let flower_pos = input.t;
        for _step in 1..=10 {
            // Simple movement towards flower
            let (ci, cj) = current_pos;
            let (fi, fj) = flower_pos;

            let mut next_pos = current_pos;
            if ci < fi
                && ci + 1 < input.N
                && !treants.contains(&(ci + 1, cj))
                && input.b[ci + 1][cj] != 'T'
            {
                next_pos = (ci + 1, cj);
            } else if ci > fi
                && ci > 0
                && !treants.contains(&(ci - 1, cj))
                && input.b[ci - 1][cj] != 'T'
            {
                next_pos = (ci - 1, cj);
            } else if cj < fj
                && cj + 1 < input.N
                && !treants.contains(&(ci, cj + 1))
                && input.b[ci][cj + 1] != 'T'
            {
                next_pos = (ci, cj + 1);
            } else if cj > fj
                && cj > 0
                && !treants.contains(&(ci, cj - 1))
                && input.b[ci][cj - 1] != 'T'
            {
                next_pos = (ci, cj - 1);
            }

            if next_pos == current_pos {
                break; // Can't move further
            }

            current_pos = next_pos;
            confirmed.insert(current_pos);

            steps.push(SimulationStep {
                adventurer_pos: current_pos,
                confirmed: confirmed.iter().cloned().collect(),
                target: Some(flower_pos),
                treants: treants.iter().cloned().collect(),
            });

            if current_pos == flower_pos {
                break; // Reached the flower
            }
        }
    }

    steps
}
