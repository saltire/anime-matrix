const grid = document.querySelector('#grid');
const slide = document.querySelector('#slide');
const current = document.querySelector('#current-color');
const button = document.querySelector('button');

const ns = 'http://www.w3.org/2000/svg';

const widths = [
  32, 33, 33, 33, 33, 33, 33, 32, 32, 31, 31, 30, 30, 29, 29, 28, 28, 27, 27,
  26, 26, 25, 25, 24, 24, 23, 23, 22, 22, 21, 21, 20, 20, 19, 19, 18, 18,
  17, 17, 16, 16, 15, 15, 14, 14, 13, 13, 12, 12, 11, 11, 10, 10, 9, 9,
];
const size = 10;
const canvaswidth = (Math.max(...widths) + 0.5) * size;
const canvasheight = ((widths.length / 2) + 0.5) * size;

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

  return poly;
};

widths.forEach((width, y) => {
  for (let x = 0; x < width; x++) {
    // Shift thr first row right by one.
    map[`${x},${y}`] = drawDiamond(y === 0 ? x + 1 : x, y);
  }
});

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
  const rows = [];
  widths.forEach((width, y) => {
    rows[y] = [];
    for (let x = 0; x < width; x++) {
      rows[y].push(parseInt(map[`${x},${y}`].getAttribute('fill').slice(1, 3), 16));
    }
  });


  fetch('/post', {
    method: 'post',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ rows }),
  });
};
