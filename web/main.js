import Module from './walk.js';

async function run() {
  const wasm = await Module();
  const { random_walk, erw_walk } = wasm;
  const canvas = document.getElementById('view');
  const ctx = canvas.getContext('2d');
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
  document.getElementById('run').onclick = () => {
    const steps = parseInt(document.getElementById('steps').value, 10);
    const type = document.getElementById('type').value;
    let result;
    if (type === 'erw') {
      const strength = parseFloat(document.getElementById('strength').value);
      const delay = parseInt(document.getElementById('delay').value, 10);
      const memory = parseInt(document.getElementById('memory').value, 10);
      result = erw_walk(steps, strength, delay, memory);
    } else {
      result = random_walk(steps);
    }
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.beginPath();
    const scale = 5;
    for (let i = 0; i < result.x.length; i++) {
      const x = canvas.width / 2 + result.x[i] * scale;
      const y = canvas.height / 2 + result.y[i] * scale;
      if (i === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    }
    ctx.strokeStyle = 'lime';
    ctx.stroke();
  };
}

run();
