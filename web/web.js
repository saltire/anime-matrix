const grid = document.querySelector('#grid');
const slide = document.querySelector('#slide');
const current = document.querySelector('#current-color');
const button = document.querySelector('button');

const ns = 'http://www.w3.org/2000/svg';

const width = 10;
const height = 10;
const size = 10;
const canvaswidth = (width + 0.5) * size;
const canvasheight = ((height / 2) + 0.5) * size;

grid.setAttribute('viewBox', `0 0 ${canvaswidth} ${canvasheight}`);

let color = '#ffffff';
let painting = null;
const map = {};

const drawDiamond = (x, y) => {
  const poly = document.createElementNS(ns, 'polygon');
  const ox = x + (y % 2 ? 0.5 : 0);
  const oy = y / 2;
  poly.setAttribute('points',
    [
      [ox + 0.5, oy],
      [ox + 1, oy + 0.5],
      [ox + 0.5, oy + 1],
      [ox, oy + 0.5],
    ]
    .map(point => point.map(v => v * size).join(','))
    .join(' '));
  poly.setAttribute('fill', '#000000');
  grid.appendChild(poly);

  poly.onmousedown = () => {
    poly.setAttribute('fill', color);
    painting = color;
  };

  poly.onmouseenter = () => {
    if (painting) {
      poly.setAttribute('fill', painting);
    }
  };

  map[`${x},${y}`] = poly;
};

for (let y = 0; y < height; y++) {
  for (let x = 0; x < width; x++) {
    drawDiamond(x, y);
  }
}

grid.onmouseup = () => {
  painting = null;
}
grid.onmouseleave = () => {
  painting = null;
}

slide.onmousedown = (e) => {
  const rect = e.target.getBoundingClientRect();
  const value = Math.round((e.x - rect.x) / rect.width * 255);
  const hex = value.toString(16);
  color = `#${hex}${hex}${hex}`;
  current.setAttribute('fill', color);
}

button.onclick = () => {
  const values = [];
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      values.push(parseInt(map[`${x},${y}`].getAttribute('fill').slice(1, 3), 16));
    }
  }

  fetch('/post', {
    method: 'post',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ values }),
  });
};
