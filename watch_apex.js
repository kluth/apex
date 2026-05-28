const fs = require('fs');
const { exec } = require('child_process');

const APEX_FILE = 'examples/human_skeleton.apex';
const COMPILE_CMD = 'cargo run --example export_human';

console.log(`🚀 APEX Robust Watch Mode: Polling ${APEX_FILE}...`);

// Use watchFile (polling) for better compatibility in restricted/containerized environments
fs.watchFile(APEX_FILE, { interval: 1000 }, (curr, prev) => {
    if (curr.mtime !== prev.mtime) {
        console.log(`\n📄 [${new Date().toLocaleTimeString()}] Change detected via polling. Recompiling...`);
        
        exec(COMPILE_CMD, (error, stdout, stderr) => {
            if (error) {
                console.error(`❌ Compilation Error: ${error.message}`);
                return;
            }
            console.log(`✅ Compilation Successful. Exported human_skeleton.glb`);
        });
    }
});
