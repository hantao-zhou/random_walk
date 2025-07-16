import { randomWalk, edgeReinforcedWalk } from './sim.js';

function init() {
  const canvas = document.getElementById('view');
  const ctx = canvas.getContext('2d');

  const resize = () => {
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
  };

  window.addEventListener('resize', resize);

  document.getElementById('run').onclick = () => {
    resize();
    const steps = parseInt(document.getElementById('steps').value, 10);
    const count = parseInt(document.getElementById('count').value, 10);
    const type = document.getElementById('type').value;

    const walks = [];
    for (let n = 0; n < count; n++) {
      let result;
      if (type === 'erw') {
        const strength = parseFloat(document.getElementById('strength').value);
        const delay = parseInt(document.getElementById('delay').value, 10);
        const memory = parseInt(document.getElementById('memory').value, 10);
        result = edgeReinforcedWalk(steps, strength, delay, memory);
      } else {
        result = randomWalk(steps);
      }
      const hue = (n * 360) / count;
      walks.push({ result, color: `hsl(${hue}, 100%, 50%)` });
    }

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    const scale = 10;
    let i = 1;
    const drawStep = () => {
      if (i >= steps + 1) return;

      ctx.fillStyle = 'rgba(0, 0, 0, 0.02)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      walks.forEach(w => {
        const { result, color } = w;
        if (i < result.x.length) {
          ctx.beginPath();
          const x1 = canvas.width / 2 + result.x[i - 1] * scale;
          const y1 = canvas.height / 2 + result.y[i - 1] * scale;
          const x2 = canvas.width / 2 + result.x[i] * scale;
          const y2 = canvas.height / 2 + result.y[i] * scale;
          ctx.moveTo(x1, y1);
          ctx.lineTo(x2, y2);
          ctx.strokeStyle = color;
          ctx.stroke();
        }
      });

      i += 1;
      requestAnimationFrame(drawStep);
    };

    requestAnimationFrame(drawStep);
  };
}

window.addEventListener('DOMContentLoaded', init);
