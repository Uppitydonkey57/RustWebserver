const width = 2000;
const height = 1000;
var canvas;
var ctx;
function load() {
	canvas = document.getElementById("myCanvas");
	ctx = canvas.getContext("2d");
}

var planets = {{}}

function draw() {
	ctx.beginPath();
	ctx.rect(0,0,width,height);
	ctx.fill();
	ctx.closePath();

}

function update() {
}

function loop(timestamp) {
  var progress = timestamp - lastRender

  update(progress)
  draw()

  lastRender = timestamp
  window.requestAnimationFrame(loop)
}
var lastRender = 0
window.requestAnimationFrame(loop)
