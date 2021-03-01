// OpenGL resources

var glResources = {};

// Game state

const worldSizeX = 55;
const worldSizeY = 44;

var player = { x: worldSizeX / 2, y: worldSizeY / 2 };
var worldMap = createWorldMap();

// Pressed-key state

const keyPressed = new Set();

// Buffer for accumulating geometry to be sent for rendering
// Position: four vertices per quad, four components per position (x, y, s, t)
// Colors: four colors per quad, four components (RGBA) per color

const maxQuads = 2048;
const vertexPositions = new Float32Array(maxQuads * 16);
const vertexColors = new Float32Array(maxQuads * 16);
let numQuads = 0;

// Functions

const loadImage = src =>
	new Promise((resolve, reject) => {
		const img = new Image();
		img.onload = () => resolve(img);
		img.onerror = reject;
		img.src = src;
	});

function loadWasm(src) {
	return new Promise((resolve, reject) => {
		fetch(src);
	});
}

function loadResourcesThenStart() {
	loadImage('tiles.png').then(image => main(image));
}

function main(image) {

	// Initialize all WebGL resources

	const canvas = document.querySelector("#canvas");
	const gl = canvas.getContext("webgl", { alpha: false, antialias: false, depth: false });

	if (gl == null) {
		alert("Unable to initialize WebGL. Your browser or machine may not support it.");
		return;
	}

	glResources = initGlResources(gl, image);

	// Set up various WebGL state that won't change for the duration of the program:

	gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
	gl.enable(gl.BLEND);
	gl.clearColor(0.65, 0.65, 0.65, 1.0);

	gl.bindBuffer(gl.ARRAY_BUFFER, glResources.buffers.position);
	gl.vertexAttribPointer(glResources.attribLocations.vertexPosition, 4, gl.FLOAT, false, 0, 0);
	gl.enableVertexAttribArray(glResources.attribLocations.vertexPosition);

	gl.bindBuffer(gl.ARRAY_BUFFER, glResources.buffers.color);
	gl.vertexAttribPointer(glResources.attribLocations.vertexColor, 4, gl.FLOAT, false, 0, 0);
	gl.enableVertexAttribArray(glResources.attribLocations.vertexColor);

	gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, glResources.buffers.indices);

	gl.useProgram(glResources.program);

	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D, glResources.texture);
	gl.uniform1i(glResources.uniformLocations.uSampler, 0);

	// Set up the keyboard state update

	document.body.addEventListener("keydown", e => {
		// console.log("Key Pressed:" + e.key + " (" + e.code + ")");
		keyPressed.add(e.code);
	});
	document.body.addEventListener("keyup", e => {
		// console.log("Key Released:" + e.key + " (" + e.code + ")");
		keyPressed.delete(e.code);
	});

	// Main loop

	let then = 0;

	function render(now) {
		const deltaTime = Math.min(1/60, (now - then) * 0.001); // convert to seconds
		then = now;

		update(deltaTime);
		drawScene(gl, glResources);

		requestAnimationFrame(render);
	}

	requestAnimationFrame(render);
}

function initGlResources(gl, image) {
	const vsSource = `
		attribute vec4 aVertexPosition;
		attribute vec4 aVertexColor;
		
		uniform mat4 uProjectionMatrix;

		varying lowp vec4 vColor;
		varying highp vec2 vTextureCoord;

		void main() {
			gl_Position = uProjectionMatrix * vec4(aVertexPosition.xy, 0, 1);
			vColor = aVertexColor;
			vTextureCoord = aVertexPosition.zw;
		}
	`;

	const fsSource = `
		varying lowp vec4 vColor;
		varying highp vec2 vTextureCoord;

		uniform sampler2D uSampler;

		void main() {
			gl_FragColor = vColor * texture2D(uSampler, vTextureCoord);
		}
	`;

	const program = initShaderProgram(gl, vsSource, fsSource);

	const buffers = initBuffers(gl);

	const texture = createTextureFromImage(gl, image);

	return {
		program: program,
		attribLocations: {
			vertexPosition: gl.getAttribLocation(program, 'aVertexPosition'),
			vertexColor: gl.getAttribLocation(program, 'aVertexColor'),
		},
		uniformLocations: {
			projectionMatrix: gl.getUniformLocation(program, 'uProjectionMatrix'),
			uSampler: gl.getUniformLocation(program, 'uSampler'),
		},
		buffers: buffers,
		texture: texture,
	};
}

function initBuffers(gl) {
	return {
		position: gl.createBuffer(),
		color: gl.createBuffer(),
		indices: createElementBuffer(gl),
	};
}

function createElementBuffer(gl) {

	const indices = new Uint16Array(maxQuads * 6);

	for (let i = 0; i < maxQuads; ++i) {
		let j = 6*i;
		let k = 4*i;
		indices[j+0] = k+0;
		indices[j+1] = k+1;
		indices[j+2] = k+2;
		indices[j+3] = k+2;
		indices[j+4] = k+1;
		indices[j+5] = k+3;
	}

	const indexBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer);
	gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, indices, gl.STATIC_DRAW);
	
	return indexBuffer;
}

