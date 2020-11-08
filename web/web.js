const ns = 'http://www.w3.org/2000/svg';
const grid = document.querySelector('#grid');
const slide = document.querySelector('#slide');
const current = document.querySelector('#current-color');

const width = 10;
const height = 10;
const size = 10;
const canvaswidth = (width + 0.5) * size;
const canvasheight = ((height / 2) + 0.5) * size;

grid.setAttribute('viewBox', `0 0 ${canvaswidth} ${canvasheight}`);

let color = 'white';
let painting = null;

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
  poly.setAttribute('fill', 'black');
  grid.appendChild(poly);

  poly.onmousedown = () => {
    poly.setAttribute('fill', color);
    painting = color;
    // if (poly.getAttribute('fill') === 'blue') {
    //   poly.setAttribute('fill', 'red');
    //   painting = 'red';
    // }
    // else {
    //   poly.setAttribute('fill', 'blue');
    //   painting = 'blue';
    // }
  };

  poly.onmouseenter = () => {
    if (painting) {
      poly.setAttribute('fill', painting);
    }
  };
};

for (x = 0; x < 10; x++) {
  for (y = 0; y < 10; y++) {
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
  color = `rgb(${value}, ${value}, ${value})`;
  current.setAttribute('fill', color);
}