function update(deltaTime) {
	const controlX =
		((keyPressed.has('ArrowLeft') || keyPressed.has('Numpad4') || keyPressed.has('Numpad7') || keyPressed.has('Numpad1')) ? -1 : 0) +
		((keyPressed.has('ArrowRight') || keyPressed.has('Numpad6') || keyPressed.has('Numpad3') || keyPressed.has('Numpad9')) ? 1 : 0);
	const controlY =
		((keyPressed.has('ArrowUp') || keyPressed.has('Numpad8') || keyPressed.has('Numpad7') || keyPressed.has('Numpad9')) ? 1 : 0) +
		((keyPressed.has('ArrowDown') || keyPressed.has('Numpad2') || keyPressed.has('Numpad1') || keyPressed.has('Numpad3')) ? -1 : 0);

	const speed = 8.0;

	player.x += controlX * speed * deltaTime;
	player.y += controlY * speed * deltaTime;
}

function randomIntInRange(range) {
	return Math.floor(Math.random() * range);
}

function createWorldMap() {
	const worldMap = new Map();
	for (let i = 0; i < 100; ++i) {
		const x = randomIntInRange(worldSizeX);
		const y = randomIntInRange(worldSizeY);
		const key = [x, y];
		worldMap.set(key, 144);
	}
	return worldMap;
}

function drawScene(gl, glResources) {
	const screenX = gl.canvas.clientWidth;
	const screenY = gl.canvas.clientHeight;
	const sx = 32 / screenX;
	const sy = 32 / screenY;
	const tx = -16 * worldSizeX / screenX;
	const ty = -16 * worldSizeY / screenY;

	const projectionMatrix = mat4.fromValues(sx, 0, 0, 0, 0, sy, 0, 0, 0, 0, 1, 0, tx, ty, 0, 1);

	gl.clear(gl.COLOR_BUFFER_BIT);

	gl.uniformMatrix4fv(glResources.uniformLocations.projectionMatrix, false, projectionMatrix);

	for (let x = 0; x < worldSizeX; ++x) {
		for (let y = 0; y < worldSizeY; ++y) {
			addTile(gl, 132, x, y, 0, 0.68, 0, 1);
		}
	}
	worldMap.forEach((tileIndex, [x, y]) => addTile(gl, tileIndex, x, y, 0, 0.68, 0, 1));
	addTile(gl, 208, Math.floor(player.x * 16) / 16, Math.floor(player.y * 16) / 16, 0.66, 0.66, 0.66, 1);

	renderQuads(gl);
}

function initShaderProgram(gl, vsSource, fsSource) {
	const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
	const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

	const program = gl.createProgram();
	gl.attachShader(program, vertexShader);
	gl.attachShader(program, fragmentShader);
	gl.linkProgram(program);

	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(program));
		return null;
	}

	return program;
}

function loadShader(gl, type, source) {
	const shader = gl.createShader(type);

	gl.shaderSource(shader, source);
	gl.compileShader(shader);

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
		gl.deleteShader(shader);
		return null;
	}

	return shader;
}

function createTextureFromImage(gl, image) {
	const texture = gl.createTexture();
	gl.bindTexture(gl.TEXTURE_2D, texture);

	const level = 0;
	const internalFormat = gl.RGBA;
	const srcFormat = gl.RGBA;
	const srcType = gl.UNSIGNED_BYTE;
	gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
	gl.texImage2D(gl.TEXTURE_2D, level, internalFormat, srcFormat, srcType, image);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);

	return texture;
}

function renderQuads(gl) {
	if (numQuads > 0) {
		gl.bindBuffer(gl.ARRAY_BUFFER, glResources.buffers.position);
		gl.bufferData(gl.ARRAY_BUFFER, vertexPositions, gl.DYNAMIC_DRAW);

		gl.bindBuffer(gl.ARRAY_BUFFER, glResources.buffers.color);
		gl.bufferData(gl.ARRAY_BUFFER, vertexColors, gl.DYNAMIC_DRAW);

		gl.drawElements(gl.TRIANGLES, 6 * numQuads, gl.UNSIGNED_SHORT, 0);
	}
	numQuads = 0;
}

function addTile(gl, i, x, y, r, g, b, a) {
	const tileX = i % 16;
	const tileY = 15 - Math.floor(i / 16);

	const s0 = tileX / 16;
	const t0 = (tileY + 1) / 16;
	const s1 = (tileX + 1) / 16;
	const t1 = tileY / 16;

	addQuad(gl, x, y, x+1, y+1, s0, t0, s1, t1, r, g, b, a);
}

function addQuad(gl, x0, y0, x1, y1, s0, t0, s1, t1, r, g, b, a) {
	if (numQuads >= maxQuads) {
		renderQuads(gl);
	}

	// Append four vertices to the position/texcoord and color arrays

	const i = numQuads * 16;

	vertexPositions[i+0] = x0;
	vertexPositions[i+1] = y0;
	vertexPositions[i+2] = s0;
	vertexPositions[i+3] = t0;

	vertexPositions[i+4] = x1;
	vertexPositions[i+5] = y0;
	vertexPositions[i+6] = s1;
	vertexPositions[i+7] = t0;

	vertexPositions[i+8] = x0;
	vertexPositions[i+9] = y1;
	vertexPositions[i+10] = s0;
	vertexPositions[i+11] = t1;

	vertexPositions[i+12] = x1;
	vertexPositions[i+13] = y1;
	vertexPositions[i+14] = s1;
	vertexPositions[i+15] = t1;

	for (let j = 0; j < 16; j += 4) {
		vertexColors[i+j+0] = r;
		vertexColors[i+j+1] = g;
		vertexColors[i+j+2] = b;
		vertexColors[i+j+3] = a;
	}

	++numQuads;
}

window.onload = loadResourcesThenStart;